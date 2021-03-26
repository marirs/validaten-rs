#[cfg(any(feature = "crypto"))]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "crypto")]
pub mod crypto;