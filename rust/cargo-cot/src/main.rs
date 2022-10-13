use ansi_term::{Color, Style};
use clap::{Parser, Subcommand};
use eos_rs_build::target_region::TargetRegion;
use serde_json::Value;
use std::env::current_dir;
use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::iter::once;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs, process};
use which::which;

const ABOUT: &str = "
Cargo extension to build c-of-time projects and burn/write them to a ROM.";

#[derive(Parser, Debug)]
#[clap(
    name = "cargo-cot",
    bin_name = "cargo",
    version,
    disable_help_subcommand = true,
    propagate_version = true
)]
#[allow(dead_code)]
enum Opt {
    #[clap(
    name = "cot",
    version,
    about = ABOUT,
    override_usage = "cargo cot [COMMAND] [OPTIONS] [<args>]",
    disable_version_flag = true
    )]
    Cot {
        #[clap(subcommand)]
        command: Commands,
    },
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Builds the project, including the C codebase.
    /// This is equivalent to `cargo build -Zbuild-std=core,alloc --target ./armv5te-none-ndseoseabi-XX.json`,
    /// where XX is the region specified.
    Build {
        /// The region to build for; `eu`, `na` or `jp`.
        ///
        /// If not specified the region will be taken from `workspace.metadata.cot.region` in the
        /// Cargo.toml (if it is specified).
        region: Option<String>,

        /// Build artifacts in release mode, with optimization.
        #[clap(short, long)]
        release: bool,

        /// Any additional argument after '--' will be forwarded to cargo build.
        #[clap(last = true, value_parser)]
        cargo_args: Vec<OsString>,
    },

    /// Build the project and write the project to a ROM.
    /// Overlay 36 is patched and patches in ../patches are applied to the game (including the glue
    /// code from the `patches!` macro).
    Burn {
        /// The region to build for; `eu`, `na` or `jp`.
        ///
        /// If not specified the region will be taken from `workspace.metadata.cot.region` in the
        /// Cargo.toml (if it is specified).
        region: Option<String>,

        /// Path to the input ROM to patch.
        rom_path: PathBuf,

        /// Path where the patched ROM should be saved.
        out_path: PathBuf,

        /// Build artifacts in release mode, with optimization.
        #[clap(short, long)]
        release: bool,

        /// Any additional argument after '--' will be forwarded to cargo build.
        #[clap(last = true, value_parser)]
        cargo_args: Vec<OsString>,
    },
}

fn main() -> ! {
    let Opt::Cot { command } = Opt::parse();

    let burn_rom_path;
    let burn_rom_out_path;
    let build_region_str;
    let build_release;
    let build_cargo_args;

    match command {
        Commands::Build {
            region,
            release,
            cargo_args,
        } => {
            build_region_str = region;
            build_release = release;
            build_cargo_args = cargo_args;
            burn_rom_path = None;
            burn_rom_out_path = None;
        }
        Commands::Burn {
            region,
            release,
            cargo_args,
            rom_path,
            out_path,
        } => {
            build_region_str = region;
            build_release = release;
            build_cargo_args = cargo_args;
            burn_rom_path = Some(fs::canonicalize(rom_path).expect("The ROM path does not exist."));
            burn_rom_out_path =
                Some(fs::canonicalize(out_path).expect("The out path does not exist."));
        }
    }

    let manifest_dir = get_manifest_dir(&build_cargo_args);
    assert!(
        manifest_dir.exists(),
        "The manifest directory must exist: {:?}",
        manifest_dir
    );

    let build_region_str = match build_region_str {
        None => {
            // Try to read the build region from the Cargo.toml
            match cargo_metadata_region(manifest_dir.as_path()) {
                None => {
                    eprintln!("{}", Color::Red.paint("Error: A region must be specified."));
                    process::exit(1)
                }
                Some(build_region_str) => build_region_str,
            }
        }
        Some(build_region_str) => build_region_str,
    };

    match TargetRegion::from_str(build_region_str) {
        Ok(build_region) => {
            cargo_build(
                manifest_dir.as_path(),
                &build_region,
                build_release,
                build_cargo_args,
            );
            if let Some(rom_path) = burn_rom_path {
                burn(
                    manifest_dir.as_path(),
                    &build_region,
                    rom_path,
                    burn_rom_out_path.unwrap(),
                    build_release,
                );
            }
            process::exit(0)
        }
        Err(err) => {
            eprintln!("{}", Color::Red.paint(format!("Error: {}", err)));
            process::exit(1)
        }
    }
}

fn cargo_metadata_region(manifest_dir: &Path) -> Option<String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut child = Command::new(cargo)
        .args([
            "metadata",
            "--no-deps",
            "--manifest-path",
            manifest_dir.join("Cargo.toml").to_string_lossy().as_ref(),
            "--format-version",
            "1",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let exit = child.wait().unwrap();
    if !exit.success() {
        process::exit(exit.code().unwrap_or(1));
    }
    let mut output = vec![];
    child
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut output)
        .unwrap();
    let output_parsed: Value = serde_json::from_str(&String::from_utf8(output).unwrap()).unwrap();

    if let Value::Object(package) = output_parsed {
        if let Some(Value::Object(metadata)) = package.get("metadata") {
            if let Some(Value::Object(cot)) = metadata.get("cot") {
                if let Some(Value::String(region)) = cot.get("region") {
                    return Some(region.clone());
                }
            }
        }
    }
    None
}

fn cargo_build(
    manifest_dir: &Path,
    build_region: &TargetRegion,
    build_release: bool,
    build_cargo_args: Vec<OsString>,
) {
    let target_fname = build_region.target_str();
    let target_file = manifest_dir.join(&format!("{}.json", target_fname));
    if !target_file.exists() {
        eprintln!(
            "{}",
            Color::Red.paint(format!(
                "Error: The target file '{}.json' was not found in the manifest directory.",
                target_fname
            ))
        );
        process::exit(1)
    };
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut args_iter: Box<dyn Iterator<Item = OsString>> = Box::new(
        [
            "build".into(),
            "-Zbuild-std=core,alloc".into(),
            "--target".into(),
            target_file.into_os_string(),
        ]
        .into_iter()
        .chain(build_cargo_args.into_iter()),
    );
    if build_release {
        args_iter = Box::new(args_iter.chain(once(OsString::from("--release"))));
    }
    let exit = Command::new(cargo)
        .args(args_iter)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    if !exit.success() {
        process::exit(exit.code().unwrap_or(1));
    }
}

fn burn(
    manifest_dir: &Path,
    build_region: &TargetRegion,
    rom_path: PathBuf,
    rom_out_path: PathBuf,
    build_release: bool,
) {
    let cot_base_path = manifest_dir.parent().unwrap();

    let out_dir_profile = if build_release { "release" } else { "debug" };
    let elf_path = manifest_dir.join(format!(
        "target/{}/{}/eos-rs-bin.elf",
        build_region.target_str(),
        out_dir_profile
    ));
    let bin_path = manifest_dir.join(format!(
        "target/{}/{}/eos-rs-bin.bin",
        build_region.target_str(),
        out_dir_profile
    ));

    print_info("Starting burning...");

    let objcopy = which("arm-none-eabi-objcopy").unwrap_or_else(|_| {
        print_error(
            "Was unable to find 'arm-none-eabi-objcopy' command. Is DevkitPro correctly set up?",
        );
        process::exit(1);
    });

    if !build_release {
        print_warning("You are burning a version with debugging information, for the final hack, you should use the --release flag.");
    }

    let python = get_python_interpreter(cot_base_path);

    print_task("Extracting & stripping binary...");
    burn_run(
        objcopy,
        &[
            "--strip-all",
            "-O",
            "binary",
            elf_path.to_str().unwrap(),
            bin_path.to_str().unwrap(),
        ],
        manifest_dir,
    );

    print_task("Running patcher...");
    burn_run(
        python,
        &[
            "scripts/patch.py",
            build_region.as_str_upper(),
            rom_path.to_str().unwrap(),
            bin_path.to_str().unwrap(),
            elf_path.to_str().unwrap(),
            rom_out_path.to_str().unwrap(),
        ],
        cot_base_path,
    );

    print_success(format!(
        "Output ROM written to: {}",
        rom_out_path.to_string_lossy()
    ))
}

fn get_manifest_dir(cargo_args: &[OsString]) -> PathBuf {
    let mut path = current_dir().unwrap();
    let mut cargo_args_iter = cargo_args.iter();
    if let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        path = PathBuf::from(cargo_manifest_dir)
    }
    while let Some(arg) = cargo_args_iter.next() {
        if arg == "--manifest-path" {
            if let Some(manifest_path) = cargo_args_iter.next() {
                path = PathBuf::from(manifest_path).parent().unwrap().to_path_buf();
            }
            break;
        }
    }
    fs::canonicalize(current_dir().unwrap().join(path)).unwrap()
}

fn get_python_interpreter(base_dir: &Path) -> PathBuf {
    #[cfg(windows)]
    let interpreter_path = base_dir.join("venv/bin/python.exe");
    #[cfg(not(windows))]
    let interpreter_path = base_dir.join("venv/bin/python");

    if !interpreter_path.exists() {
        print_task("Creating Python virtualenv...");
        let base_python = which("python3").unwrap_or_else(|_| {
            print_error("Was unable to find Python 3. Is it installed?");
            process::exit(1);
        });
        burn_run(
            &base_python,
            &["-m", "venv", base_dir.join("venv").to_str().unwrap()],
            base_dir,
        );
        if !interpreter_path.exists() {
            print_error("Was unable to find the Python interpreter after creating the venv.");
            process::exit(1);
        }
        burn_run(
            &interpreter_path,
            &["-m", "pip", "install", "ndspy", "keystone-engine", "pyyaml"],
            base_dir,
        );
    }

    print_note(format!(
        "Using Python interpreter at: {}",
        interpreter_path.to_string_lossy()
    ));
    interpreter_path
}

fn burn_run<S: AsRef<OsStr>>(cmd: S, args: &[&str], dir: &Path) {
    let arg_list = args.to_vec().join(" ");
    burn_print(
        "$",
        format!("{} {}", cmd.as_ref().to_string_lossy(), arg_list),
        Color::Purple,
        false,
        false,
    );
    assert!(
        dir.exists(),
        "The working directory for the command ({:?}) does not exist.",
        dir
    );
    let exit = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap_or_else(|e| {
            print_error(format!("Failed to spawn command: {:?}", e));
            process::exit(1)
        });
    if !exit.success() {
        print_error("Command failed!");
        process::exit(exit.code().unwrap_or(1));
    }
}

#[inline(always)]
fn print_info<S: AsRef<str>>(msg: S) {
    burn_print("ℹ", msg, Color::Cyan, true, true);
}

#[inline(always)]
fn print_note<S: AsRef<str>>(msg: S) {
    burn_print("ℹ", msg, Color::Purple, true, false);
}

#[inline(always)]
fn print_task<S: AsRef<str>>(msg: S) {
    burn_print("⚒", msg, Color::Green, true, false);
}

#[inline(always)]
fn print_error<S: AsRef<str>>(msg: S) {
    burn_print("❌", msg, Color::Red, true, true);
}

#[inline(always)]
fn print_warning<S: AsRef<str>>(msg: S) {
    burn_print("⚠", msg, Color::Yellow, true, false);
}

#[inline(always)]
fn print_success<S: AsRef<str>>(msg: S) {
    burn_print("✅", msg, Color::Green, true, true);
}

fn burn_print<S: AsRef<str>>(icon: &str, msg: S, color_icon: Color, color_msg: bool, bold: bool) {
    let mut style_icon = Style::new().fg(color_icon);
    let mut style_msg = Style::new();

    style_icon.is_bold = bold;
    style_msg.is_bold = bold;

    if color_msg {
        style_msg.foreground = Some(color_icon);
    }

    println!(
        "{} {}",
        style_icon.paint(icon),
        style_msg.paint(msg.as_ref())
    );
}
