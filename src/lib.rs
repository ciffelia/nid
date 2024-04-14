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
//! nid = "2.1.0-rc.1"
//! ```
//!
//! When you want a new Nano ID, you can generate one using the [`Nanoid::new`] method.
//!
//! ```
//! use nid::Nanoid;
//! let id: Nanoid = Nanoid::new();
//! ```
//!
//! You can parse a string into a Nano ID using the [`std::str::FromStr`] or [`TryFrom`] trait.
//!
//! ```
//! use nid::Nanoid;
//! let id: Nanoid = "3hYR3muA_xvjMrrrqFWxF".parse()?;
//! let id: Nanoid = "iH26rJ8CpRz-gfIh7TSRu".to_string().try_into()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! If the Nano ID string is constant, you can also use the [`nanoid`] macro to parse it at compile time.
//!
//! ```
//! use nid::{nanoid, Nanoid};
//! let id = nanoid!("ClCrhcvy5kviH5ZozARfi");
//! const ID: Nanoid = nanoid!("9vZZWqFI_rTou3Mutq1LH");
//! ```
//!
//! The length of the Nano ID is 21 by default. You can change it by specifying the generic parameter.
//!
//! ```
//! use nid::Nanoid;
//! let id: Nanoid<10> = "j1-SOTHHxi".parse()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! You can also use a different alphabet. The list of available alphabets is in the [`alphabet`] module.
//!
//! ```
//! use nid::{alphabet::Base62Alphabet, Nanoid};
//! let id: Nanoid<10, Base62Alphabet> = Nanoid::new();
//! ```
//!
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
//! # Features
//!
//! - `serde`: Add support for serialization and deserialization of [`Nanoid`]. Implement [`serde::Serialize`] and [`serde::Deserialize`] for [`Nanoid`].
//! - `zeroize`: Add support for zeroizing the memory of [`Nanoid`]. Implement [`zeroize::Zeroize`] for [`Nanoid`].
//!
//! # Comparison with other implementations of Nano ID
//!
//! [`nanoid`](https://docs.rs/nanoid) and [`nano-id`](https://docs.rs/nano-id) are other implementations of Nano ID in Rust.
//! The main difference between `nid` and the other implementations is that `nid` has [`Nanoid`] type to represent Nano IDs.
//! This type provides a safe way to generate and parse Nano IDs.
//! This is similar to [`uuid`](https://docs.rs/uuid) crate, which provides [`Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html) type to represent UUIDs.

#![cfg_attr(doc_auto_cfg, feature(doc_auto_cfg))]
#![deny(missing_debug_implementations, missing_docs)]

pub mod alphabet;

use std::{marker::PhantomData, mem::MaybeUninit};

use alphabet::{Alphabet, AlphabetExt, Base64UrlAlphabet};

/// A Nano ID.
///
/// # Generic parameters
///
/// - `N`: The length of the Nano ID. The default is `21`.
/// - `A`: The alphabet used in the Nano ID. The default is [`Base64UrlAlphabet`].
///
/// # Generating
///
/// When you want a new Nano ID, you can generate one using the [`Nanoid::new`].
///
/// ```
/// use nid::Nanoid;
/// let id: Nanoid = Nanoid::new();
/// ```
///
/// # Parsing
///
/// You can parse a string into a Nano ID using the [`std::str::FromStr`] or [`TryFrom`] trait.
///
/// ```
/// use nid::Nanoid;
/// let id: Nanoid = "3hYR3muA_xvjMrrrqFWxF".parse()?;
/// let id: Nanoid = "iH26rJ8CpRz-gfIh7TSRu".to_string().try_into()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// If you try to parse an invalid Nano ID, you will get an error.
///
/// ```
/// use nid::{Nanoid, ParseError};
///
/// let result: Result<Nanoid, _> = "61psxw-too_short".parse();
/// assert!(matches!(result, Err(ParseError::InvalidLength { .. })));
///
/// let result: Result<Nanoid, _> = "6yt_invalid_char#####".to_string().try_into();
/// assert!(matches!(result, Err(ParseError::InvalidCharacter(_))));
/// ```
///
/// # Converting to a string
///
/// You can get the string representation of the Nano ID using the [`AsRef<str>`] or [`Display`](std::fmt::Display) trait.
///
/// ```
/// use nid::Nanoid;
/// let id: Nanoid = "Z9ifKfmBL7j69naN7hthu".parse()?;
/// assert_eq!(id.as_ref(), "Z9ifKfmBL7j69naN7hthu");
/// assert_eq!(id.to_string(), "Z9ifKfmBL7j69naN7hthu");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub struct Nanoid<const N: usize = 21, A: Alphabet = Base64UrlAlphabet> {
    /// The Nano ID string. All characters are ASCII.
    inner: [u8; N],

    _marker: PhantomData<fn() -> A>,
}

/// An error that can occur when parsing a string into a Nano ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
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
    /// Generate a new Nano ID using random number generator seeded by the system.
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
    /// The function will panic if the random number generator is not able to generate random numbers.
    /// This function also panics if the provided [`Alphabet`] produces non-ascii characters, but this
    /// never happens unless the alphabet is implemented incorrectly.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self::new_with(rand::thread_rng())
    }

    /// Generate a new Nano ID using the provided random number generator.
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
    /// The function will panic if the provided random number generator is not able to generate random numbers.
    /// This function also panics if the provided [`Alphabet`] produces non-ascii characters, but this
    /// never happens unless the alphabet is implemented incorrectly.
    #[must_use]
    pub fn new_with(mut rng: impl rand::Rng) -> Self {
        // SAFETY: The `assume_init` is safe because the type we are claiming to have initialized
        // here is a bunch of `MaybeUninit`s, which do not require initialization.
        // cf. https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut buf: [MaybeUninit<u8>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let distr = rand::distributions::Uniform::from(0..A::VALID_SYMBOL_LIST.len());
        for b in &mut buf {
            b.write(A::VALID_SYMBOL_LIST[rng.sample(distr)]);
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

    /// Parse a string into a [`Nanoid`].
    ///
    /// # Errors
    ///
    /// - If the length of the string is not equal to the expected length, this method returns [`ParseError::InvalidLength`].
    /// - If the string contains a character that is not in the alphabet, this method returns [`ParseError::InvalidCharacter`].
    pub const fn try_from_str(s: &str) -> Result<Self, ParseError> {
        let s = s.as_bytes();

        // This conversion is copied from the `TryFrom` implementation. We can't call `try_from` here because it's not const.
        // https://github.com/rust-lang/rust/blob/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/array/mod.rs#L250-L264
        let buf = if s.len() == N {
            let ptr = s.as_ptr() as *const [u8; N];
            // SAFETY: ok because we just checked that the length fits
            unsafe { *ptr }
        } else {
            return Err(ParseError::InvalidLength {
                expected: N,
                actual: s.len(),
            });
        };

        Self::try_from_bytes(buf)
    }

    /// Parse a byte array into a [`Nanoid`].
    ///
    /// # Errors
    ///
    /// If the byte array contains a character that is not in the alphabet, this method returns [`ParseError::InvalidCharacter`].
    pub const fn try_from_bytes(buf: [u8; N]) -> Result<Self, ParseError> {
        let mut i = 0;
        while i < N {
            if buf[i] >= A::VALID_SYMBOL_MAP.len() as u8 || !A::VALID_SYMBOL_MAP[buf[i] as usize] {
                return Err(ParseError::InvalidCharacter(buf[i]));
            }
            i += 1;
        }

        Ok(Nanoid {
            inner: buf,
            _marker: PhantomData,
        })
    }

    /// Get the string representation of the [`Nanoid`].
    #[must_use]
    #[inline]
    const fn as_str(&self) -> &str {
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
    #[inline]
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

#[cfg(feature = "serde")]
impl<const N: usize, A: Alphabet> serde::Serialize for Nanoid<N, A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize, A: Alphabet> serde::Deserialize<'de> for Nanoid<N, A> {
    fn deserialize<D>(deserializer: D) -> Result<Nanoid<N, A>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Parse [`Nanoid`]s from strings at compile time.
///
/// This macro transforms a constant string into [`Nanoid`] at compile time.
/// If the provided string is not a valid Nano ID, the program will not compile.
///
/// # Arguments
///
/// - `$id`: The Nano ID string.
/// - `$alphabet`: The alphabet used in the Nano ID. The default is [`Base64UrlAlphabet`].
///
/// # Examples
///
/// ```
/// use nid::{alphabet::Base62Alphabet, nanoid, Nanoid};
///
/// let id1 = nanoid!("F6JA-LPEbPpz71qxDjaId");
/// const ID1: Nanoid = nanoid!("F6JA-LPEbPpz71qxDjaId");
///
/// // With a different length.
/// let id2 = nanoid!("P2_LONIp4S");
/// const ID2: Nanoid<10> = nanoid!("P2_LONIp4S");
///
/// // With a different alphabet.
/// let id3 = nanoid!("F6JAzLPEbPpz71qxDjaId", Base62Alphabet);
/// const ID3: Nanoid<21, Base62Alphabet> = nanoid!("F6JAzLPEbPpz71qxDjaId", Base62Alphabet);
/// ```
///
/// # Compilation errors
///
/// If the provided string is not a valid Nano ID, the program will not compile.
///
/// ```compile_fail
/// use nid::nanoid;
/// let id = nanoid!("abc###"); // Compilation error: the provided string has invalid character
/// ```
#[macro_export]
macro_rules! nanoid {
    ($id:expr $(, $alphabet:ty)? $(,)?) => {{
        const ID: $crate::Nanoid<{ $crate::std::primitive::str::as_bytes($id).len() }$(, $alphabet)?> = match $crate::Nanoid::try_from_str($id) {
            $crate::std::result::Result::Ok(id) => id,
            $crate::std::result::Result::Err($crate::ParseError::InvalidLength { .. }) => {
                $crate::std::unreachable!()
            }
            $crate::std::result::Result::Err($crate::ParseError::InvalidCharacter(_)) => {
                $crate::std::panic!("the provided string has invalid character")
            }
        };
        ID
    }};
}

#[doc(hidden)]
pub use std;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::alphabet::{Base16Alphabet, Base58Alphabet, Base62Alphabet};

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

            assert_eq!(counts.len(), A::VALID_SYMBOL_LIST.len());

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
    fn test_convert_to_string() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = s.parse().unwrap();

            // Test `Display` trait
            assert_eq!(format!("{}", id), s);

            // Test `From<String>` trait
            assert_eq!(String::from(id), s);

            // Test `AsRef<str>` trait
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
    fn test_parse_valid() {
        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let id: Nanoid<N, A> = Nanoid::try_from_str(s).unwrap();
            assert_eq!(id.as_str(), s);

            let id: Nanoid<N, A> = s.to_string().try_into().unwrap();
            assert_eq!(id.as_str(), s);

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
        fn inner<const N: usize, A: Alphabet>(s: &str, expected: usize, actual: usize) {
            let result: Result<Nanoid<N, A>, _> = Nanoid::try_from_str(s);
            assert_eq!(result, Err(ParseError::InvalidLength { expected, actual }));

            let result: Result<Nanoid<N, A>, _> = s.to_string().try_into();
            assert_eq!(result, Err(ParseError::InvalidLength { expected, actual }));

            let result: Result<Nanoid<N, A>, _> = s.parse();
            assert_eq!(result, Err(ParseError::InvalidLength { expected, actual }));
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
        fn inner<const N: usize, A: Alphabet>(s: &str, character: u8) {
            let result: Result<Nanoid<N, A>, _> = Nanoid::try_from_str(s);
            assert_eq!(result, Err(ParseError::InvalidCharacter(character)));

            let result: Result<Nanoid<N, A>, _> = s.to_string().try_into();
            assert_eq!(result, Err(ParseError::InvalidCharacter(character)));

            let result: Result<Nanoid<N, A>, _> = s.parse();
            assert_eq!(result, Err(ParseError::InvalidCharacter(character)));
        }

        inner::<21, Base64UrlAlphabet>("$TQBHLT47zhMMxee2LRSo", b'$');
        inner::<21, Base62Alphabet>("1234567890-1234567890", b'-');
        inner::<21, Base58Alphabet>("AtDQpkiYrFufeIGWbcSRk", b'I');
        inner::<6, Base64UrlAlphabet>("アイ", 0xe3);
        inner::<10, Base62Alphabet>(" \n \n \n \n \n", b' ');
        inner::<12, Base58Alphabet>("abcdefghijkl", b'l');
    }

    #[cfg(feature = "serde")]
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

    #[cfg(feature = "serde")]
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

    #[cfg(feature = "serde")]
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

    #[cfg(feature = "zeroize")]
    #[test]
    fn test_zeroize() {
        use zeroize::Zeroize;

        fn inner<const N: usize, A: Alphabet>(s: &str) {
            let mut id: Nanoid<N, A> = s.parse().unwrap();
            id.zeroize();
        }

        inner::<21, Base64UrlAlphabet>("ABCDEFGHIJKLMNOPQ123_");
        inner::<21, Base62Alphabet>("ABCDEFGHIJKLMNOPQ1234");
        inner::<21, Base58Alphabet>("ABCDEFGHJKLMNPQ123456");
        inner::<6, Base64UrlAlphabet>("abc12-");
        inner::<10, Base62Alphabet>("abc1234XYZ");
        inner::<12, Base58Alphabet>("abc123XYZ123");
    }

    #[test]
    fn test_nanoid_macro() {
        {
            let id = nanoid!("vj-JewhEyrcoWbaLEXTp-");
            const ID: Nanoid = nanoid!("vj-JewhEyrcoWbaLEXTp-");
            assert_eq!(id.as_str(), "vj-JewhEyrcoWbaLEXTp-");
            assert_eq!(ID.as_str(), "vj-JewhEyrcoWbaLEXTp-");
        }

        {
            let id = nanoid!("4KC9zU3v_8mLJokZ");
            const ID: Nanoid<16> = nanoid!("4KC9zU3v_8mLJokZ");
            assert_eq!(id.as_str(), "4KC9zU3v_8mLJokZ");
            assert_eq!(ID.as_str(), "4KC9zU3v_8mLJokZ");
        }

        {
            let id = nanoid!("5B0AD0A10D", Base16Alphabet);
            const ID: Nanoid<10, Base16Alphabet> = nanoid!("5B0AD0A10D", Base16Alphabet);
            assert_eq!(id.as_str(), "5B0AD0A10D");
            assert_eq!(ID.as_str(), "5B0AD0A10D");
        }

        nanoid!("vj-JewhEyrcoWbaLEXTp-",);
        nanoid!("5B0AD0A10D", Base16Alphabet,);
    }
}
