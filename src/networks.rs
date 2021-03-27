use regex::Regex;
use std::{
    net::IpAddr,
    str::FromStr
};

pub fn is_ipv4(value: &str) -> bool {
    //! Check to see if a given value corresponds to IPv4 Address.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_ipv4;
    //!
    //! fn main() {
    //!     assert!(is_ipv4("128.10.133.22"));
    //! }
    //! ```
    let ip = if let Ok(ipaddr) = IpAddr::from_str(value){
        ipaddr
    } else {
        return false
    };
    ip.is_ipv4()
}

pub fn is_ipv4_cidr(value: &str) -> bool {
    //! Check to see if a given IPv4 Address with CIDR is valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_ipv4_cidr;
    //!
    //! fn main() {
    //!     assert!(is_ipv4_cidr("10.0.0.0/24"));
    //! }
    //! ```
    let splitted_groups: Vec<&str> = value.splitn(2, '/').collect();
    let prefix = splitted_groups[0];
    let suffix = splitted_groups[1];

    let nsuffix: u32 = match suffix.parse() {
        Ok(x) => x,
        Err(_) => {
            return false;
        }
    };

    if nsuffix > 32 {
        return false;
    }

    if !is_ipv4(prefix) {
        return false;
    }

    true
}

pub fn is_ipv6(value: &str) -> bool {
    //! Check to see if a given value corresponds to IPv6 Address.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_ipv6;
    //!
    //! fn main() {
    //!     assert!(is_ipv6("::ffff:127.0.0.1"));
    //! }
    //! ```
    let ip = if let Ok(ipaddr) = IpAddr::from_str(value){
        ipaddr
    } else {
        return false
    };
    ip.is_ipv6()
}

pub fn is_ipv6_cidr(value: &str) -> bool {
    //! Check to see if a given IPv6 Address with CIDR is valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_ipv6_cidr;
    //!
    //! fn main() {
    //!     assert!(is_ipv6_cidr("2001:0DB8:1234::/48"));
    //! }
    //! ```
    let splitted_groups: Vec<&str> = value.splitn(2, '/').collect();
    let prefix = splitted_groups[0];
    let suffix = splitted_groups[1];

    let nsuffix: u32 = match suffix.parse() {
        Ok(x) => x,
        Err(_) => {
            return false;
        }
    };

    if nsuffix > 128 {
        return false;
    }

    if !is_ipv6(prefix) {
        return false;
    }

    true
}

pub fn is_ip_loopback(value: &str) -> bool {
    //! Check to see if a given value corresponds to Local/loopback IP Address.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_ip_loopback;
    //!
    //! fn main() {
    //!     assert!(is_ip_loopback("::1"));
    //! }
    //! ```
    let ip = if let Ok(ipaddr) = IpAddr::from_str(value){
        ipaddr
    } else {
        return false
    };
    ip.is_loopback()
}

pub fn is_ipv_any(value: &str) -> bool {
    //! Check to see if a given value corresponds to IP Address.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_ipv_any;
    //!
    //! fn main() {
    //!     assert!(is_ipv_any("::ffff:127.0.0.1"));
    //! }
    //! ```
    if is_ipv4(value) {
        return true;
    } else if is_ipv6(value) {
        return true;
    }
    false
}

pub fn which_ipv(value: &str) -> Option<&str> {
    //! Check to see if a given value corresponds to IP Address & return its IP version.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::which_ipv;
    //!
    //! fn main() {
    //!     assert_eq!(which_ipv("::1"), Some("IPv6"));
    //! }
    //! ```
    if is_ipv4(value) {
        return Some("IPv4")
    } else if is_ipv6(value) {
        return Some("IPv6")
    }
    None
}

pub fn is_mac_address(value: &str) -> bool {
    //! Check to see if a given value corresponds to MAC Address.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::networks::is_mac_address;
    //!
    //! fn main() {
    //!     assert!(is_mac_address("F6-7C-9E-36-C9-E3"));
    //! }
    //! ```
    let pattern = match Regex::new(r#"^(?:[0-9a-fA-F]{2}[:.-]){5}[0-9a-fA-F]{2}$"#) {
        Ok(x) => x,
        Err(_) => {
            return false;
        }
    };
    if pattern.is_match(value) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ipv4() {
        assert!(is_ipv4("10.10.10.1"));
        assert!(is_ipv4("100.128.10.132"));
        assert!(!is_ipv4("12.110.105.256"));
        assert!(!is_ipv4("10.2.13"));
        assert!(!is_ipv4("256.10.10.1000"));
        assert!(is_ipv4("100.17.5.119"));
        assert!(is_ipv4("127.0.0.1"));
    }

    #[test]
    fn test_is_ipv4_cidr() {
        assert!(is_ipv4_cidr("10.0.0.0/8"));
        assert!(is_ipv4_cidr("10.0.0.0/32"));
        assert!(!is_ipv4_cidr("10.0.0.0/33"));
        assert!(!is_ipv4_cidr("270.0.0.1000/24"));
    }

    #[test]
    fn test_is_ipv6() {
        assert!(is_ipv6("2041:0000:140F::875B:131B"));
        assert!(is_ipv6("::ffff:127.0.0.1"));
        assert!(is_ipv6("::ffff:7f00:1"));
        assert!(is_ipv6("::1"));
        assert!(is_ipv6("2041:0:140F::875B:131B"));
        assert!(is_ipv6("2041:0000:140F::875B:131B"));
        assert!(is_ipv6("fcb7:360a:242a:2d0d:392e:bc22:a45:3573"));
        assert!(!is_ipv6("2002:::1234::"));
        assert!(is_ipv6("3b8f:473b:d1a7:ba09:d28c:3cd:7f46:c95e"));
        assert!(is_ipv6("0000:0000:0000:0000:0000:FFFF:2BE0:9E74"));
        assert!(is_ipv6("::ffff:43.224.158.116"));
    }

    #[test]
    fn test_is_ipv6_cidr() {
        assert!(is_ipv6_cidr("2001:0DB8:1234::/48"));
        assert!(is_ipv6_cidr("2001:0DB8:12a4::/128"));
        assert!(!is_ipv6_cidr("2005:0DB8:1234::/130"));
        assert!(!is_ipv6_cidr("2002:::1234::/48"));
    }

    #[test]
    fn test_is_ip_loopback() {
        assert!(is_ip_loopback("127.0.0.1"));
        assert!(is_ip_loopback("::1"));
        assert!(!is_ip_loopback("10.122.1.130"));
        assert!(!is_ip_loopback("::ffff:7f00:1"));
    }

    #[test]
    fn test_is_ipv_any() {
        assert!(is_ipv_any("127.0.0.1"));
        assert!(is_ipv_any("::1"));
    }

    #[test]
    fn test_which_ipv() {
        assert_eq!(which_ipv("::1"), Some("IPv6"));
        assert_eq!(which_ipv("127.0.0.1"), Some("IPv4"));
        assert_eq!(which_ipv("2002:::1234::"), None);
    }

    #[test]
    fn test_is_mac_address() {
        assert!(is_mac_address("F6-7C-9E-36-C9-E3"));
        assert!(is_mac_address("F6:7C:9E:36:C9:E3"));
        assert!(is_mac_address("F6.7C.9E.36.C9.E3"));
    }
}