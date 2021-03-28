#[cfg(any(
feature = "crypto",
feature = "hashes",
feature = "creditcard",
feature = "internet"
))]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "hashes")]
pub mod hashes;

#[cfg(feature = "creditcard")]
pub mod creditcard;

#[cfg(feature = "networks")]
pub mod networks;

#[cfg(feature = "internet")]
pub mod internet;
