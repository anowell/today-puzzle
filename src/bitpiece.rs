use std::fmt;

use crate::bitboard::BitBoard;

/// Each piece fits on 4x4 bit board.
///
/// We represent it with a u32 that represents half of a `BitBoard(u64)`
/// Using a bitmaps 8 wide by 4 tall allows using basic shift operations
/// to apply piece masks directly onto a `BitBoard`.
/// The 4 left columns of bits are unused and should always be `0`
///
/// Example "bold plus" piece:
///
/// ```ignore
/// 0 0 0 0 0 1 1 0
/// 0 0 0 0 1 1 1 1
/// 0 0 0 0 1 1 1 1
/// 0 0 0 0 0 1 1 0
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitPiece(pub u32);

impl fmt::Debug for BitPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitPiece(0x{:08x})", self.0)
    }
}

impl BitPiece {
    pub const fn new(b: u32) -> BitPiece {
        BitPiece(b)
    }

    /// Assumes shape is aligned to LSB
    pub const fn width(&self) -> usize {
        if self.0 & 0xFEFEFEFE == 0 {
            1
        } else if self.0 & 0xFCFCFCFC == 0 {
            2
        } else if self.0 & 0xF8F8F8F8 == 0 {
            3
        } else {
            4
        }
    }

    /// Assumes shapes is aligned to LSB
    pub const fn height(&self) -> usize {
        if self.0 & 0xFFFFFFF0 == 0 {
            1
        } else if self.0 & 0xFFFFF0F0 == 0 {
            2
        } else if self.0 & 0xFFF0F0F0 == 0 {
            3
        } else {
            4
        }
    }

    /// Horizontal flip
    ///
    /// Remains in the half
    ///
    /// x' = 3 - x
    /// y' = y
    pub const fn flip(&self) -> BitPiece {
        let mut bp = BitPiece(0);
        let mut i = 0;
        let mut j;
        while i < 4 {
            j = 0;
            while j < 4 {
                if self.0 & (1 << (i * 8 + j)) != 0 {
                    bp.0 |= 1 << (i * 8 + 3 - j)
                }
                j += 1;
            }
            i += 1;
        }
        bp.align()
    }
    /// Rotate 90 deg clockwise
    ///
    /// y' = x
    /// x' = 3-y
    pub const fn rotate(&self) -> BitPiece {
        let mut bp = BitPiece(0);
        let mut i = 0;
        let mut j;
        while i < 4 {
            j = 0;
            while j < 4 {
                if self.0 & (1 << (8 * i + j)) != 0 {
                    bp.0 |= 1 << (8 * j + 3 - i)
                }
                j += 1;
            }
            i += 1;
        }
        bp.align()
    }

    /// Aligns the shape to the LSB (effectively moving it to the bottom-right of the bitmap)
    ///
    /// ```ignore
    /// 0 0 0 0 1 1 1 0
    /// 0 0 0 0 1 1 0 0
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// ```
    ///
    /// becomes:
    ///
    /// ```ignore
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 1 1 1
    /// 0 0 0 0 0 1 1 0
    /// ```
    const fn align(&self) -> BitPiece {
        let mut bp = BitPiece(self.0);
        if bp.0 & 0x07070707 == 0 {
            bp.0 >>= 3;
        } else if bp.0 & 0x03030303 == 0 {
            bp.0 >>= 2;
        } else if bp.0 & 0x01010101 == 0 {
            bp.0 >>= 1;
        }
        if bp.0 & 0x000F0F0F == 0 {
            bp.0 >>= 24;
        } else if bp.0 & 0x00000F0F == 0 {
            bp.0 >>= 16;
        } else if bp.0 & 0x0000000F == 0 {
            bp.0 >>= 8;
        }
        bp
    }

    #[inline]
    pub fn to_bitboard(self, x: usize, y: usize) -> BitBoard {
        BitBoard::new((self.0 as u64) << (y * 8 + x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_align() {
        assert_eq!(BitPiece(0x103), BitPiece(0x103).align());
        assert_eq!(BitPiece(0x203), BitPiece(0x203 << 2).align());
        assert_eq!(BitPiece(0x307), BitPiece(0x307 << 8).align());
        assert_eq!(BitPiece(0x303), BitPiece(0x303 << 10).align());
        assert_eq!(BitPiece(0xF0808), BitPiece(0xF080800).align());
    }

    #[test]
    fn piece_rotate() {
        let piece = BitPiece(0x301);
        assert_eq!(BitPiece(0x103), piece.rotate());
        assert_eq!(BitPiece(0x203), piece.rotate().rotate());
        assert_eq!(BitPiece(0x302), piece.rotate().rotate().rotate());
        assert_eq!(BitPiece(0x301), piece.rotate().rotate().rotate().rotate());
    }

    #[test]
    fn piece_flip() {
        assert_eq!(BitPiece(0x203), BitPiece(0x103).flip());
        assert_eq!(BitPiece(0x103070F), BitPiece(0x80C0E0F).flip());
        assert_eq!(BitPiece(0x1020408), BitPiece(0x8040201).flip());
        assert_eq!(BitPiece(0x1010101), BitPiece(0x1010101).flip());
    }

    #[test]
    fn piece_to_bitboard() {
        assert_eq!(BitPiece(0x203).to_bitboard(0, 0), BitBoard::new(0x0203));
        assert_eq!(BitPiece(0x203).to_bitboard(1, 0), BitBoard::new(0x406));
        assert_eq!(BitPiece(0x203).to_bitboard(0, 1), BitBoard::new(0x020300));
        assert_eq!(BitPiece(0x203).to_bitboard(1, 1), BitBoard::new(0x040600));
        assert_eq!(
            BitPiece(0x0F0F0F0F).to_bitboard(0, 0),
            BitBoard::new(0x0F0F0F0F)
        );
        assert_eq!(
            BitPiece(0x0A0A0A0A).to_bitboard(4, 4),
            BitBoard::new(0xA0A0A0A000000000)
        );
    }
}
