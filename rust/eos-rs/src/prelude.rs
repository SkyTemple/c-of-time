//! Helpful prelude that will pull in a lot of useful types for you,
//! and set up the allocator and panic handler.
//! Logging macros of the [`log`] crate are also included.

#[cfg(not(test))]
pub use crate::allocation::ALLOCATOR;
#[cfg(not(test))]
#[allow(unused_imports)]  // I'm not sure if without this import the panic_handler will be registered.
pub use crate::panic::*;
#[doc(hidden)]  // So it's not documented twice.
pub use crate::patches;
pub use log::{debug, error, info, trace, warn};
#[cfg(feature = "io")]
pub use crate::api::io::prelude::*;
