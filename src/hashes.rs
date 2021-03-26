use regex::Regex;

lazy_static! {
    static ref MD5: Regex = Regex::new(r"^(?i)[0-9a-f]{32}$").unwrap();
    static ref SHA1: Regex = Regex::new(r"^(?i)[0-9a-f]{40}$").unwrap();
    static ref SHA224: Regex = Regex::new(r"^(?i)[0-9a-f]{56}$").unwrap();
    static ref SHA256: Regex = Regex::new(r"^(?i)[0-9a-f]{64}$").unwrap();
    static ref SHA384: Regex = Regex::new(r"^(?i)[0-9a-f]{96}$").unwrap();
    static ref SHA512: Regex = Regex::new(r"^(?i)[0-9a-f]{128}$").unwrap();
}

enum Type {
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}

impl Type {
    fn name<'a>(&self) -> &'a str {
        match *self {
            Type::MD5 => "MD5",
            Type::SHA1 => "SHA1",
            Type::SHA224 => "SHA224",
            Type::SHA256 => "SHA256",
            Type::SHA384 => "SHA384",
            Type::SHA512 => "SHA512",
        }
    }

    fn pattern<'a>(&self) -> &'a Regex {
        match *self {
            Type::MD5 => &MD5,
            Type::SHA1 => &SHA1,
            Type::SHA224 => &SHA224,
            Type::SHA256 => &SHA256,
            Type::SHA384 => &SHA384,
            Type::SHA512 => &SHA512,
        }
    }

    fn all() -> Vec<Type> {
        vec![
            Type::MD5,
            Type::SHA1,
            Type::SHA224,
            Type::SHA256,
            Type::SHA384,
            Type::SHA512,
        ]
    }
}

/// Evaluate Hash Type
fn validate(value: &str) -> bool {
    for hash in Type::all() {
        if hash.pattern().is_match(&value) {
            return true
        }
    }
    false
}

pub fn is_md5(value: &str) -> bool {
    //! Check if a given hash value is MD5.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_md5;
    //!
    //! fn main() {
    //!     assert_eq!(is_md5("<md5 value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn is_sha1(value: &str) -> bool {
    //! Check if a given hash value is SHA1.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_sha1;
    //!
    //! fn main() {
    //!     assert_eq!(is_sha1("<sha1 value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn is_sha224(value: &str) -> bool {
    //! Check if a given hash value is SHA224.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_sha224;
    //!
    //! fn main() {
    //!     assert_eq!(is_sha224("<sha224 value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn is_sha256(value: &str) -> bool {
    //! Check if a given hash value is SHA256.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_sha256;
    //!
    //! fn main() {
    //!     assert_eq!(is_sha256("<sha256 value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn is_sha384(value: &str) -> bool {
    //! Check if a given hash value is SHA384.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_sha384;
    //!
    //! fn main() {
    //!     assert_eq!(is_sha384("<sha384 value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn is_sha512(value: &str) -> bool {
    //! Check if a given hash value is SHA512.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_sha512;
    //!
    //! fn main() {
    //!     assert_eq!(is_sha512("<sha512 value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn is_hash_any(value: &str) -> bool {
    //! Check if a given value corresponds to a Hash Type.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::is_hash_any;
    //!
    //! fn main() {
    //!     assert_eq!(is_hash_any("<hash value>"), false)
    //! }
    //! ```
    validate(value)
}

pub fn which_hash(value: &str) -> Option<&str> {
    //! Check if a given value corresponds to a Hash Type
    //! and outputs the appropriate Hash Name.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::hashes::which_hash;
    //!
    //! fn main() {
    //!     assert_eq!(which_hash("<hash value>"), None)
    //! }
    //! ```
    for hash in Type::all() {
        if hash.pattern().is_match(&value) {
            return Some(hash.name())
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_which_hash() {
        // md5
        assert_eq!(which_hash("5eb63bbbe01eeed093cb22bb8f5acdc3"), Some("MD5"));
        // sha1
        assert_eq!(which_hash("2AAE6C35C94FCFB415DBE95F408B9CE91EE846ED"), Some("SHA1"));
        // sha224
        assert_eq!(which_hash("2f05477fc24bb4faefd86517156dafdecec45b8ad3cf2522a563582b"), Some("SHA224"));
        // sha256
        assert_eq!(which_hash("B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"), Some("SHA256"));
        // sha384
        assert_eq!(which_hash("fdbd8e75a67f29f701a4e040385e2e23986303ea10239211af907fcbb83578b3e417cb71ce646efd0819dd8c088de1bd"), Some("SHA384"));
        // sha512
        assert_eq!(which_hash("309ECC489C12D6EB4CC40F50C902F2B4D0ED77EE511A7C7A9BCD3CA86D4CD86F989DD35BC5FF499670DA34255B45B0CFD830E81F605DCF7DC5542E93AE9CD76F"), Some("SHA512"));
    }

    #[test]
    fn test_is_hash_any() {
        assert_eq!(is_hash_any("3f21"), false);
        assert_eq!(is_hash_any("5eb63bbbe01eeed093cb22bb8f5acdc3"), true);
    }

    #[test]
    fn test_is_md5() {
        assert_eq!(is_md5("5eb63bbbe01eeed093cb22bb8f5acdc3"), true);
    }

    #[test]
    fn test_is_sha1() {
        assert_eq!(is_sha1("2AAE6C35C94FCFB415DBE95F408B9CE91EE846ED"), true);
    }

    #[test]
    fn test_is_sha224() {
        assert_eq!(is_sha224("2f05477fc24bb4faefd86517156dafdecec45b8ad3cf2522a563582b"), true);
    }

    #[test]
    fn test_is_sha256() {
        assert_eq!(is_sha256("B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"), true);
    }

    #[test]
    fn test_is_sha384() {
        assert_eq!(is_sha384("fdbd8e75a67f29f701a4e040385e2e23986303ea10239211af907fcbb83578b3e417cb71ce646efd0819dd8c088de1bd"), true);
    }

    #[test]
    fn test_is_sha512() {
        assert_eq!(is_sha512("309ECC489C12D6EB4CC40F50C902F2B4D0ED77EE511A7C7A9BCD3CA86D4CD86F989DD35BC5FF499670DA34255B45B0CFD830E81F605DCF7DC5542E93AE9CD76F"), true);
    }
}