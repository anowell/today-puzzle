use crate::bitpiece::BitPiece;

// Unique variations of a given piece - rotations and reflections calculated at compile time
pub const PIECE_RECT: Variations<2> = Variations::<2>::rotations(0x707); // 6 squares
pub const PIECE_U: Variations<4> = Variations::<4>::rotations(0x30103); // 5 squares
pub const PIECE_CORNER: Variations<4> = Variations::<4>::rotations(0x10107); // 5 squares
pub const PIECE_TALL_S: Variations<4> = Variations::<4>::rotations_and_reflections(0x30206); // 5 squares
pub const PIECE_TALL_L: Variations<8> = Variations::<8>::rotations_and_reflections(0x10F); // 5 squares
pub const PIECE_LONG_Z: Variations<8> = Variations::<8>::rotations_and_reflections(0x30E); // 5 squares
pub const PIECE_UNEVEN_T: Variations<8> = Variations::<8>::rotations_and_reflections(0x20F); // 5 squares
pub const PIECE_SIX: Variations<8> = Variations::<8>::rotations_and_reflections(0x30301); // 5 squares

pub const PIECE_W: Variations<4> = Variations::<4>::rotations(0x60301); // 5 squares
pub const PIECE_H: Variations<8> = Variations::<8>::rotations_and_reflections(0x10705); // 6 squares
pub const PIECE_TALL_T: Variations<4> = Variations::<4>::rotations(0x20207); // 5 squares
pub const PIECE_SQUARE: Variations<1> = Variations::<1>::new(0x303); // 4 squares
pub const PIECE_L: Variations<8> = Variations::<8>::rotations_and_reflections(0x107); // 4 squares
pub const PIECE_T: Variations<4> = Variations::<4>::rotations(0x207); // 4 squares
pub const PIECE_LINE: Variations<2> = Variations::<2>::rotations(0x0F); // 4 squares
pub const PIECE_Z: Variations<4> = Variations::<4>::rotations_and_reflections(0x306); // 4 squares

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variations<const N: usize>(pub [BitPiece; N]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub variations: &'static [BitPiece],
}

impl<const N: usize> Variations<N> {
    pub const fn as_ref(&'static self) -> Piece {
        Piece {
            variations: &self.0,
        }
    }
}

impl Variations<1> {
    #[allow(unused)]
    /// Instantiate a piece that has no unique rotations or reflections (fully symmetrical)
    const fn new(shape: u32) -> Variations<1> {
        Variations([BitPiece::new(shape)])
    }
}

impl Variations<2> {
    /// Generate variations for a piece that only has 1 extra unique rotation
    const fn rotations(shape: u32) -> Variations<2> {
        Variations([BitPiece::new(shape), BitPiece::new(shape).rotate()])
    }
}

impl Variations<4> {
    /// Generate variations for pieces that have 4 unique orientations (reflections are not unique)
    const fn rotations(shape: u32) -> Variations<4> {
        let mut variations = [BitPiece::new(shape); 4];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].rotate();
        variations[3] = variations[2].rotate();
        Variations(variations)
    }

    /// Generate variations for pieces that have 4 unique orientations (2 rotations and their reflections)
    const fn rotations_and_reflections(shape: u32) -> Variations<4> {
        let mut variations = [BitPiece::new(shape); 4];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].flip();
        variations[3] = variations[2].rotate();
        Variations(variations)
    }
}

impl Variations<8> {
    /// Generate variations for pieces that have 8 unique orientations
    const fn rotations_and_reflections(shape: u32) -> Variations<8> {
        let mut variations = [BitPiece::new(shape); 8];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].rotate();
        variations[3] = variations[2].rotate();
        variations[4] = variations[3].flip();
        variations[5] = variations[4].rotate();
        variations[6] = variations[5].rotate();
        variations[7] = variations[6].rotate();
        Variations(variations)
    }
}
