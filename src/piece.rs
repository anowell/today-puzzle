use crate::bitboard::BitBoard;

pub const PIECE_RECT: Piece<2> = Piece::<2>::rotations(0x0077);
pub const PIECE_U: Piece<4> = Piece::<4>::rotations(0x0313);
pub const PIECE_CORNER: Piece<4> = Piece::<4>::rotations(0x0117);
pub const PIECE_TALL_S: Piece<4> = Piece::<4>::rotations_and_reflections(0x0326);
pub const PIECE_L: Piece<8> = Piece::<8>::rotations_and_reflections(0x001F);
pub const PIECE_LONG_Z: Piece<8> = Piece::<8>::rotations_and_reflections(0x003E);
pub const PIECE_UNEVEN_T: Piece<8> = Piece::<8>::rotations_and_reflections(0x002F);
pub const PIECE_SIX: Piece<8> = Piece::<8>::rotations_and_reflections(0x0331);

pub const PIECES: [PieceRef; 8] = [
    PIECE_RECT.as_ref(),
    PIECE_U.as_ref(),
    PIECE_CORNER.as_ref(),
    PIECE_TALL_S.as_ref(),
    PIECE_L.as_ref(),
    PIECE_LONG_Z.as_ref(),
    PIECE_UNEVEN_T.as_ref(),
    PIECE_SIX.as_ref(),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece<const N: usize> {
    pub(crate) variations: [BitPiece; N],
} // TODO incorporate a width and height so we know how much translation we can do

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PieceRef<'a> {
    pub variations: &'a[BitPiece]
}

impl <const N: usize> Piece<N> {
    pub const fn as_ref(&self) -> PieceRef {
        PieceRef {
            variations: &self.variations
        }
    }
}

impl Piece<2> {
    const fn rotations(shape: u16) -> Piece<2> {
        Piece {
            variations: [BitPiece(shape), BitPiece(shape).rotate()],
        }
    }
}

impl Piece<4> {
    const fn rotations(shape: u16) -> Piece<4> {
        let mut variations = [BitPiece(shape); 4];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].rotate();
        variations[3] = variations[2].rotate();
        Piece { variations }
    }
    const fn rotations_and_reflections(shape: u16) -> Piece<4> {
        let mut variations = [BitPiece(shape); 4];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].flip();
        variations[3] = variations[2].rotate();
        Piece { variations }
    }
}

impl Piece<8> {
    const fn rotations_and_reflections(shape: u16) -> Piece<8> {
        let mut variations = [BitPiece(shape); 8];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].rotate();
        variations[3] = variations[2].rotate();
        variations[4] = variations[3].flip();
        variations[5] = variations[4].rotate();
        variations[6] = variations[5].rotate();
        variations[7] = variations[6].rotate();
        Piece { variations }
    }
}

/// Each piece fits on 4x4 bit board.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitPiece(pub u16);
impl BitPiece {
    /// Horizontal flip
    pub const fn flip(&self) -> BitPiece {
        let mut bp = BitPiece(0);
        let mut i = 0;
        let mut j;
        while i < 4 {
            j = 0;
            while j < 4 {
                if self.0 & (1 << (i * 4 + j)) != 0 {
                    bp.0 = bp.0 | (1 << (i * 4 + 3 - j))
                }
                j += 1;
            }
            i += 1;
        }
        bp.align()
    }

    // assumes aligned
    pub const fn width(&self) -> usize {
        if self.0 & 0xCCCC == 0 {
            2
        } else if self.0 & 0x8888 == 0 {
            3
        } else {
            4
        }
    }

    // assumes aligned
    pub const fn height(&self) -> usize {
        if self.0 & 0xFF00 == 0 {
            2
        } else if self.0 & 0xF000 == 0 {
            3
        } else {
            4
        }

    }
    // Rotate 90^ clockwise
    // y' = x
    // x' = 3-y
    pub const fn rotate(&self) -> BitPiece {
        let mut bp = BitPiece(0);
        let mut i = 0;
        let mut j;
        while i < 4 {
            j = 0;
            while j < 4 {
                if self.0 & (1 << (4 * i + j)) != 0 {
                    bp.0 = bp.0 | (1 << (4 * j + 3 - i))
                }
                j += 1;
            }
            i += 1;
        }
        bp.align()
    }

    const fn align(&self) -> BitPiece {
        let mut bp = BitPiece(self.0);
        if bp.0 & 0x3333 == 0 {
            bp.0 >>= 2;
        } else if bp.0 & 0x1111 == 0 {
            bp.0 >>= 1;
        }
        if bp.0 & 0x00FF == 0 {
            bp.0 >>= 8;
        } else if bp.0 & 0x000F == 0 {
            bp.0 >>= 4;
        }
        bp
    }

    pub fn to_bitboard(&self, x: usize, y: usize) -> BitBoard {
        let mut bp = BitBoard(0);
        for i in 0..4 {
            let seg = (self.0 & (0xF << (4 * i))) >> (4 * i);
            if seg != 0 {
                bp.0 = bp.0 | ((seg as u64) << (8 * (y + i) + x));
            }
        }
        bp
    }
}
