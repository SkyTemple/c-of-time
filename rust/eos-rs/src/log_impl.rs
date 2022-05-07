//! [`log`] crate logging implementation for EoS.

use alloc::ffi::CString;
use alloc::format;
use crate::ctypes::c_char;
use log::{Level, LevelFilter, Metadata, Record};
use crate::ffi;

static LOGGER: EoSLogger = EoSLogger;
static mut LOGGER_INIT: bool = false;

/// Registers the logger at the [`log`] crate. This is safe to be called multiple times.
pub fn register_logger() {
    // We will ignore errors during logger setup.
    // SAFETY: We only have one thread, we are sure we are the only ones calling this.
    unsafe {
        if !LOGGER_INIT {
            log::set_logger_racy(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug)).ok();
            LOGGER_INIT = true;
        }
    }
}

struct EoSLogger;

impl log::Log for EoSLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let c_str = CString::new(format!(
                "[rs] {} : {} -- {} [{}:{}]",
                record.level(),
                record.target(),
                record.args(),
                record.module_path().unwrap_or("?"),
                record.line().unwrap_or_default()
            )).unwrap();
            let c_ptr = c_str.as_ptr() as *const c_char;
            unsafe {
                match record.level() {
                    Level::Error => ffi::DebugPrint(2, c_ptr),
                    Level::Warn => ffi::DebugPrint(1, c_ptr),
                    _ => ffi::DebugPrint(0, c_ptr),
                }
            }
        }
    }

    fn flush(&self) {}
}
