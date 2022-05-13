//! Nintendo DS system & game bootstrapping related functions.

use crate::ffi;

/// Probably related to booting the game?
///
/// This function prints the debug message "task proc boot".
///
/// # Safety
/// This function is related to game-boot and will probably modify a lot of global data.
pub unsafe fn task_proc_boot() {
    ffi::TaskProcBoot();
}

/// Presumably blocks until the program receives an interrupt.
///
/// This just calls (in Ghidra terminology) coproc_moveto_Wait_for_interrupt(0).
/// See <https://en.wikipedia.org/wiki/ARM_architecture_family#Coprocessors>.
pub fn wait_for_interrupt() {
    unsafe { ffi::WaitForInterrupt(); }
}

/// Sets the Interrupt Master Enable (IME) register to 1, which enables all CPU interrupts
/// (if enabled in the Interrupt Enable (IE) register).
///
/// See <https://problemkaputt.de/gbatek.htm#dsiomaps>.
///
/// Returns the old value in the IME register.
///
/// # Safety
/// The caller needs to make sure the system is in a state where it is safe to enable interrupts.
pub unsafe fn enable_all_interrupts() -> bool {
    ffi::EnableAllInterrupts() > 0
}

/// Sets the Interrupt Master Enable (IME) register to 0, which disables all CPU interrupts
/// (even if enabled in the Interrupt Enable (IE) register).
///
/// See <https://problemkaputt.de/gbatek.htm#dsiomaps>.
///
/// Returns the old value in the IME register.
///
/// # Safety
/// The caller needs to make sure the system is in a state where it is safe to disable interrupts.
pub unsafe fn disable_all_interrupts() -> bool {
    ffi::DisableAllInterrupts() > 0
}

/// Get the current (system?) time as an IEEE 754 floating-point number.
///
/// Returns the current time (in seconds?).
pub fn get_time() -> f32 {
    unsafe { ffi::GetTime() }
}

/// Probably resumes the sound player if paused?
///
/// # Safety
/// This function manipulates low-level global state.
pub unsafe fn sound_resume() -> bool {
    ffi::SoundResume() > 0
}

/// Probably aborts the program with some status code? It seems to serve a similar purpose to the
/// exit(3) function.
///
/// This function prints the debug string "card pull out %d" with the status code.
///
/// # Safety
/// This function manipulates low-level global state.
pub unsafe fn card_pull_out_with_status(status: i32) {
    ffi::CardPullOutWithStatus(status);
}

/// Sets some global flag that probably triggers system exit?
///
/// This function prints the debug string "card pull out".
///
/// # Safety
/// This function manipulates low-level global state.
pub unsafe fn card_pull_out() {
    ffi::CardPullOut();
}

/// Sets some global flag that maybe indicates a save error?
///
/// This function prints the debug string "card backup error".
///
/// # Safety
/// This function manipulates low-level global state.
pub unsafe fn card_backup_error() {
    ffi::CardBackupError();
}

/// Maybe halts the process display?
///
/// This function prints the debug string "halt process disp %d" with the status code.
///
/// # Safety
/// This function manipulates low-level global state.
pub unsafe fn halt_process_disp(status_code: i32) {
    ffi::HaltProcessDisp(status_code);
}

/// Supposed to return a debug flag. Just returns 0 in the final binary.
pub fn get_debug_flag1(flag_id: u32) -> u32 {
    // SAFETY: This is more or less an "atomic" operation.
    unsafe { ffi::GetDebugFlag1(flag_id) }
}

/// Supposed to return a debug flag. Just returns 0 in the final binary.
pub fn get_debug_flag2(flag_id: u32) -> u32 {
    // SAFETY: This is more or less an "atomic" operation.
    unsafe { ffi::GetDebugFlag2(flag_id) }
}

/// Supposed to set a debug flag. No-op in the final binary.
pub fn set_debug_flag1(flag_id: u32, value: u32) {
    // SAFETY: This is more or less an "atomic" operation.
    unsafe { ffi::SetDebugFlag1(flag_id, value) }
}

/// Supposed to return a debug flag. No-op in the final binary.
pub fn set_debug_flag2(flag_id: u32, value: u32) {
    // SAFETY: This is more or less an "atomic" operation.
    unsafe { ffi::SetDebugFlag2(flag_id, value) }
}
