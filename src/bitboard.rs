use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not};

/// A good old-fashioned bitboard (borrowed from chess crate)
///
/// Board is shaped as follows:
/// ```ignore
/// Ja Fe Ma Ap Ma Ju XX XX
/// Ju Au Se Oc No De XX XX
/// 01 02 03 04 05 06 07 XX
/// 08 09 10 11 12 13 14 XX
/// 15 16 17 18 19 20 21 XX
/// 22 23 24 25 26 27 28 XX
/// 29 30 31 XX XX XX XX XX
/// ```
///
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Default)]
pub struct BitBoard(pub u64);

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitBoard(0x{:04x})", self.0)
    }
}

// Impl BitAnd
impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: &BitBoard) {
        self.0 &= other.0;
    }
}

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: &BitBoard) {
        self.0 |= other.0;
    }
}

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: &BitBoard) {
        self.0 ^= other.0;
    }
}

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl fmt::Display for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if x % 8 == 7 {
                s.push('\n');
            }
        }
        write!(f, "{s}")
    }
}

// This pattern, aligned to the LSB of 8x8 bitboard
// 0 1 0
// 1 0 1
// 0 1 0
const NEIGHBOR_PATTERN: u64 = 0x020502;

impl BitBoard {
    /// Construct a new bitboard from a u64
    #[inline]
    pub fn new(b: u64) -> BitBoard {
        BitBoard(b)
    }

    #[inline]
    pub fn intersects(&self, other: BitBoard) -> bool {
        self.0 & other.0 != 0
    }

    /// Returns true if the board has any gaps that are too small for a piece
    ///
    /// This allows the solve algorithms to evaluate fewer placements.
    /// Experimentally, this results in up to a 10x improvement on fully solving a puzzle.
    ///
    /// Currently, this only checks for single-square gaps.
    /// A subsequent attempt to detect gaps with at least 3 connected squares
    /// doubled the complexity, but solving a puzzle tended to be 10% slower
    /// than using this simpler implementation.
    pub fn has_small_gaps(self) -> bool {
        for i in 0..64 {
            // For any square that empty, shift the NEIGHBOR_PATTERN to surround that square.
            // Then BitAnd the resulting pattern with inverse of the board
            // Any `1`s in the resulting pattern indicate other gaps connected to this square.
            if self.0 & (1 << i) == 0 {
                let neighbor_pattern = if i < 9 {
                    NEIGHBOR_PATTERN >> (9 - i as u32)
                } else {
                    NEIGHBOR_PATTERN << (i as u32 - 9)
                };
                let neighbors = (!self.0) & neighbor_pattern;
                if neighbors == 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_small_gaps() {
        assert!(!BitBoard(0x0).has_small_gaps());
        assert!(!BitBoard(0xFFFFFFFFFFFFFFFF).has_small_gaps());
        assert!(!BitBoard(0xFE).has_small_gaps());
        assert!(!BitBoard(0xF0).has_small_gaps());
        assert!(!BitBoard(0xAAAAAAAAAAAAAAAA).has_small_gaps());
        assert!(!BitBoard(0x5555555555555555).has_small_gaps());
        assert!(!BitBoard(0xA5A5A5A5A5A5A5A5).has_small_gaps());
        assert!(BitBoard(0xFFFE).has_small_gaps());
        assert!(BitBoard(0xAA55AA55AA55AA55).has_small_gaps());
        assert!(BitBoard(0xFFFFFFF7FFFFFFFF).has_small_gaps());
    }
}
