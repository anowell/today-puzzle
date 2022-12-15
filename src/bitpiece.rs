use std::fmt;

use crate::bitboard::BitBoard;

/// Each piece fits on 4x4 bit board.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitPiece(pub u16);

impl fmt::Debug for BitPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitPiece(0x{:04x})", self.0)
    }
}

impl BitPiece {
    pub const fn new(b: u16) -> BitPiece {
        BitPiece(b)
    }

    /// Assumes shape is aligned to LSB
    pub const fn width(&self) -> usize {
        if self.0 & 0xEEEE == 0 {
            1
        } else if self.0 & 0xCCCC == 0 {
            2
        } else if self.0 & 0x8888 == 0 {
            3
        } else {
            4
        }
    }

    /// Assumes shapes is aligned to LSB
    pub const fn height(&self) -> usize {
        if self.0 & 0xFFF0 == 0 {
            1
        } else if self.0 & 0xFF00 == 0 {
            2
        } else if self.0 & 0xF000 == 0 {
            3
        } else {
            4
        }
    }

    /// Horizontal flip
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
                if self.0 & (1 << (i * 4 + j)) != 0 {
                    bp.0 |= 1 << (i * 4 + 3 - j)
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
                if self.0 & (1 << (4 * i + j)) != 0 {
                    bp.0 |= 1 << (4 * j + 3 - i)
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
    /// 1 1 1 0
    /// 1 1 0 0
    /// 0 0 0 0
    /// 0 0 0 0
    /// ```
    ///
    /// becomes:
    ///
    /// ```ignore
    /// 0 0 0 0
    /// 0 0 0 0
    /// 0 1 1 1
    /// 0 1 1 0
    /// ```
    const fn align(&self) -> BitPiece {
        let mut bp = BitPiece(self.0);
        if bp.0 & 0x7777 == 0 {
            bp.0 >>= 3;
        } else if bp.0 & 0x3333 == 0 {
            bp.0 >>= 2;
        } else if bp.0 & 0x1111 == 0 {
            bp.0 >>= 1;
        }
        if bp.0 & 0x0FFF == 0 {
            bp.0 >>= 12;
        } else if bp.0 & 0x00FF == 0 {
            bp.0 >>= 8;
        } else if bp.0 & 0x000F == 0 {
            bp.0 >>= 4;
        }
        bp
    }

    #[inline]
    pub fn to_bitboard(self, x: usize, y: usize) -> BitBoard {
        let val = self.0 as u64;
        let mut bb = val & 0xF;
        bb |= (val & 0xF0) << 4;
        bb |= (val & 0xF00) << 8;
        bb |= (val & 0xF000) << 12;
        BitBoard::new(bb << (y * 8 + x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_align() {
        assert_eq!(BitPiece(0x13), BitPiece(0x13).align());
        assert_eq!(BitPiece(0x23), BitPiece(0x23 << 2).align());
        assert_eq!(BitPiece(0x37), BitPiece(0x37 << 8).align());
        assert_eq!(BitPiece(0x33), BitPiece(0x33 << 10).align());
        assert_eq!(BitPiece(0x0F88), BitPiece(0xF880).align());
    }

    #[test]
    fn piece_rotate() {
        let piece = BitPiece(0x31);
        assert_eq!(BitPiece(0x13), piece.rotate());
        assert_eq!(BitPiece(0x23), piece.rotate().rotate());
        assert_eq!(BitPiece(0x32), piece.rotate().rotate().rotate());
        assert_eq!(BitPiece(0x31), piece.rotate().rotate().rotate().rotate());
    }

    #[test]
    fn piece_flip() {
        assert_eq!(BitPiece(0x23), BitPiece(0x13).flip());
        assert_eq!(BitPiece(0x137F), BitPiece(0x8CEF).flip());
        assert_eq!(BitPiece(0x1248), BitPiece(0x8421).flip());
        assert_eq!(BitPiece(0x1111), BitPiece(0x1111).flip());
    }

    #[test]
    fn piece_to_bitboard() {
        assert_eq!(BitPiece(0x23).to_bitboard(0, 0), BitBoard::new(0x0203));
        assert_eq!(BitPiece(0x23).to_bitboard(1, 0), BitBoard::new(0x406));
        assert_eq!(BitPiece(0x23).to_bitboard(0, 1), BitBoard::new(0x020300));
        assert_eq!(BitPiece(0x23).to_bitboard(1, 1), BitBoard::new(0x040600));
        assert_eq!(
            BitPiece(0xFFFF).to_bitboard(0, 0),
            BitBoard::new(0x0F0F0F0F)
        );
        assert_eq!(
            BitPiece(0xAAAA).to_bitboard(4, 4),
            BitBoard::new(0xA0A0A0A000000000)
        );
    }
}
