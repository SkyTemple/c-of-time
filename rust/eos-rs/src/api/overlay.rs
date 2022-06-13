//! Module for dealing with the load-state of Nintendo DS overlays.

use crate::ffi;

/// This represents a promise to the compiler that while a reference
/// to this struct exists, the overlay N is loaded.
///
/// This is used as a parameter in functions that need a given overlay to be loaded.
///
/// Dropping the lease does *not* unload an overlay, but you can use
/// the [`Self::unload`] method on a lease to do so.
///
/// You can clone a lease, but you will have the responsibility to make
/// sure the overlay is not unloaded while clone lease is in use too.
#[derive(Clone)]
pub struct OverlayLoadLease<const N: u32>(());

impl<const N: u32> OverlayLoadLease<N> {
    /// Gets a lease on a loaded overlay.
    ///
    /// This is a runtime checked version of `acquire_unchecked`.
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
        unsafe { ffi::OverlayIsLoaded(Self::group_id()) > 0 }
    }

    /// Loads the given overlay if it isn't already loaded.
    ///
    /// # Safety
    /// This will change how memory is mapped. You will need to make
    /// sure there are no conflicts with other overlays and that all
    /// memory currently referenced will still be valid after this call.
    pub unsafe fn load() -> Self {
        if !Self::is_loaded() {
            ffi::LoadOverlay(Self::group_id())
        }
        Self(())
    }

    /// Unloads the overlay.
    ///
    /// # Safety
    /// This will render all still existing leases on the overlay invalid.
    /// You need to make sure that all leases are dropped.
    pub unsafe fn unload(self) {
        ffi::UnloadOverlay(Self::group_id());
    }

    /// Returns the group ID of the overlay. This supports all game builtin overlays.
    /// Panics if no mapping is known.
    pub fn group_id() -> u32 {
        match N {
            0 => ffi::overlay_group_id::OGROUP_OVERLAY_0,
            1 => ffi::overlay_group_id::OGROUP_OVERLAY_1,
            2 => ffi::overlay_group_id::OGROUP_OVERLAY_2,
            3 => ffi::overlay_group_id::OGROUP_OVERLAY_3,
            4 => ffi::overlay_group_id::OGROUP_OVERLAY_4,
            5 => ffi::overlay_group_id::OGROUP_OVERLAY_5,
            6 => ffi::overlay_group_id::OGROUP_OVERLAY_6,
            7 => ffi::overlay_group_id::OGROUP_OVERLAY_7,
            8 => ffi::overlay_group_id::OGROUP_OVERLAY_8,
            9 => ffi::overlay_group_id::OGROUP_OVERLAY_9,
            10 => ffi::overlay_group_id::OGROUP_OVERLAY_10,
            11 => ffi::overlay_group_id::OGROUP_OVERLAY_11,
            12 => ffi::overlay_group_id::OGROUP_OVERLAY_12,
            13 => ffi::overlay_group_id::OGROUP_OVERLAY_13,
            14 => ffi::overlay_group_id::OGROUP_OVERLAY_14,
            15 => ffi::overlay_group_id::OGROUP_OVERLAY_15,
            16 => ffi::overlay_group_id::OGROUP_OVERLAY_16,
            17 => ffi::overlay_group_id::OGROUP_OVERLAY_17,
            18 => ffi::overlay_group_id::OGROUP_OVERLAY_18,
            19 => ffi::overlay_group_id::OGROUP_OVERLAY_19,
            20 => ffi::overlay_group_id::OGROUP_OVERLAY_20,
            21 => ffi::overlay_group_id::OGROUP_OVERLAY_21,
            22 => ffi::overlay_group_id::OGROUP_OVERLAY_22,
            23 => ffi::overlay_group_id::OGROUP_OVERLAY_23,
            24 => ffi::overlay_group_id::OGROUP_OVERLAY_24,
            25 => ffi::overlay_group_id::OGROUP_OVERLAY_25,
            26 => ffi::overlay_group_id::OGROUP_OVERLAY_26,
            27 => ffi::overlay_group_id::OGROUP_OVERLAY_27,
            28 => ffi::overlay_group_id::OGROUP_OVERLAY_28,
            29 => ffi::overlay_group_id::OGROUP_OVERLAY_29,
            30 => ffi::overlay_group_id::OGROUP_OVERLAY_30,
            31 => ffi::overlay_group_id::OGROUP_OVERLAY_31,
            32 => ffi::overlay_group_id::OGROUP_OVERLAY_32,
            33 => ffi::overlay_group_id::OGROUP_OVERLAY_33,
            34 => ffi::overlay_group_id::OGROUP_OVERLAY_34,
            35 => ffi::overlay_group_id::OGROUP_OVERLAY_35,
            _ => panic!("No known overlay group for overlay."),
        }
    }
}

/// Trait for all structs that require a lease to be created (but nothing else).
pub trait CreatableWithLease<const N: u32>
where
    Self: Sized,
{
    /// Internal constructor that needs to be implemented. Don't call this directly.
    #[doc(hidden)]
    fn _create(lease: OverlayLoadLease<N>) -> Self;

    /// Create a new instance by providing a lease.
    ///
    /// # Important
    /// Overlay N must not be unloaded during the lifetime of the returned object.
    fn new(lease: OverlayLoadLease<N>) -> Self {
        Self::_create(lease)
    }

    /// Create a new instance by checking if the overlay
    /// is loaded and acquiring a lease on it.
    /// This will panic if the overlay is not loaded.
    ///
    /// # Important
    /// Overlay N must not be unloaded during the lifetime of the returned object.
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
