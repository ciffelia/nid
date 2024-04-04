//! This module defines the [`Alphabet`] trait and provides implementations for the most common alphabets used in Nano ID.
//!
//! An alphabet is a set of symbols that can be used in Nano ID. In this crate, only ASCII characters can be used as symbols.
//!
//! The default alphabet used in Nano ID is [`Base64UrlAlphabet`], which contains `A-Za-z0-9_-` symbols.
//!
//! # Implementing a custom alphabet
//!
//! To implement a custom alphabet, you need to create a new type that implements the [`Alphabet`] trait.
//!
//! # Examples
//!
//! ```rust
//! use nid::{alphabet::{Base36Alphabet, Base58Alphabet}, Nanoid};
//!
//! // Use the default Base64URL alphabet, which contains `A-Za-z0-9_-` symbols.
//! type ShopId = Nanoid<9>;
//! let id: ShopId = Nanoid::new();
//! let id: ShopId = "kP_IH1DPM".parse()?;
//!
//! // Use Base36 alphabet, which contains `A-Z0-9` symbols.
//! type UserId = Nanoid<21, Base36Alphabet>;
//! let id: UserId = Nanoid::new();
//! let id: UserId = "NDBIZRQSB6OGXJS06AN5L".parse()?;
//!
//! // Use Base58 alphabet, which contains `A-Za-z0-9` symbols excluding `0OlI`.
//! type ItemId = Nanoid<16, Base58Alphabet>;
//! let id: ItemId = Nanoid::new();
//! let id: ItemId = "96MrjhHuWXJMLCKh".parse()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// A set of symbols that can be used in Nano ID. In this crate, only ASCII characters can be used as symbols.
///
/// For the list of available alphabets, see the [`alphabet`](crate::alphabet) module.
///
/// # Implementing a custom alphabet
///
/// To implement a custom alphabet, you need to create a new type that implements the [`Alphabet`] trait.
///
/// ```rust
/// use nid::{alphabet::Alphabet, Nanoid};
///
/// struct CustomAlphabet;
///
/// impl Alphabet for CustomAlphabet {
///     const SYMBOL_LIST: &'static [u8] = b"(){}[]<>";
/// }
///
/// let id: Nanoid<21, CustomAlphabet> = Nanoid::new();
/// let id: Nanoid<21, CustomAlphabet> = "{{)((})>]<)}(>)(<)<){".parse()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Note that the alphabet must contain only ASCII characters. If you use an alphabet with non-ASCII characters, the compiler will raise an error.
///
/// ```compile_fail
/// use nid::{alphabet::Alphabet, Nanoid};
///
/// struct CustomAlphabet;
///
/// impl Alphabet for CustomAlphabet {
///     const SYMBOL_LIST: &'static [u8] = b"abc012\xa0\xa1";
/// }
///
/// let id: Nanoid<21, CustomAlphabet> = Nanoid::new(); // Compile error: found non-ascii symbol in alphabet
/// ```
pub trait Alphabet {
    /// The symbols that can be used in Nano ID.
    ///
    /// Symbols are represented as [`u8`] values. We may change this to [`std::ascii::Char`] when it becomes stable.
    const SYMBOL_LIST: &'static [u8];
}

/// An extension trait for [`Alphabet`] that provides additional constants.
pub(crate) trait AlphabetExt {
    /// The symbols that can be used in Nano ID.
    ///
    /// This is the same as [`Alphabet::SYMBOL_LIST`], but with the guarantee that all elements are ASCII characters.
    /// If [`Alphabet::SYMBOL_LIST`] contains non-ASCII characters, reading this constant will result in a compilation error.
    const VALID_SYMBOL_LIST: &'static [u8];

    /// A map that indicates whether a symbol is in the alphabet.
    ///
    /// If [`Alphabet::SYMBOL_LIST`] contains non-ASCII characters, reading this constant will result in a compilation error.
    const VALID_SYMBOL_MAP: [bool; 128];
}

impl<A: Alphabet> AlphabetExt for A {
    const VALID_SYMBOL_LIST: &'static [u8] = {
        assert_all_ascii(A::SYMBOL_LIST);
        A::SYMBOL_LIST
    };

    const VALID_SYMBOL_MAP: [bool; 128] = {
        let mut symbols_map = [false; 128];
        let mut i = 0;
        while i < A::VALID_SYMBOL_LIST.len() {
            symbols_map[A::VALID_SYMBOL_LIST[i] as usize] = true;
            i += 1;
        }
        symbols_map
    };
}

/// Assert that all elements are ASCII characters.
const fn assert_all_ascii(s: &[u8]) {
    let mut i = 0;
    while i < s.len() {
        assert!(s[i].is_ascii(), "found non-ascii symbol in alphabet");
        i += 1;
    }
}

macro_rules! define_and_impl_alphabet {
    ($name:ident, $symbols:expr, $description:expr $(,)?) => {
        #[doc = concat!(" ", $description, "

 # Example
 
 ```rust
 use nid::{alphabet::", stringify!($name), ", Nanoid};
 let id: Nanoid<21, ", stringify!($name), "> = Nanoid::new();
 ```")]
        #[derive(Debug)]
        pub struct $name;

        impl Alphabet for $name {
            const SYMBOL_LIST: &'static [u8] = $symbols;
        }
    };
}

define_and_impl_alphabet!(
    Base64UrlAlphabet,
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-",
    "Alphabet with `A-Za-z0-9_-` symbols. This is the default alphabet used in Nano ID.",
);

define_and_impl_alphabet!(
    Base62Alphabet,
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
    "Alphabet with `A-Za-z0-9` symbols.",
);

define_and_impl_alphabet!(
    Base58Alphabet,
    b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz123456789",
    "Alphabet with `A-Za-z0-9` symbols excluding `0OlI`.",
);

define_and_impl_alphabet!(
    Base36Alphabet,
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
    "Alphabet with `A-Z0-9` symbols.",
);

define_and_impl_alphabet!(
    Base36LowercaseAlphabet,
    b"abcdefghijklmnopqrstuvwxyz0123456789",
    "Alphabet with `a-z0-9` symbols.",
);

define_and_impl_alphabet!(
    Base32Alphabet,
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    "Alphabet with `A-Z2-7` symbols.",
);

define_and_impl_alphabet!(
    Base32LowercaseAlphabet,
    b"abcdefghijklmnopqrstuvwxyz234567",
    "Alphabet with `a-z2-7` symbols.",
);

define_and_impl_alphabet!(
    Base16Alphabet,
    b"ABCDEF0123456789",
    "Alphabet with `A-F0-9` symbols.",
);

define_and_impl_alphabet!(
    Base16LowercaseAlphabet,
    b"abcdef0123456789",
    "Alphabet with `a-f0-9` symbols.",
);

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_alphabet_len() {
        assert_eq!(Base64UrlAlphabet::SYMBOL_LIST.len(), 64);
        assert_eq!(Base62Alphabet::SYMBOL_LIST.len(), 62);
        assert_eq!(Base58Alphabet::SYMBOL_LIST.len(), 58);
        assert_eq!(Base36Alphabet::SYMBOL_LIST.len(), 36);
        assert_eq!(Base36LowercaseAlphabet::SYMBOL_LIST.len(), 36);
        assert_eq!(Base32Alphabet::SYMBOL_LIST.len(), 32);
        assert_eq!(Base32LowercaseAlphabet::SYMBOL_LIST.len(), 32);
        assert_eq!(Base16Alphabet::SYMBOL_LIST.len(), 16);
        assert_eq!(Base16LowercaseAlphabet::SYMBOL_LIST.len(), 16);
    }

    #[test]
    fn test_alphabet_symbol_map() {
        assert!(Base64UrlAlphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(!Base64UrlAlphabet::VALID_SYMBOL_MAP[b':' as usize]);
        assert!(Base62Alphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(!Base62Alphabet::VALID_SYMBOL_MAP[b'-' as usize]);
        assert!(Base58Alphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(!Base58Alphabet::VALID_SYMBOL_MAP[b'0' as usize]);
        assert!(Base36Alphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(!Base36Alphabet::VALID_SYMBOL_MAP[b'a' as usize]);
        assert!(Base36LowercaseAlphabet::VALID_SYMBOL_MAP[b'a' as usize]);
        assert!(!Base36LowercaseAlphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(Base32Alphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(!Base32Alphabet::VALID_SYMBOL_MAP[b'8' as usize]);
        assert!(Base32LowercaseAlphabet::VALID_SYMBOL_MAP[b'a' as usize]);
        assert!(!Base32LowercaseAlphabet::VALID_SYMBOL_MAP[b'8' as usize]);
        assert!(Base16Alphabet::VALID_SYMBOL_MAP[b'A' as usize]);
        assert!(!Base16Alphabet::VALID_SYMBOL_MAP[b'Z' as usize]);
        assert!(Base16LowercaseAlphabet::VALID_SYMBOL_MAP[b'a' as usize]);
        assert!(!Base16LowercaseAlphabet::VALID_SYMBOL_MAP[b'z' as usize]);
    }
}
