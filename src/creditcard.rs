use regex::Regex;
use std::ops::Range;
use checkluhn;

lazy_static! {
    // Debit Cards
    static ref VISAELECTRON: Regex = Regex::new(r"^4(026|17500|405|508|844|91[37])").unwrap();
    static ref MAESTRO: Regex = Regex::new(r"^(5(018|0[23]|[68])|6(39|7))").unwrap();
    static ref FORBRUGSFORENINGEN: Regex = Regex::new(r"^600").unwrap();
    static ref DANKORT: Regex = Regex::new(r"^5019").unwrap();

    // Credit Cards
    static ref VISA: Regex = Regex::new(r"^4").unwrap();
    static ref MASTERCARD: Regex = Regex::new(r"^(5[1-5]|2[2-7])").unwrap();
    static ref AMEX: Regex = Regex::new(r"^3[47]").unwrap();
    static ref DINERSCLUB: Regex = Regex::new(r"^3[0689]").unwrap();
    static ref DISCOVER: Regex = Regex::new(r"^6([045]|22)").unwrap();
    static ref UNIONPAY: Regex = Regex::new(r"^(62|88)").unwrap();
    static ref JCB: Regex = Regex::new(r"^35").unwrap();
}

#[derive(Clone, PartialEq)]
enum Type {
    VisaElectron,
    Maestro,
    Forbrugsforeningen,
    Dankort,
    Visa,
    MasterCard,
    Amex,
    DinersClub,
    Discover,
    UnionPay,
    JCB,
}

impl Type {
    fn name<'a>(&self) -> &'a str {
        match *self {
            Type::VisaElectron => "Visa Electron",
            Type::Maestro => "Maestro",
            Type::Forbrugsforeningen => "Forbrugsforeningen",
            Type::Dankort => "Dankort",
            Type::Visa => "Visa",
            Type::MasterCard => "MasterCard",
            Type::Amex => "Amex",
            Type::DinersClub => "Diners Club",
            Type::Discover => "Discover",
            Type::UnionPay => "UnionPay",
            Type::JCB => "JCB",
        }
    }

    fn pattern<'a>(&self) -> &'a Regex {
        match *self {
            Type::VisaElectron => &VISAELECTRON,
            Type::Maestro => &MAESTRO,
            Type::Forbrugsforeningen => &FORBRUGSFORENINGEN,
            Type::Dankort => &DANKORT,
            Type::Visa => &VISA,
            Type::MasterCard => &MASTERCARD,
            Type::Amex => &AMEX,
            Type::DinersClub => &DINERSCLUB,
            Type::Discover => &DISCOVER,
            Type::UnionPay => &UNIONPAY,
            Type::JCB => &JCB,
        }
    }

    fn length<'a>(&self) -> Range<usize> {
        match *self {
            Type::VisaElectron => Range { start: 16, end: 16 },
            Type::Maestro => Range { start: 12, end: 19 },
            Type::Forbrugsforeningen => Range { start: 16, end: 16 },
            Type::Dankort => Range { start: 16, end: 16 },
            Type::Visa => Range { start: 13, end: 16 },
            Type::MasterCard => Range { start: 16, end: 16 },
            Type::Amex => Range { start: 15, end: 15 },
            Type::DinersClub => Range { start: 13, end: 16 },
            Type::Discover => Range { start: 16, end: 16 },
            Type::JCB => Range { start: 16, end: 16 },
            Type::UnionPay => Range { start: 16, end: 19 },
        }
    }

    fn all() -> Vec<Type> {
        vec![
            // Debit Cards
            Type::VisaElectron,
            Type::Maestro,
            Type::Forbrugsforeningen,
            Type::Dankort,

            // Credit Cards
            Type::Visa,
            Type::MasterCard,
            Type::Amex,
            Type::DinersClub,
            Type::Discover,
            Type::UnionPay,
            Type::JCB,
        ]
    }
}

/// Check if the given card number and card type has a valid length
fn is_length_valid(card_number: &str, card_type: &Type) -> bool {
    let size = card_number.len();
    let range = card_type.length();

    size >= range.start && size <= range.end
}

/// Check if card number passes luhn test
fn is_luhn_valid(card_number: &str) -> bool {
    checkluhn::validate(&card_number)
}

/// Evaluate Card Type & Validate Card for its Brand
fn validate(value: &str) -> bool {
    // if card number (value) contains spaces in between, remove them
    let value = value.replace(" ", "");
    for card in Type::all() {
        if card.pattern().is_match(&value) {
            // Check Length
            if is_length_valid(&value, &card) {
                // Check if it passes luhn algorithm
                if is_luhn_valid(&value) {
                    return true
                }
                // luhn check failed
                return false
            }
            // card type length check failed
            return false
        }
    }
    false
}

pub fn is_valid_visa_electron(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is Visa Electron
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_visa_electron;
    //! fn main() {
    //!     assert_eq!(is_valid_visa_electron("4844 1614 5954 6174"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_maestro(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is Maestro
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_maestro;
    //! fn main() {
    //!     assert_eq!(is_valid_maestro("5898009041197"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_forbrugsforeningen(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is Forbrugsforeningen
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_forbrugsforeningen;
    //! fn main() {
    //! assert_eq!(is_valid_forbrugsforeningen("6007221111111110"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_dankort(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is Dankort
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_dankort;
    //! fn main() {
    //! assert_eq!(is_valid_dankort("5019118545073189"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_visa(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is VISA
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_visa;
    //! fn main() {
    //! assert_eq!(is_valid_visa("4035 3005 3980 4083"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_mastercard(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is MasterCard
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_mastercard;
    //! fn main() {
    //! assert_eq!(is_valid_mastercard("5463 1135 8998 2388"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_amex(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is American Express
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_amex;
    //! fn main() {
    //! assert_eq!(is_valid_amex("3707 897090 84107"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_diners_club(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is Dinners Club
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_diners_club;
    //! fn main() {
    //! assert_eq!(is_valid_diners_club("3022143741431999"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_discover(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is Discover
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_discover;
    //! fn main() {
    //! assert_eq!(is_valid_discover("6011575126600688"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_unionpay(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is UnionPay
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_unionpay;
    //! fn main() {
    //! assert_eq!(is_valid_unionpay("62600094752489242"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_valid_jcb(value: &str) -> bool {
    //! Check if the Given Credit/Debit card number is JCB
    //! and that if its valid.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::is_valid_jcb;
    //! fn main() {
    //! assert_eq!(is_valid_jcb("3588337499926343"), true)
    //! }
    //! ```
    validate(value)
}

pub fn is_card_any(value: &str) -> bool {
    //! Check if a given value is a credit/debit card number.
    //!
    //! ## Example usage
    //! ```rust
    //! use validaten::creditcard::is_card_any;
    //! fn main() {
    //!     assert_eq!(is_card_any("3588337499926343"), true);
    //! }
    //! ```
    validate(value)
}

pub fn which_card(value: &str) -> Option<&str> {
    //! Determines which Credit/Debit Card is provided.
    //!
    //! ## Example Usage
    //! ```rust
    //! use validaten::creditcard::which_card;
    //!
    //! fn main() {
    //!     println!("{:?}", which_card("3707 897090 84107"));
    //! }
    //! ```
    // remove the spaces if the card number contains
    let value = value.replace(" ", "");
    for card in Type::all() {
        if card.pattern().is_match(&value) {
            return Some(card.name())
        }
    }
    None
}

#[cfg(test)]
mod tests {
    // Card numbers are generated from: https://debitcard-generator.com/validator
    use super::*;

    #[test]
    fn test_which_card() {
        // Visa Electron
        assert_eq!(which_card("4844161459546175"), Some("Visa Electron"));
        // Maestro
        assert_eq!(which_card("5898009041193"), Some("Maestro"));
        // Forbrugsforeningen
        assert_eq!(which_card("6007221111111110"), Some("Forbrugsforeningen"));
        // Dankort
        assert_eq!(which_card("5019118545073184"), Some("Dankort"));
        // VISA
        assert_eq!(which_card("4035300539804083"), Some("Visa"));
        // MasterCard
        assert_eq!(which_card("5463113589982388"), Some("MasterCard"));
        // Amex
        assert_eq!(which_card("370789709084107"), Some("Amex"));
        // Dinners Club
        assert_eq!(which_card("3022143741431999"), Some("Diners Club"));
        // Discover
        assert_eq!(which_card("6011575126600688"), Some("Discover"));
        // China UnionPay
        assert_eq!(which_card("62600094752489245"), Some("UnionPay"));
        // JCB
        assert_eq!(which_card("3588337499926343"), Some("JCB"));
    }

    #[test]
    fn test_is_card_any() {
        assert!(is_card_any("3588337499926343"));
        assert!(is_card_any("5019118545073189"));
        assert!(!is_card_any("5019112"));
    }

    #[test]
    fn test_is_valid_visa_electron() {
        assert!(is_valid_visa_electron("4844 1614 5954 6174"));
        assert!(!is_valid_visa_electron("4844161459546175"));
    }

    #[test]
    fn test_is_valid_maestro() {
        assert!(is_valid_maestro("6796265520244"));
        assert!(!is_valid_maestro("5898009041193"));
    }

    #[test]
    fn test_is_valid_forbrugsforeningen() {
        assert!(is_valid_forbrugsforeningen("6007221111111110"));
        assert!(!is_valid_forbrugsforeningen("6007221111111111"));
    }

    #[test]
    fn test_is_valid_dankort() {
        assert!(is_valid_dankort("5019118545073189"));
        assert!(!is_valid_dankort("5019118545073184"));
    }

    #[test]
    fn test_is_valid_visa() {
        assert!(is_valid_visa("4035300539804083"));
        assert!(!is_valid_visa("4035300539804082"));
    }

    #[test]
    fn test_is_valid_mastercard() {
        assert!(is_valid_mastercard("5463113589982388"));
        assert!(!is_valid_mastercard("5463113589982387"));
    }

    #[test]
    fn test_is_valid_amex() {
        assert!(is_valid_amex("370789709084107"));
        assert!(!is_valid_amex("370789709084106"));
    }

    #[test]
    fn test_is_valid_diners_club() {
        assert!(is_valid_diners_club("3022143741431999"));
        assert!(!is_valid_diners_club("3022143741431990"));
        assert!(is_valid_diners_club("3860847190349"));
        assert!(is_valid_diners_club("30043277253245"));
    }

    #[test]
    fn test_is_valid_discover() {
        assert!(is_valid_discover("6011575126600688"));
        assert!(!is_valid_discover("6011575126600689"))
    }

    #[test]
    fn test_is_valid_unionpay() {
        assert!(is_valid_unionpay("62600094752489242"));
        assert!(!is_valid_unionpay("62600094752489245"));
    }

    #[test]
    fn test_is_valid_jcb() {
        assert!(is_valid_jcb("3588337499926343"));
        assert!(!is_valid_jcb("3588337499926345"))
    }
}