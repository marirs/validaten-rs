#[cfg(any(
feature = "crypto",
feature = "hashes"
))]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "hashes")]
pub mod hashes;
