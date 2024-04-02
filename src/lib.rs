//! Generate and parse Nano IDs.
//!
//! Nano ID is a small, secure, URL-friendly, unique string ID.
//! Here's an example of a Nano ID:
//!
//! ```text
//! qjH-6uGrFy0QgNJtUh0_c
//! ```
//!
//! This crate is a Rust implementation of the original [Nano ID](https://github.com/ai/nanoid) library written in JavaScript.
//! Please refer to the original library for the detailed explanation of Nano ID.
//!
//! # Getting started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # TODO
//! ```
//!
//! When you want a new Nano ID, you can generate one using the [`Nanoid::new`] method:
//!
//! ```
//! use nid::Nanoid;
//! let id: Nanoid = Nanoid::new();
//! ```
//!
//! You can also parse a string into a Nano ID using the [`FromStr`](std::str::FromStr) trait:
//!
//! ```
//! use nid::Nanoid;
//! let id: Nanoid = "3hYR3muA_xvjMrrrqFWxF".parse()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! The length of the Nano ID is 21 by default, but you can change it by specifying the generic parameter:
//!
//! ```
//! use nid::Nanoid;
//! let id: Nanoid<10> = "j1-SOTHHxi".parse()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! You can also use a different alphabet:
//!
//! ```
//! use nid::{alphabet::Base62Alphabet, Nanoid};
//! let id: Nanoid<10, Base62Alphabet> = Nanoid::new();
//! ```
//! # Examples
//!
//! ```
//! use nid::{alphabet::Base62Alphabet, Nanoid};
//!
//! // Generate a new Nano ID and print it.
//! let id: Nanoid = Nanoid::new();
//! println!("{}", id);
//!
//! // Parse a string into a Nano ID and convert it back to a string.
//! let id: Nanoid = "abcdefg1234567UVWXYZ_".parse()?;
//! let s = id.to_string();
//!
//! // Parse a string into a Nano ID with a different length and alphabet.
//! let id: Nanoid<9, Base62Alphabet> = "abc123XYZ".parse()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Comparison with other implementations of Nano ID
//!
//! [`nanoid`](https://docs.rs/nanoid) and [`nano-id`](https://docs.rs/nano-id) are other implementations of Nano ID in Rust.
//! The main difference between `nid` and the other implementations is that `nid` has [`Nanoid`] type to represent Nano IDs.
//! This type provides a safe way to generate and parse Nano IDs.
//! This is similar to [`uuid`](https://docs.rs/uuid) crate, which provides [`Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html) type to represent UUIDs.

#![deny(missing_debug_implementations, missing_docs)]

pub mod alphabet;

use std::{marker::PhantomData, mem::MaybeUninit};

use alphabet::{Alphabet, Base64UrlAlphabet};

/// A Nano ID.
///
/// # Generic parameters
///
/// - `N`: The length of the Nano ID. The default is `21`.
/// - `A`: The alphabet used in the Nano ID. The default is [`Base64UrlAlphabet`].
///
/// # Examples
///
/// ```
/// use nid::{alphabet::Base62Alphabet, Nanoid};
///
/// // Generate a new Nano ID and print it.
/// let id: Nanoid = Nanoid::new();
/// println!("{}", id);
///
/// // Parse a string into a Nano ID and convert it back to a string.
/// let id: Nanoid = "abcdefg1234567UVWXYZ_".parse()?;
/// let s = id.to_string();
///
/// // Parse a string into a Nano ID with a different length and alphabet.
/// let id: Nanoid<9, Base62Alphabet> = "abc123XYZ".parse()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Nanoid<const N: usize = 21, A: Alphabet = Base64UrlAlphabet> {
    /// The Nano ID string. All characters are ASCII.
    inner: [u8; N],

    _marker: PhantomData<fn() -> A>,
}

/// An error that can occur when parsing a Nano ID.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// The length of the provided value is not equal to the expected length.
    #[error("Invalid length: expected {expected} bytes, but got {actual} bytes")]
    InvalidLength {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },

    /// The provided value contains a character that is not in the alphabet.
    #[error("Invalid character: {0:x}")]
    InvalidCharacter(u8),
}

impl<const N: usize, A: Alphabet> Nanoid<N, A> {
    /// Generate a new instance of [`Nanoid`] using random number generator seeded by the system.
    ///
    /// # Examples
    ///
    /// ```
    /// use nid::Nanoid;
    /// let id: Nanoid = Nanoid::new();
    /// ```
    ///
    /// # Panics
    ///
    /// The function will panic if the random number generator is not able to generate random numbers
    /// or the provided alphabet produces non-ascii characters.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self::new_with(rand::thread_rng())
    }

    /// Generate a new instance of [`Nanoid`] using the provided random number generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use nid::Nanoid;
    /// let id: Nanoid = Nanoid::new_with(rand::thread_rng());
    /// ```
    ///
    /// # Panics
    ///
    /// The function will panic if the provided random number generator is not able to generate random numbers
    /// or the provided alphabet produces non-ascii characters.
    #[must_use]
    pub fn new_with(mut rng: impl rand::Rng) -> Self {
        // SAFETY: The `assume_init` is safe because the type we are claiming to have initialized
        // here is a bunch of `MaybeUninit`s, which do not require initialization.
        // cf. https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut buf: [MaybeUninit<u8>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let distr = rand::distributions::Uniform::from(0..A::len());
        for b in &mut buf {
            let s = A::get(rng.sample(distr));
            assert!(s.is_ascii(), "the alphabet contains non-ascii characters");
            b.write(s);
        }

        // Convert `MaybeUninit<u8>` to `u8`. `MaybeUninit::assume_init` doesn't work due to the limitation of the compiler.
        // cf. https://github.com/rust-lang/rust/issues/61956
        let buf = {
            let ptr = &mut buf as *mut _ as *mut [u8; N];
            // SAFETY: The `MaybeUninit` array is fully initialized and can be read as an array of `u8`.
            unsafe { ptr.read() }
        };

        Self {
            inner: buf,
            _marker: PhantomData,
        }
    }

    /// Try to parse a string into a [`Nanoid`].
    fn try_from_str(s: &str) -> Result<Self, ParseError> {
        let buf = s
            .as_bytes()
            .try_into()
            .map_err(|_| ParseError::InvalidLength {
                expected: N,
                actual: s.len(),
            })?;

        Self::try_from_bytes(buf)
    }

    /// Try to parse a byte array into a [`Nanoid`].
    fn try_from_bytes(buf: [u8; N]) -> Result<Self, ParseError> {
        for b in buf {
            if !b.is_ascii() || !A::contains(b) {
                return Err(ParseError::InvalidCharacter(b));
            }
        }

        Ok(Nanoid {
            inner: buf,
            _marker: PhantomData,
        })
    }

    /// Get the string representation of the [`Nanoid`].
    ///
    /// You can also use the [`AsRef<str>`] or [`Into<String>`] trait to get the string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use nid::Nanoid;
    /// let id: Nanoid = "Z9ifKfmBL7j69naN7hthu".parse()?;
    /// assert_eq!(id.as_str(), "Z9ifKfmBL7j69naN7hthu");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn as_str(&self) -> &str {
        // SAFETY: all characters are ASCII.
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

// `Copy` cannot be derived due to a limitation of the compiler.
// https://github.com/rust-lang/rust/issues/26925
impl<const N: usize, A: Alphabet> Copy for Nanoid<N, A> {}

// `Clone` cannot be derived as well.
impl<const N: usize, A: Alphabet> Clone for Nanoid<N, A> {
    fn clone(&self) -> Self {
        *self
    }
}

// `PartialEq` cannot be derived as well.
impl<const N: usize, A: Alphabet> PartialEq for Nanoid<N, A> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

// `Eq` cannot be derived as well.
impl<const N: usize, A: Alphabet> Eq for Nanoid<N, A> {}

// `Hash` cannot be derived as well.
impl<const N: usize, A: Alphabet> std::hash::Hash for Nanoid<N, A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

// `PartialOrd` cannot be derived as well.
impl<const N: usize, A: Alphabet> PartialOrd for Nanoid<N, A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// `Ord` cannot be derived as well.
impl<const N: usize, A: Alphabet> Ord for Nanoid<N, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<const N: usize, A: Alphabet> std::fmt::Debug for Nanoid<N, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Nanoid").field(&self.as_str()).finish()
    }
}

impl<const N: usize, A: Alphabet> std::fmt::Display for Nanoid<N, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const N: usize, A: Alphabet> From<Nanoid<N, A>> for String {
    fn from(id: Nanoid<N, A>) -> Self {
        id.as_str().to_owned()
    }
}

impl<const N: usize, A: Alphabet> AsRef<str> for Nanoid<N, A> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize, A: Alphabet> TryFrom<String> for Nanoid<N, A> {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from_str(&s)
    }
}

impl<const N: usize, A: Alphabet> std::str::FromStr for Nanoid<N, A> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

impl<const N: usize, A: Alphabet> serde::Serialize for Nanoid<N, A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de, const N: usize, A: Alphabet> serde::Deserialize<'de> for Nanoid<N, A> {
    fn deserialize<D>(deserializer: D) -> Result<Nanoid<N, A>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::alphabet::{Base58Alphabet, Base62Alphabet};

    #[test]
    fn test_new_unique() {
        fn inner<const N: usize, A: Alphabet>() {
            let id1: Nanoid<N, A> = Nanoid::new();
            let id2: Nanoid<N, A> = Nanoid::new();
            assert_ne!(id1, id2);
        }

        inner::<21, Base64UrlAlphabet>();
        inner::<21, Base62Alphabet>();
        inner::<21, Base58Alphabet>();
        inner::<6, Base64UrlAlphabet>();
        inner::<10, Base62Alphabet>();
        inner::<12, Base58Alphabet>();
    }

    #[test]
    fn test_new_uniformity() {
        fn inner<const N: usize, A: Alphabet>(iterations: usize) {
            let mut counts = HashMap::new();

            for _ in 0..iterations {
                let id: Nanoid<N, A> = Nanoid::new();
                for c in id.as_str().chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }
            }

            assert_eq!(counts.len(), A::len());

            let max_count = counts.values().max().unwrap();
            let min_count = counts.values().min().unwrap();
            let expected_count = counts.values().sum::<usize>() as f64 / counts.len() as f64;
            assert!((max_count - min_count) as f64 / expected_count < 0.05);
        }

        inner::<21, Base64UrlAlphabet>(100_000);
        inner::<21, Base62Alphabet>(100_000);
        inner::<21, Base58Alphabet>(100_000);
        inner::<6, Base64UrlAlphabet>(400_000);
        inner::<10, Base62Alphabet>(200_000);
        inner::<12, Base58Alphabet>(200_000);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_alphabet() {
        struct InvalidAlphabet;
        impl Alphabet for InvalidAlphabet {
            fn len() -> usize {
                1
            }

            fn get(index: usize) -> u8 {
                [b'\xa0'][index]
            }

            fn contains(symbol: u8) -> bool {
                symbol == b'\xa0'
            }
        }

        let _ = Nanoid::<21, InvalidAlphabet>::new();
    }

    #[test]
    fn test_copy() {
        fn inner<const N: usize, A: Alphabet>() {
            let id: Nanoid<N, A> = Nanoid::new();
            let copied = id;
            assert_eq!(id, copied);
        }

        inner::<21, Base64UrlAlphabet>();
        inner::<21, Base62Alphabet>();
        inner::<21, Base58Alphabet>();
        inner::<6, Base64UrlAlphabet>();
        inner::<10, Base62Alphabet>();
        inner::<12, Base58Alphabet>();
    }

    #[test]
    fn test_clone() {
        fn inner<const N: usize, A: Alphabet>() {
            let id: Nanoid<N, A> = Nanoid::new();
            let cloned = Clone::clone(&id);
            assert_eq!(id, cloned);
        }

        inner::<21, Base64UrlAlphabet>();
        inner::<21, Base62Alphabet>();
        inner::<21, Base58Alphabet>();
        inner::<6, Base64UrlAlphabet>();
        inner::<10, Base62Alphabet>();
        inner::<12, Base58Alphabet>();
    }

    #[test]
    fn test_eq() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id1: Nanoid<N, A> = s.parse().unwrap();
            let id2: Nanoid<N, A> = s.parse().unwrap();
            assert_eq!(id1, id2);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_hash() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id1: Nanoid<N, A> = s.parse().unwrap();
            let id2: Nanoid<N, A> = s.parse().unwrap();

            let mut map = HashMap::new();
            map.insert(id1, ());
            assert!(map.contains_key(&id2));
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_partial_cmp() {
        fn inner<const N: usize, A: Alphabet>(s1: &str, s2: &str) {
            let id1: Nanoid<N, A> = s1.parse().unwrap();
            let id2: Nanoid<N, A> = s2.parse().unwrap();
            assert_eq!(id1.partial_cmp(&id2), Some(std::cmp::Ordering::Less));
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQRSTU", "ABCDEFGHIJKLMNOPQRSTV");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234", "ABCDEFGHIJKLMNOPQ5678");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456", "ZBCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-", "abc12_");
        inner::<10, Base62Alphabet>("aBc1234XYZ", "abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123", "abc123XYZ124");
    }

    #[test]
    fn test_cmp() {
        fn inner<const N: usize, A: Alphabet>(s1: &str, s2: &str) {
            let id1: Nanoid<N, A> = s1.parse().unwrap();
            let id2: Nanoid<N, A> = s2.parse().unwrap();
            assert_eq!(id1.cmp(&id2), std::cmp::Ordering::Greater);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQRSTV", "ABCDEFGHIJKLMNOPQRSTU");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ5678", "ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ZBCDEFGHJKLMNPQ123456", "ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12_", "abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ", "aBc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ124", "abc123XYZ123");
    }

    #[test]
    fn test_debug_format() {
        fn inner<const N: usize, A: Alphabet>(s: &str, f: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();
            assert_eq!(format!("{:?}", id), f);
        }

        inner::<21, Base64UrlAlphabet>(
            "ABCDEFGHIJKLMNOPQ123_",
            "Nanoid(\"ABCDEFGHIJKLMNOPQ123_\")",
        );
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234", "Nanoid(\"ABCDEFGHIJKLMNOPQ1234\")");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456", "Nanoid(\"ABCDEFGHJKLMNPQ123456\")");
        inner::<6, Base64UrlAlphabet>("abc12-", "Nanoid(\"abc12-\")");
        inner::<10, Base62Alphabet>("abc1234XYZ", "Nanoid(\"abc1234XYZ\")");
        inner::<12, Base58Alphabet>("abc123XYZ123", "Nanoid(\"abc123XYZ123\")");
    }

    #[test]
    fn test_display_format() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();
            assert_eq!(format!("{}", id), s);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_into_string() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();
            assert_eq!(String::from(id), s);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_as_ref_str() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();
            assert_eq!(id.as_ref(), s);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_try_from_string_valid() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = s.to_string().try_into().unwrap();
            assert_eq!(id.as_str(), s);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_parse_valid() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();
            assert_eq!(id.as_str(), s);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_parse_invalid_length() {
        fn inner<const N: usize, A: Alphabet>(s: &str, e: usize, a: usize) {
            let result: Result<Nanoid<N, A>, _> = s.parse();
            if let Err(ParseError::InvalidLength { expected, actual }) = result {
                assert_eq!(expected, e);
                assert_eq!(actual, a);
            } else {
                panic!("unexpected result: {:?}", result);
            }
        }

        inner::<21, Base64UrlAlphabet>("ABCDEF123!!", 21, 11);
        inner::<21, Base62Alphabet>("#1234567890123456789012345", 21, 26);
        inner::<21, Base58Alphabet>("あいうえお", 21, 15);
        inner::<6, Base64UrlAlphabet>("abcdefg", 6, 7);
        inner::<10, Base62Alphabet>("-_-_", 10, 4);
        inner::<12, Base58Alphabet>("###", 12, 3);
    }

    #[test]
    fn test_parse_invalid_character() {
        fn inner<const N: usize, A: Alphabet>(s: &str, c: u8) {
            let result: Result<Nanoid<N, A>, _> = s.parse();
            if let Err(ParseError::InvalidCharacter(character)) = result {
                assert_eq!(character, c);
            } else {
                panic!("unexpected result: {:?}", result);
            }
        }

        inner::<21, Base64UrlAlphabet>("$TQBHLT47zhMMxee2LRSo", b'$');
        inner::<21, Base62Alphabet>("1234567890-1234567890", b'-');
        inner::<21, Base58Alphabet>("AtDQpkiYrFufeIGWbcSRk", b'I');
        inner::<6, Base64UrlAlphabet>("アイ", 0xe3);
        inner::<10, Base62Alphabet>(" \n \n \n \n \n", b' ');
        inner::<12, Base58Alphabet>("abcdefghijkl", b'l');
    }

    #[test]
    fn test_serialize() {
        fn inner<const N: usize, A: Alphabet>(s: &str, expected_serialized: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();
            let serialized = serde_json::to_string(&id).unwrap();
            assert_eq!(serialized, expected_serialized);
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_", "\"ABCDEFGHIJKLMNOPQ123_\"");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234", "\"ABCDEFGHIJKLMNOPQ1234\"");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456", "\"ABCDEFGHJKLMNPQ123456\"");
        inner::<6, Base64UrlAlphabet>("abc12-", "\"abc12-\"");
        inner::<10, Base62Alphabet>("abc1234XYZ", "\"abc1234XYZ\"");
        inner::<12, Base58Alphabet>("abc123XYZ123", "\"abc123XYZ123\"");
    }

    #[test]
    fn test_deserialize_valid() {
        fn inner<const N: usize, A: Alphabet>(serialized: &str, expected_id: &str) {
            let id: Nanoid<N, A> = serde_json::from_str(serialized).unwrap();
            assert_eq!(id.as_str(), expected_id);
        }

        inner::<21, Base64UrlAlphabet>("\"ABCDEFGHIJKLMNOPQ123_\"", "ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("\"ABCDEFGHIJKLMNOPQ1234\"", "ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("\"ABCDEFGHJKLMNPQ123456\"", "ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("\"abc12-\"", "abc12-");
        inner::<10, Base62Alphabet>("\"abc1234XYZ\"", "abc1234XYZ");
        inner::<12, Base58Alphabet>("\"abc123XYZ123\"", "abc123XYZ123");
    }

    #[test]
    fn test_deserialize_invalid() {
        fn inner<const N: usize, A: Alphabet>(serialized: &str) {
            let result: Result<Nanoid<N, A>, _> = serde_json::from_str(serialized);
            assert!(result.is_err());
        }

        inner::<21, Base64UrlAlphabet>("\"ABCDEF123!!\"");
        inner::<21, Base62Alphabet>("\"#1234567890123456789012345\"");
        inner::<21, Base58Alphabet>("\"あいうえお\"");
        inner::<6, Base64UrlAlphabet>("\"アイ\"");
        inner::<10, Base62Alphabet>("\" \\n \\n \\n \\n \\n\"");
        inner::<12, Base58Alphabet>("\"abcdefghijkl\"");
    }
}
