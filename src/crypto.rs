use regex::Regex;

lazy_static! {
    /// Bitcoin Regex Pattern
    static ref BTC: Regex = Regex::new(r"(?i)^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$").unwrap();
    /// Bitcoin Cash Regex Pattern
    static ref BCH: Regex = Regex::new(r"(?i)^((bitcoincash|bchreg|bchtest):)?(q|p)[a-z0-9]{41}$").unwrap();
    /// Ethereum Regex Pattern
    static ref ETH: Regex = Regex::new(r"(?i)^0x[a-fA-F0-9]{40}$").unwrap();
    /// Litecoin Regex Pattern
    static ref LTC: Regex = Regex::new(r"(?i)^[LM3][a-km-zA-HJ-NP-Z1-9]{26,33}$").unwrap();
    /// Dodge Coin Regex Pattern
    static ref DODGE: Regex = Regex::new(r"(?i)^D{1}[5-9A-HJ-NP-U]{1}[1-9A-HJ-NP-Za-km-z]{32}$").unwrap();
    /// Dash Regex Pattern
    static ref DASH: Regex = Regex::new(r"^(?i)X[1-9A-HJ-NP-Za-km-z]{33}$").unwrap();
    /// Monero Regex Pattern
    static ref XMR: Regex = Regex::new(r"(?i)^4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$").unwrap();
    /// Neo Regex Pattern
    static ref NEO: Regex = Regex::new(r"(?i)^A[0-9a-zA-Z]{33}$").unwrap();
    /// Ripple Regex Pattern
    static ref XRP: Regex = Regex::new(r"(?i)^r|X[0-9a-zA-Z]{33,47}$").unwrap();
}

pub fn is_bitcoin(value: &str) -> bool {
    //! Check if the given crypto address is Bitcoin.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_bitcoin;
    //! fn main() {
    //!     assert_eq!(is_bitcoin("<bitcoin address>"), false);
    //! }
    //! ```
    if BTC.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_bitcoin_cash(value: &str) -> bool {
    //! Check if the given crypto address is Bitcoin Cash.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_bitcoin_cash;
    //! fn main() {
    //!     assert_eq!(is_bitcoin_cash("<bitcoin cash address>"), false);
    //! }
    //! ```
    if BCH.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_ethereum(value: &str) -> bool {
    //! Check if the given crypto address is Ethereum.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_ethereum;
    //! fn main() {
    //!     assert_eq!(is_ethereum("<ethereum address>"), false);
    //! }
    //! ```
    if ETH.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_litecoin(value: &str) -> bool {
    //! Check if the given crypto address is Litecoin.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_litecoin;
    //! fn main() {
    //!     assert_eq!(is_litecoin("<litecoin address>"), false);
    //! }
    //! ```
    if LTC.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_dogecoin(value: &str) -> bool {
    //! Check if the given crypto address is Dodgecoin.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_dogecoin;
    //! fn main() {
    //!     assert_eq!(is_dogecoin("<dodgecoin address>"), false);
    //! }
    //! ```
    if DODGE.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_dash(value: &str) -> bool {
    //! Check if the given crypto address is Dash.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_dash;
    //! fn main() {
    //!     assert_eq!(is_dash("<dash address>"), false);
    //! }
    //! ```
    if DASH.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_monero(value: &str) -> bool {
    //! Check if the given crypto address is Monero.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_monero;
    //! fn main() {
    //!     assert_eq!(is_monero("<monero address>"), false);
    //! }
    //! ```
    if XMR.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_neo(value: &str) -> bool {
    //! Check if the given crypto address is Neo.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_neo;
    //! fn main() {
    //!     assert_eq!(is_neo("<neo address>"), false);
    //! }
    //! ```
    if NEO.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_ripple(value: &str) -> bool {
    //! Check if the given crypto address is Ripple.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_ripple;
    //! fn main() {
    //!     assert_eq!(is_ripple("<ripple address>"), false);
    //! }
    //! ```
    if XRP.is_match(value) {
        true
    } else {
        false
    }
}

pub fn is_cryptocurrency_any(value: &str) -> bool {
    //! Check if the given string is a Crypto Currency.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::is_cryptocurrency_any;
    //! fn main() {
    //!     assert_eq!(is_cryptocurrency_any("<cryptocurrency address>"), false);
    //! }
    //! ```
    if is_bitcoin(value) {
        return true;
    } else if is_bitcoin_cash(value) {
        return true;
    } else if is_ethereum(value) {
        return true;
    } else if is_litecoin(value) {
        return true;
    } else if is_dogecoin(value) {
        return true;
    } else if is_dash(value) {
        return true;
    } else if is_monero(value) {
        return true;
    } else if is_neo(value) {
        return true;
    } else if is_ripple(value) {
        return true;
    }

    false
}

pub fn which_cryptocurrency(value: &str) -> Option<&str> {
    //! Output the Crypto Currency Name given the cryptocurrencty address.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::crypto::which_cryptocurrency;
    //! fn main() {
    //!     assert_eq!(which_cryptocurrency("<cryptocurrency address>"), None);
    //! }
    //! ```
    if is_bitcoin(value) {
        return Some("Bitcoin");
    } else if is_bitcoin_cash(value) {
        return Some("Bitcoin Cash");
    } else if is_ethereum(value) {
        return Some("Ethereum");
    } else if is_litecoin(value) {
        return Some("Litecoin");
    } else if is_dogecoin(value) {
        return Some("Dodgecoin");
    } else if is_dash(value) {
        return Some("Dash");
    } else if is_monero(value) {
        return Some("Monero");
    } else if is_neo(value) {
        return Some("Neo");
    } else if is_ripple(value) {
        return Some("Ripple");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_which_cryptocurrency() {
        // Bitcoin
        assert_eq!(which_cryptocurrency("1GiWxH6PzSSmbdcK72XfGpqhjSb6nae6h9"), Some("Bitcoin"));
        // Bitcoin Cash
        assert_eq!(which_cryptocurrency("qppjlghjlwg6tgxv7ffhvs43rlul0kpp4c0shk4dr6"), Some("Bitcoin Cash"));
        // Ethereum
        assert_eq!(which_cryptocurrency("0xaae47eae4ddd4877e0ae0bc780cfaee3cc3b52cb"), Some("Ethereum"));
        // Litecoin
        assert_eq!(which_cryptocurrency("LQ4i7FLNhfCC9GXw682mS1NzvVKbtJAFZq"), Some("Litecoin"));
        // Dodgecoin
        assert_eq!(which_cryptocurrency("D6K2nqqQKycTucCSFSHhpiig4yQ6NPQRf9"), Some("Dodgecoin"));
        // Dash
        assert_eq!(which_cryptocurrency("XqLYPDTADW6EYuQmTcEAx81o8EHTKwqTK8"), Some("Dash"));
        // Monero
        assert_eq!(which_cryptocurrency("41gYNjXMeXaTmZFVv645A1HRVoA637cXFGbDdLV8Gn5hLvfxfRLKigUTvm2HVZhBzDVPeGpDy71qxASTpRFgepDwLexA8Ti"), Some("Monero"));
        // Neo
        assert_eq!(which_cryptocurrency("AeHauBkGkHPTxh4PEUhNr7WRgivmcdCRnR"), Some("Neo"));
        // Ripple
        assert_eq!(which_cryptocurrency("rUocf1ixKzTuEe34kmVhRvGqNCofY1NJzV"), Some("Ripple"));
        // No coin identified
        assert_eq!(which_cryptocurrency("LQ4i7FLNbtJAFZq"), None);
    }

    #[test]
    fn test_is_cryptocurrency_any() {
        assert_eq!(is_cryptocurrency_any("D6K2nqqQKycTucCSFSHhpiig4yQ6NPQRf9"), true);
        assert_eq!(is_cryptocurrency_any("LQ4i7FLNbtJAFZq"), false);
    }

    #[test]
    fn test_is_bitcoin() {
        assert_eq!(is_bitcoin("1GiWxH6PzSSmbdcK72XfGpqhjSb6nae6h9"), true)
    }

    #[test]
    fn test_is_bitcoin_cash() {
        assert_eq!(is_bitcoin_cash("qppjlghjlwg6tgxv7ffhvs43rlul0kpp4c0shk4dr6"), true)
    }

    #[test]
    fn test_is_ethereum() {
        assert_eq!(is_ethereum("0xaae47eae4ddd4877e0ae0bc780cfaee3cc3b52cb"), true)
    }

    #[test]
    fn test_is_litecoin() {
        assert_eq!(is_litecoin("LQ4i7FLNhfCC9GXw682mS1NzvVKbtJAFZq"), true)
    }

    #[test]
    fn test_is_dogecoin() {
        assert_eq!(is_dogecoin("D6K2nqqQKycTucCSFSHhpiig4yQ6NPQRf9"), true)
    }

    #[test]
    fn test_is_dash() {
        assert_eq!(is_dash("XqLYPDTADW6EYuQmTcEAx81o8EHTKwqTK8"), true)
    }

    #[test]
    fn test_is_monero() {
        assert_eq!(is_monero("41gYNjXMeXaTmZFVv645A1HRVoA637cXFGbDdLV8Gn5hLvfxfRLKigUTvm2HVZhBzDVPeGpDy71qxASTpRFgepDwLexA8Ti"), true)
    }

    #[test]
    fn test_is_neo() {
        assert_eq!(is_neo("AeHauBkGkHPTxh4PEUhNr7WRgivmcdCRnR"), true)
    }

    #[test]
    fn test_is_ripple() {
        assert_eq!(is_ripple("rUocf1ixKzTuEe34kmVhRvGqNCofY1NJzV"), true)
    }
}