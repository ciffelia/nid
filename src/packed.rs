//! Packed byte representation for Nano IDs.
//!
//! This module provides a compact storage format for Nano IDs where multiple
//! characters are packed into fewer bytes based on the alphabet size.
//!
//! # Example
//!
//! ```
//! use nid::{Nanoid, alphabet::Base64UrlAlphabet, packed::PackedNanoid};
//!
//! let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
//! let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id)?;
//! let unpacked: Nanoid<21, Base64UrlAlphabet> = packed.unpack()?;
//! assert_eq!(id, unpacked);
//! # Ok::<(), nid::packed::PackError>(())
//! ```

use std::marker::PhantomData;

use crate::alphabet::{Alphabet, AlphabetExt};
use crate::Nanoid;

/// An error that can occur during pack/unpack operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
pub enum PackError {
    /// The character is not in the alphabet (during pack).
    #[error("Invalid character '{char}' at position {position}")]
    InvalidCharacter {
        /// The position in the Nano ID.
        position: usize,
        /// The invalid character.
        char: char,
    },

    /// The character index is out of range for the alphabet (during unpack).
    #[error("Invalid character index {index} at position {position}")]
    InvalidIndex {
        /// The position in the Nano ID.
        position: usize,
        /// The invalid index value.
        index: usize,
    },
}

/// An extension trait for [`Alphabet`] that provides the pack size and reverse lookup.
///
/// This trait defines how many bits are needed to represent each character
/// in the packed representation, and provides a reverse lookup map for
/// O(1) character-to-index conversion.
pub trait AlphabetPackExt: Alphabet {
    /// Number of bits per character in the packed representation.
    const PACK_BITS: usize;

    /// Reverse lookup map: maps ASCII character to its index in the alphabet.
    /// Value is 255 (u8::MAX) for characters not in the alphabet.
    const CHAR_TO_INDEX: [u8; 128];

    /// Get the index of a character in the alphabet.
    /// Returns `None` if the character is not in the alphabet or not ASCII.
    #[inline]
    fn char_to_index(ch: u8) -> Option<usize> {
        if ch >= 128 {
            return None;
        }
        let idx = Self::CHAR_TO_INDEX[ch as usize];
        if idx == u8::MAX {
            None
        } else {
            Some(idx as usize)
        }
    }
}

/// Blanket implementation of [`AlphabetPackExt`] for all [`Alphabet`] types.
///
/// This automatically computes:
/// - `PACK_BITS` from the alphabet size.
/// - `CHAR_TO_INDEX` from the symbol list.
impl<A: Alphabet> AlphabetPackExt for A {
    const PACK_BITS: usize = (<Self as Alphabet>::SYMBOL_LIST.len() - 1).ilog2() as usize + 1;

    const CHAR_TO_INDEX: [u8; 128] = {
        let mut map = [u8::MAX; 128];
        let mut i = 0;
        while i < <Self as Alphabet>::SYMBOL_LIST.len() {
            map[<Self as Alphabet>::SYMBOL_LIST[i] as usize] = i as u8;
            i += 1;
        }
        map
    };
}

/// A packed byte representation of a Nano ID.
///
/// This struct stores Nano IDs more efficiently by packing multiple characters
/// into each byte based on the alphabet's pack size.
///
/// # Type Parameters
///
/// - `N`: Number of characters (same as the corresponding `Nanoid`)
/// - `A`: Alphabet type that implements [`AlphabetPackExt`]
/// - `B`: Number of packed bytes (`ceil(N * A::PACK_BITS / 8)`)
///
/// # Example
///
/// ```
/// use nid::{Nanoid, alphabet::Base64UrlAlphabet, packed::PackedNanoid};
///
/// // A 21-character Base64Url Nano ID packs into 16 bytes
/// let id: Nanoid<21, Base64UrlAlphabet> = "qjH-6uGrFy0QgNJtUh0_c".parse()?;
/// let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id)?;
///
/// // Get the raw packed bytes
/// let bytes: &[u8; 16] = packed.as_bytes();
///
/// // Unpack back to the original Nano ID
/// let unpacked: Nanoid<21, Base64UrlAlphabet> = packed.unpack()?;
/// assert_eq!(id, unpacked);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub struct PackedNanoid<const N: usize, A: AlphabetPackExt, const B: usize> {
    inner: [u8; B],
    _marker: PhantomData<fn() -> A>,
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> PackedNanoid<N, A, B> {
    /// Pack a [`Nanoid`] into a [`PackedNanoid`].
    ///
    /// # Errors
    ///
    /// Returns [`PackError::InvalidIndex`] if a character cannot be found in the alphabet.
    /// This should never happen for a valid `Nanoid`.
    ///
    /// # Example
    ///
    /// ```
    /// use nid::{Nanoid, alphabet::Base64UrlAlphabet, packed::PackedNanoid};
    ///
    /// let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
    /// let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id)?;
    /// # Ok::<(), nid::packed::PackError>(())
    /// ```
    pub fn pack(nanoid: &Nanoid<N, A>) -> Result<Self, PackError> {
        let mut packed = [0u8; B];
        Self::pack_impl(&nanoid.inner, &mut packed)?;
        Ok(Self {
            inner: packed,
            _marker: PhantomData,
        })
    }

    /// Unpack a [`PackedNanoid`] back to a [`Nanoid`].
    ///
    /// # Errors
    ///
    /// Returns [`PackError::InvalidIndex`] if the packed data contains an invalid
    /// character index for the alphabet.
    ///
    /// # Example
    ///
    /// ```
    /// use nid::{Nanoid, alphabet::Base64UrlAlphabet, packed::PackedNanoid};
    ///
    /// let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
    /// let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id)?;
    /// let unpacked: Nanoid<21, Base64UrlAlphabet> = packed.unpack()?;
    /// assert_eq!(id, unpacked);
    /// # Ok::<(), nid::packed::PackError>(())
    /// ```
    pub fn unpack(&self) -> Result<Nanoid<N, A>, PackError> {
        let mut chars = [0u8; N];
        Self::unpack_impl(&self.inner, &mut chars)?;

        // SAFETY: unpack_impl validates all characters
        Ok(Nanoid {
            inner: chars,
            _marker: PhantomData,
        })
    }

    /// Get the packed bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use nid::{Nanoid, alphabet::Base64UrlAlphabet, packed::PackedNanoid};
    ///
    /// let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
    /// let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id)?;
    /// let bytes: &[u8; 16] = packed.as_bytes();
    /// # Ok::<(), nid::packed::PackError>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; B] {
        &self.inner
    }

    /// Create a [`PackedNanoid`] from raw packed bytes without validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure the bytes represent a valid packed Nano ID
    /// (i.e., unpacking them produces a valid `Nanoid`).
    #[must_use]
    #[inline]
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; B]) -> Self {
        Self {
            inner: bytes,
            _marker: PhantomData,
        }
    }

    fn pack_impl(src: &[u8; N], dst: &mut [u8; B]) -> Result<(), PackError> {
        let pack_bits = A::PACK_BITS;
        let mut bit_buffer: u64 = 0;
        let mut bits_in_buffer: usize = 0;
        let mut dst_idx: usize = 0;

        for (i, &ch) in src.iter().enumerate() {
            let idx = A::char_to_index(ch).ok_or(PackError::InvalidCharacter {
                position: i,
                char: ch as char,
            })?;

            // Add to bit buffer (MSB first)
            bit_buffer = (bit_buffer << pack_bits) | (idx as u64);
            bits_in_buffer += pack_bits;

            // Extract complete bytes
            while bits_in_buffer >= 8 && dst_idx < B {
                bits_in_buffer -= 8;
                dst[dst_idx] = ((bit_buffer >> bits_in_buffer) & 0xFF) as u8;
                dst_idx += 1;
            }
        }

        // Handle remaining bits (left-padded with zeros)
        if bits_in_buffer > 0 && dst_idx < B {
            dst[dst_idx] = ((bit_buffer << (8 - bits_in_buffer)) & 0xFF) as u8;
        }

        Ok(())
    }

    fn unpack_impl(src: &[u8; B], dst: &mut [u8; N]) -> Result<(), PackError> {
        let pack_bits = A::PACK_BITS;
        let mask = (1u64 << pack_bits) - 1;
        let mut bit_buffer: u64 = 0;
        let mut bits_in_buffer: usize = 0;
        let mut src_idx: usize = 0;

        for (i, dst_byte) in dst.iter_mut().enumerate() {
            // Ensure we have enough bits
            while bits_in_buffer < pack_bits && src_idx < B {
                bit_buffer = (bit_buffer << 8) | (src[src_idx] as u64);
                bits_in_buffer += 8;
                src_idx += 1;
            }

            // Extract pack_bits from the top
            bits_in_buffer -= pack_bits;
            let idx = ((bit_buffer >> bits_in_buffer) & mask) as usize;

            // Validate index
            if idx >= A::VALID_SYMBOL_LIST.len() {
                return Err(PackError::InvalidIndex {
                    position: i,
                    index: idx,
                });
            }

            *dst_byte = A::VALID_SYMBOL_LIST[idx];
        }

        Ok(())
    }
}

// Manual trait implementations (same pattern as Nanoid)

impl<const N: usize, A: AlphabetPackExt, const B: usize> Copy for PackedNanoid<N, A, B> {}

impl<const N: usize, A: AlphabetPackExt, const B: usize> Clone for PackedNanoid<N, A, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> PartialEq for PackedNanoid<N, A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> Eq for PackedNanoid<N, A, B> {}

impl<const N: usize, A: AlphabetPackExt, const B: usize> std::hash::Hash for PackedNanoid<N, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> PartialOrd for PackedNanoid<N, A, B> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> Ord for PackedNanoid<N, A, B> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> std::fmt::Debug for PackedNanoid<N, A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PackedNanoid").field(&self.inner).finish()
    }
}

impl<const N: usize, A: AlphabetPackExt, const B: usize> AsRef<[u8; B]> for PackedNanoid<N, A, B> {
    fn as_ref(&self) -> &[u8; B] {
        &self.inner
    }
}

/// Create a [`PackedNanoid`] type with automatic byte size computation.
///
/// This macro computes the required byte size `B` based on the number of
/// characters `N` and the alphabet's `PACK_BITS` using the formula:
/// `ceil(N * PACK_BITS / 8)` (same as [`AlphabetPackExt::packed_bytes`]).
///
/// # Example
///
/// ```
/// use nid::{alphabet::Base64UrlAlphabet, packed_nanoid_type};
///
/// // Creates type PackedNanoid<21, Base64UrlAlphabet, 16>
/// // (21 * 6 bits = 126 bits = 16 bytes)
/// type PackedId = packed_nanoid_type!(21, Base64UrlAlphabet);
/// ```
#[macro_export]
macro_rules! packed_nanoid_type {
    ($n:expr, $alphabet:ty) => {
        $crate::PackedNanoid<
            $n,
            $alphabet,
            {
                ($n * <$alphabet as $crate::packed::AlphabetPackExt>::PACK_BITS + 7) / 8
            },
        >
    };
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::alphabet::{
        Base16Alphabet, Base32Alphabet, Base36Alphabet, Base58Alphabet, Base62Alphabet,
        Base64UrlAlphabet,
    };

    #[test]
    fn test_pack_bits_values() {
        assert_eq!(Base16Alphabet::PACK_BITS, 4);
        assert_eq!(Base32Alphabet::PACK_BITS, 5);
        assert_eq!(Base36Alphabet::PACK_BITS, 6);
        assert_eq!(Base58Alphabet::PACK_BITS, 6);
        assert_eq!(Base62Alphabet::PACK_BITS, 6);
        assert_eq!(Base64UrlAlphabet::PACK_BITS, 6);
    }

    #[test]
    fn test_roundtrip_base64url() {
        for _ in 0..100 {
            let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
            let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<10, Base64UrlAlphabet> = Nanoid::new();
            let packed: PackedNanoid<10, Base64UrlAlphabet, 8> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<8, Base64UrlAlphabet> = Nanoid::new();
            let packed: PackedNanoid<8, Base64UrlAlphabet, 6> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<4, Base64UrlAlphabet> = Nanoid::new();
            let packed: PackedNanoid<4, Base64UrlAlphabet, 3> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }
    }

    #[test]
    fn test_roundtrip_base32() {
        for _ in 0..100 {
            let id: Nanoid<21, Base32Alphabet> = Nanoid::new();
            let packed: PackedNanoid<21, Base32Alphabet, 14> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<10, Base32Alphabet> = Nanoid::new();
            let packed: PackedNanoid<10, Base32Alphabet, 7> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<8, Base32Alphabet> = Nanoid::new();
            let packed: PackedNanoid<8, Base32Alphabet, 5> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }
    }

    #[test]
    fn test_roundtrip_base16() {
        for _ in 0..100 {
            let id: Nanoid<21, Base16Alphabet> = Nanoid::new();
            let packed: PackedNanoid<21, Base16Alphabet, 11> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<10, Base16Alphabet> = Nanoid::new();
            let packed: PackedNanoid<10, Base16Alphabet, 5> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }

        for _ in 0..100 {
            let id: Nanoid<8, Base16Alphabet> = Nanoid::new();
            let packed: PackedNanoid<8, Base16Alphabet, 4> = PackedNanoid::pack(&id).unwrap();
            let unpacked = packed.unpack().unwrap();
            assert_eq!(id, unpacked);
        }
    }

    #[test]
    fn test_packed_size_reduction() {
        // Base64Url: 21 chars * 6 bits = 126 bits = 16 bytes (vs 21 bytes)
        let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
        let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id).unwrap();
        assert_eq!(packed.as_bytes().len(), 16);
        assert!(packed.as_bytes().len() < 21);

        // Base32: 21 chars * 5 bits = 105 bits = 14 bytes (vs 21 bytes)
        let id: Nanoid<21, Base32Alphabet> = Nanoid::new();
        let packed: PackedNanoid<21, Base32Alphabet, 14> = PackedNanoid::pack(&id).unwrap();
        assert_eq!(packed.as_bytes().len(), 14);

        // Base16: 21 chars * 4 bits = 84 bits = 11 bytes (vs 21 bytes)
        let id: Nanoid<21, Base16Alphabet> = Nanoid::new();
        let packed: PackedNanoid<21, Base16Alphabet, 11> = PackedNanoid::pack(&id).unwrap();
        assert_eq!(packed.as_bytes().len(), 11);
    }

    #[test]
    fn test_copy() {
        fn inner<const N: usize, A: AlphabetPackExt, const B: usize>() {
            let id: Nanoid<N, A> = Nanoid::new();
            let packed: PackedNanoid<N, A, B> = PackedNanoid::pack(&id).unwrap();
            let copied = packed;
            assert_eq!(packed, copied);
        }

        inner::<21, Base64UrlAlphabet, 16>();
        inner::<21, Base32Alphabet, 14>();
        inner::<21, Base16Alphabet, 11>();
    }

    #[test]
    fn test_clone() {
        fn inner<const N: usize, A: AlphabetPackExt, const B: usize>() {
            let id: Nanoid<N, A> = Nanoid::new();
            let packed: PackedNanoid<N, A, B> = PackedNanoid::pack(&id).unwrap();
            let cloned = Clone::clone(&packed);
            assert_eq!(packed, cloned);
        }

        inner::<21, Base64UrlAlphabet, 16>();
        inner::<21, Base32Alphabet, 14>();
        inner::<21, Base16Alphabet, 11>();
    }

    #[test]
    fn test_eq() {
        let id: Nanoid<21, Base64UrlAlphabet> = "ABCDEFGHIJKLMNOPQ123_".parse().unwrap();
        let packed1: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id).unwrap();
        let packed2: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id).unwrap();
        assert_eq!(packed1, packed2);
    }

    #[test]
    fn test_ne() {
        let id1: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
        let id2: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
        let packed1: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id1).unwrap();
        let packed2: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id2).unwrap();
        assert_ne!(packed1, packed2);
    }

    #[test]
    fn test_debug_format() {
        let id: Nanoid<21, Base64UrlAlphabet> = "ABCDEFGHIJKLMNOPQ123_".parse().unwrap();
        let packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id).unwrap();
        let debug_str = format!("{:?}", packed);
        assert!(debug_str.starts_with("PackedNanoid(["));
    }

    #[test]
    fn test_packed_nanoid_type_macro() {
        type Packed64 = packed_nanoid_type!(21, Base64UrlAlphabet);
        let id: Nanoid<21, Base64UrlAlphabet> = Nanoid::new();
        let packed: Packed64 = PackedNanoid::pack(&id).unwrap();
        let unpacked: Nanoid<21, Base64UrlAlphabet> = packed.unpack().unwrap();
        assert_eq!(id, unpacked);
    }

    #[test]
    fn test_known_values_base16() {
        // Base16Alphabet: "ABCDEF0123456789"
        // '0' = index 6 = 0b0110
        // '1' = index 7 = 0b0111
        // '2' = index 8 = 0b1000
        // '3' = index 9 = 0b1001
        // Packed MSB-first: 01100111 10001001 = 0x67, 0x89
        let id: Nanoid<4, Base16Alphabet> = "0123".parse().unwrap();
        let packed: PackedNanoid<4, Base16Alphabet, 2> = PackedNanoid::pack(&id).unwrap();
        assert_eq!(packed.as_bytes(), &[0x67, 0x89]);

        // Round trip
        let unpacked = packed.unpack().unwrap();
        assert_eq!(unpacked.as_str(), "0123");
    }

    #[test]
    fn test_known_values_base32() {
        // Base32: A-Z, 2-7 (indices 0-31)
        // "ABC" -> indices [0, 1, 2] -> 00000 00001 00010 -> 00000000 0100010x
        //                                           -> [0x00, 0x44] with padding
        let id: Nanoid<3, Base32Alphabet> = "ABC".parse().unwrap();
        let packed: PackedNanoid<3, Base32Alphabet, 2> = PackedNanoid::pack(&id).unwrap();
        // 0b00000000 = 0x00, 0b01000100 = 0x44
        assert_eq!(packed.as_bytes(), &[0x00, 0x44]);

        // Round trip
        let unpacked = packed.unpack().unwrap();
        assert_eq!(unpacked.as_str(), "ABC");
    }

    #[cfg(feature = "zeroize")]
    #[test]
    fn test_zeroize() {
        use zeroize::Zeroize;

        let id: Nanoid<21, Base64UrlAlphabet> = "ABCDEFGHIJKLMNOPQ123_".parse().unwrap();
        let mut packed: PackedNanoid<21, Base64UrlAlphabet, 16> = PackedNanoid::pack(&id).unwrap();
        packed.zeroize();
    }

    #[test]
    fn test_char_to_index() {
        // Test Base64UrlAlphabet
        assert_eq!(Base64UrlAlphabet::char_to_index(b'A'), Some(0));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'Z'), Some(25));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'a'), Some(26));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'z'), Some(51));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'0'), Some(52));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'9'), Some(61));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'_'), Some(62));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'-'), Some(63));
        assert_eq!(Base64UrlAlphabet::char_to_index(b'@'), None);
        assert_eq!(Base64UrlAlphabet::char_to_index(b' '), None);
        assert_eq!(Base64UrlAlphabet::char_to_index(0x80), None); // non-ASCII

        // Test Base16Alphabet
        assert_eq!(Base16Alphabet::char_to_index(b'A'), Some(0));
        assert_eq!(Base16Alphabet::char_to_index(b'F'), Some(5));
        assert_eq!(Base16Alphabet::char_to_index(b'0'), Some(6));
        assert_eq!(Base16Alphabet::char_to_index(b'9'), Some(15));
        assert_eq!(Base16Alphabet::char_to_index(b'G'), None);
        assert_eq!(Base16Alphabet::char_to_index(b'a'), None);

        // Test Base32Alphabet
        assert_eq!(Base32Alphabet::char_to_index(b'A'), Some(0));
        assert_eq!(Base32Alphabet::char_to_index(b'Z'), Some(25));
        assert_eq!(Base32Alphabet::char_to_index(b'2'), Some(26));
        assert_eq!(Base32Alphabet::char_to_index(b'7'), Some(31));
        assert_eq!(Base32Alphabet::char_to_index(b'1'), None);
        assert_eq!(Base32Alphabet::char_to_index(b'8'), None);
    }
}
