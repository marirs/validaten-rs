use regex::Regex;

lazy_static! {
    static ref MD5: Regex = Regex::new(r"^(?i)[0-9a-f]{32}$").unwrap();
    static ref SHA1: Regex = Regex::new(r"^(?i)[0-9a-f]{40}$").unwrap();
    static ref SHA224: Regex = Regex::new(r"^(?i)[0-9a-f]{56}$").unwrap();
    static ref SHA256: Regex = Regex::new(r"^(?i)[0-9a-f]{64}$").unwrap();
    static ref SHA384: Regex = Regex::new(r"^(?i)[0-9a-f]{96}$").unwrap();
    static ref SHA512: Regex = Regex::new(r"^(?i)[0-9a-f]{128}$").unwrap();
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
    if MD5.is_match(value) {
        true
    } else {
        false
    }
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
    if SHA1.is_match(value) {
        true
    } else {
        false
    }
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
    if SHA224.is_match(value) {
        true
    } else {
        false
    }
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
    if SHA256.is_match(value) {
        true
    } else {
        false
    }
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
    if SHA384.is_match(value) {
        true
    } else {
        false
    }
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
    if SHA512.is_match(value) {
        true
    } else {
        false
    }
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
    if is_md5(value) {
        return true;
    } else if is_sha1(value) {
        return true;
    } else if is_sha224(value) {
        return true;
    } else if is_sha256(value) {
        return true;
    } else if is_sha384(value) {
        return true;
    } else if is_sha512(value) {
        return true;
    }

    false
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
    if is_md5(value) {
        return Some("MD5");
    } else if is_sha1(value) {
        return Some("SHA1");
    } else if is_sha224(value) {
        return Some("SHA224");
    } else if is_sha256(value) {
        return Some("SHA256");
    } else if is_sha384(value) {
        return Some("SHA384");
    } else if is_sha512(value) {
        return Some("SHA512");
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