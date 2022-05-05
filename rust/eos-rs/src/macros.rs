/// This forces a normal reference (&) to be converted to a mutable pointer (*mut).
/// This is extremely unsafe, the caller promises that this is OK to do in this context.
///
/// This macro mostly exists because we make some optimistic assumptions about the ffi
/// functions actually only requiring const pointers in some instances, even if the C
/// headers don't declare them as const.
macro_rules! force_mut_ptr {
    ($x:expr) => {
        $x as *const _ as *mut _
    };
}
