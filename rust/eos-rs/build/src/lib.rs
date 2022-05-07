pub mod target_region;

use std::{env, fs};
use std::fs::{read_to_string, remove_file};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use glob::glob;
use syn::{ItemMacro, parse2, parse_file};
use syn::visit::Visit;
use crate::target_region::TargetRegion;
use eos_rs_patches_def::PatchesDef;
use which::which;

struct SourceVisitor<'a> {
    cotpatch: &'a mut String
}

impl<'a> SourceVisitor<'a> {
    fn new(cotpatch: &'a mut String) -> Self {
        Self { cotpatch }
    }
}

impl<'ast> Visit<'ast> for SourceVisitor<'ast> {
    fn visit_item_macro(&mut self, i: &'ast ItemMacro) {
        // TODO: This won't work for paths.
        let name = i.mac.path.get_ident();
        if name.is_none() {
            return;
        }
        // TODO: This doesn't actually make sure this is *our* patches macro.
        if name.unwrap().to_string().as_str() == "patches" {
            // Process a patches macro.
            let input = i.mac.tokens.clone().into();
            let def = parse2::<PatchesDef>(input).unwrap();
            if let Some(glue) = def.glue {
                self.cotpatch.push_str(&glue);
            }
        }
    }
}

// This compiles the C code in ../src.
pub fn compile_c_code(makefile: &Path) {
    let makefile_dir = makefile.parent().unwrap();
    let make_cmd = Command::new("make")
        .args(["-f", makefile.to_str().unwrap(), "buildobjs", "EXTRA_CFLAGS=-D COT_RUST"])
        .current_dir(makefile_dir)
        .status()
        .expect("Failed to run make.");
    assert!(make_cmd.success(), "Failed to run make.");

    // Remove the built .bin and .elf files if they exit to not confuse anybody.
    remove_file(makefile_dir.join("out.elf")).ok();
    remove_file(makefile_dir.join("out.bin")).ok();

    // This will create .o files in the build directory.
    for obj in glob(&format!("{}/*.o", makefile_dir.join("build").to_str().unwrap())).unwrap().flatten() {
        println!("cargo:rustc-link-arg={}", obj.to_str().unwrap());
    }

}

// This generates the pmdsky-debug symbols for the linker.
// Python 3 must be in the PATH.
pub fn generate_symbols_for_linker(cot_rot: &Path) {
    let target_region = TargetRegion::from_target_env()
        .expect("Failed to determine game region. Make sure your target name ends with -na, -ja or -eu.");

    static ERR: &str = "Failed to run command to generate symbols for linker. Is 'python3' in your PATH?";

    let python = which("python3").unwrap_or_else(|_| {
        panic!("Was unable to find Python 3. Is it installed?");
    });

    let make_cmd = Command::new(python)
        .args(["scripts/generate_linkerscript.py", target_region.as_str_upper()])
        .current_dir(cot_rot)
        .status()
        .expect(ERR);

    assert!(make_cmd.success(), "{}", ERR);
}


/// This collects the glue code from the !patches macro and dumps it into a .cotpatch file
pub fn generate_cotpatch(out_file: &Path) {
    // TODO: This only works if the patches block is in the main.rs.
    let fname = PathBuf::from_str(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .unwrap()
        .join("src/main.rs");

    let content = read_to_string(&fname).expect(&format!("Unable to read Rust source file: {:?}", &fname));
    let syntax = parse_file(&content).expect(&format!("Unable to parse Rust source file: {:?}", &fname));

    let mut cotpatch = String::new();

    let mut visitor = SourceVisitor::new(&mut cotpatch);
    visitor.visit_file(&syntax);

    fs::write(out_file, cotpatch)
        .expect(&format!("Unable to write to file: {:?}", out_file));
}
