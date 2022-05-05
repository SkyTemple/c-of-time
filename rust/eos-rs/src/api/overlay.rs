use crate::ffi;

/// This represents a promise to the compiler that while a reference
/// to this struct exists, the overlay N is loaded. This is used as
/// a parameter in functions that need a given overlay to be loaded.
/// Dropping the lease does *not* unload an overlay, but you can use
/// the `unload` method on a lease to do so.
/// You can clone a lease, but you will have the responsibility to make
/// sure the overlay is not unloaded while clone lease is in use too.
#[derive(Clone)]
pub struct OverlayLoadLease<const N: u32>(());

impl<const N: u32> OverlayLoadLease<N> {
    /// Gets a lease on a loaded overlay.
    ///
    /// This is a checked version of acquire_unchecked.
    /// It will panic if the overlay is not loaded.
    ///
    /// # Important
    /// Even though this function is marked as safe, since it
    /// checks if the overlay is loaded, you still need to make
    /// sure the overlay is not unloaded during the lifetime
    /// of the returned object.
    pub fn acquire() -> Self {
        if !Self::is_loaded() {
            panic!("Overlay {} is not loaded.", N);
        }
        Self(())
    }
    /// Gets a lease on a loaded overlay.
    ///
    /// # Safety
    /// You need to make sure the overlay is actually loaded and won't
    /// get unloaded during the lifetime of the returned object.
    pub unsafe fn acquire_unchecked() -> Self {
        Self(())
    }

    /// Checks if the overlay is loaded.
    pub fn is_loaded() -> bool {
        unsafe { ffi::OverlayIsLoaded(N) > 0 }
    }

    /// Loads the given overlay if it isn't already loaded.
    ///
    /// # Safety
    /// This will change how memory is mapped. You will need to make
    /// sure there are no conflicts with other overlays and that all
    /// memory currently referenced will still be valid after this call.
    pub unsafe fn load() -> Self {
        if !Self::is_loaded() {
            ffi::LoadOverlay(N)
        }
        Self(())
    }

    /// Unloads the overlay.
    ///
    /// # Safety
    /// This will render all still existing leases on the overlay invalid.
    /// You need to make sure that all leases are dropped.
    pub unsafe fn unload(self) {
        ffi::UnloadOverlay(N);
    }
}

/// Trait for all structs that require a lease to be created (but nothing else).
pub trait CreatableWithLease<const N: u32> where Self: Sized {
    /// Internal constructor that needs to be implemented. Don't call this directly.
    fn _create(lease: OverlayLoadLease<N>) -> Self;

    /// Create a new instance by providing a lease.
    ///
    /// # Important
    /// Overlay 29 must not be unloaded during the lifetime of the returned object.
    fn new(lease: OverlayLoadLease<N>) -> Self {
        Self::_create(lease)
    }

    /// Create a new instance by checking if the overlay
    /// is loaded and acquiring a lease on it.
    /// This will panic if the overlay is not loaded.
    ///
    /// # Important
    /// Overlay 29 must not be unloaded during the lifetime of the returned object.
    fn new_checked() -> Self {
        Self::_create(OverlayLoadLease::<N>::acquire())
    }

    /// Create a new lease.
    ///
    /// # Safety
    /// This function is unsafe because it does not check if the overlay is loaded.
    /// You need to make sure that the overlay is loaded before calling this function,
    /// and additionally ensure that during the lifetime of this object the overlay
    /// is not unloaded.
    unsafe fn new_unchecked() -> Self {
        Self::_create(OverlayLoadLease::<N>::acquire_unchecked())
    }

    /// Returns a reference to the overlay lease.
    fn lease(&self) -> &OverlayLoadLease<N>;
}
