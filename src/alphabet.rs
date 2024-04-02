//! This module defines the [`Alphabet`] trait and provides implementations for the most common alphabets used in Nano ID.
//!
//! An alphabet is a set of symbols that can be used in Nano ID. In this crate, only ASCII characters can be used as symbols.
//!
//! The default alphabet used in Nano ID is [`Base64UrlAlphabet`], which contains `A-Za-z0-9_-` symbols.
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
/// NOTE: All symbols are represented as [`u8`] values. We may change this to [`std::ascii::Char`] when it becomes stable.
pub trait Alphabet {
    /// Returns the number of symbols in the alphabet.
    fn len() -> usize;

    /// Returns the symbol at the specified index.
    fn get(index: usize) -> u8;

    /// Returns `true` if the alphabet contains the specified symbol.
    fn contains(symbol: u8) -> bool;
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

        impl $name {
            const INNER: AlphabetInner<'static, { $symbols.len() }> = AlphabetInner::new($symbols);
        }

        impl Alphabet for $name {
            fn len() -> usize {
                Self::INNER.len()
            }

            fn get(index: usize) -> u8 {
                Self::INNER.get(index)
            }

            fn contains(symbol: u8) -> bool {
                Self::INNER.contains(symbol)
            }
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

struct AlphabetInner<'a, const L: usize> {
    symbols: &'a [u8; L],
    symbols_map: [bool; 128],
}

impl<'a, const L: usize> AlphabetInner<'a, L> {
    /// Create a new [`AlphabetInner`] from a byte slice that contains symbols.
    ///
    /// # Panics
    ///
    /// The function will panic if the byte slice contains non-ascii bytes.
    #[must_use]
    pub const fn new(symbols: &'a [u8; L]) -> Self {
        assert_validity(symbols);

        let mut symbols_map = [false; 128];
        let mut i = 0;
        while i < L {
            symbols_map[symbols[i] as usize] = true;
            i += 1;
        }

        Self {
            symbols,
            symbols_map,
        }
    }

    /// Returns the number of symbols in the alphabet.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns the symbol at the specified index.
    pub const fn get(&self, index: usize) -> u8 {
        self.symbols[index]
    }

    /// Returns `true` if the alphabet contains the specified symbol.
    pub const fn contains(&self, c: u8) -> bool {
        self.symbols_map[c as usize]
    }
}

/// Assert that all symbols in the alphabet are valid ASCII characters.
const fn assert_validity<const L: usize>(alphabet: &[u8; L]) {
    let mut i = 0;
    while i < L {
        assert!(alphabet[i].is_ascii(), "found non-ascii symbol in alphabet");
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_alphabet_len() {
        assert_eq!(Base64UrlAlphabet::len(), 64);
        assert_eq!(Base62Alphabet::len(), 62);
        assert_eq!(Base58Alphabet::len(), 58);
        assert_eq!(Base36Alphabet::len(), 36);
        assert_eq!(Base36LowercaseAlphabet::len(), 36);
        assert_eq!(Base32Alphabet::len(), 32);
        assert_eq!(Base32LowercaseAlphabet::len(), 32);
        assert_eq!(Base16Alphabet::len(), 16);
        assert_eq!(Base16LowercaseAlphabet::len(), 16);
    }

    #[test]
    fn test_alphabet_get() {
        assert_eq!(Base64UrlAlphabet::get(0), b'A');
        assert_eq!(Base62Alphabet::get(0), b'A');
        assert_eq!(Base62Alphabet::get(61), b'9');
        assert_eq!(Base58Alphabet::get(0), b'A');
        assert_eq!(Base58Alphabet::get(57), b'9');
        assert_eq!(Base36Alphabet::get(0), b'A');
        assert_eq!(Base36Alphabet::get(35), b'9');
        assert_eq!(Base36LowercaseAlphabet::get(0), b'a');
        assert_eq!(Base36LowercaseAlphabet::get(35), b'9');
        assert_eq!(Base32Alphabet::get(0), b'A');
        assert_eq!(Base32Alphabet::get(31), b'7');
        assert_eq!(Base32LowercaseAlphabet::get(0), b'a');
        assert_eq!(Base32LowercaseAlphabet::get(31), b'7');
        assert_eq!(Base16Alphabet::get(0), b'A');
        assert_eq!(Base16Alphabet::get(15), b'9');
        assert_eq!(Base16LowercaseAlphabet::get(0), b'a');
        assert_eq!(Base16LowercaseAlphabet::get(15), b'9');
    }

    #[test]
    fn test_alphabet_get_each() {
        fn inner<A: Alphabet>() {
            for i in 0..A::len() {
                A::get(i);
            }
        }

        inner::<Base64UrlAlphabet>();
        inner::<Base62Alphabet>();
        inner::<Base58Alphabet>();
        inner::<Base36Alphabet>();
        inner::<Base36LowercaseAlphabet>();
        inner::<Base32Alphabet>();
        inner::<Base32LowercaseAlphabet>();
        inner::<Base16Alphabet>();
        inner::<Base16LowercaseAlphabet>();
    }

    #[test]
    fn test_alphabet_contains() {
        assert!(Base64UrlAlphabet::contains(b'A'));
        assert!(!Base64UrlAlphabet::contains(b':'));
        assert!(Base62Alphabet::contains(b'A'));
        assert!(!Base62Alphabet::contains(b'-'));
        assert!(Base58Alphabet::contains(b'A'));
        assert!(!Base58Alphabet::contains(b'0'));
        assert!(Base36Alphabet::contains(b'A'));
        assert!(!Base36Alphabet::contains(b'a'));
        assert!(Base36LowercaseAlphabet::contains(b'a'));
        assert!(!Base36LowercaseAlphabet::contains(b'A'));
        assert!(Base32Alphabet::contains(b'A'));
        assert!(!Base32Alphabet::contains(b'8'));
        assert!(Base32LowercaseAlphabet::contains(b'a'));
        assert!(!Base32LowercaseAlphabet::contains(b'8'));
        assert!(Base16Alphabet::contains(b'A'));
        assert!(!Base16Alphabet::contains(b'Z'));
        assert!(Base16LowercaseAlphabet::contains(b'a'));
        assert!(!Base16LowercaseAlphabet::contains(b'z'));
    }

    #[test]
    fn test_alphabet_contains_each() {
        fn inner<A: Alphabet>() {
            for i in 0..128 {
                A::contains(i as u8);
            }
        }

        inner::<Base64UrlAlphabet>();
        inner::<Base62Alphabet>();
        inner::<Base58Alphabet>();
        inner::<Base36Alphabet>();
        inner::<Base36LowercaseAlphabet>();
        inner::<Base32Alphabet>();
        inner::<Base32LowercaseAlphabet>();
        inner::<Base16Alphabet>();
        inner::<Base16LowercaseAlphabet>();
    }

    #[test]
    fn test_alphabet_inner_new_valid() {
        let _ = AlphabetInner::new(b"abc123XYZ#");
    }

    #[test]
    #[should_panic]
    fn test_alphabet_inner_new_invalid() {
        let _ = AlphabetInner::new(b"abc123XYZ#\xa0");
    }

    #[test]
    fn test_alphabet_inner_len() {
        let a = AlphabetInner::new(b"abc123XYZ#");
        assert_eq!(a.len(), 10);
    }

    #[test]
    fn test_alphabet_inner_get() {
        let a = AlphabetInner::new(b"abc123XYZ#");
        assert_eq!(a.get(0), b'a');
        assert_eq!(a.get(9), b'#');
    }

    #[test]
    fn test_alphabet_inner_contains() {
        let a = AlphabetInner::new(b"abc123XYZ#");
        assert!(a.contains(b'1'));
        assert!(!a.contains(b'0'));
    }
}
