use crate::bitpiece::BitPiece;

// Unique variations of a given piece - rotations and reflections calculated at compile time
pub const PIECE_RECT: Variations<2> = Variations::<2>::rotations(0x0077); // 6 squares
pub const PIECE_U: Variations<4> = Variations::<4>::rotations(0x0313); // 5 squares
pub const PIECE_CORNER: Variations<4> = Variations::<4>::rotations(0x0117); // 5 squares
pub const PIECE_TALL_S: Variations<4> = Variations::<4>::rotations_and_reflections(0x0326); // 5 squares
pub const PIECE_TALL_L: Variations<8> = Variations::<8>::rotations_and_reflections(0x001F); // 5 squares
pub const PIECE_LONG_Z: Variations<8> = Variations::<8>::rotations_and_reflections(0x003E); // 5 squares
pub const PIECE_UNEVEN_T: Variations<8> = Variations::<8>::rotations_and_reflections(0x002F); // 5 squares
pub const PIECE_SIX: Variations<8> = Variations::<8>::rotations_and_reflections(0x0331); // 5 squares

pub const PIECE_W: Variations<4> = Variations::<4>::rotations(0x0631); // 5 squares
pub const PIECE_H: Variations<8> = Variations::<8>::rotations_and_reflections(0x0175); // 6 squares
pub const PIECE_TALL_T: Variations<4> = Variations::<4>::rotations(0x0227); // 5 squares
pub const PIECE_SQUARE: Variations<1> = Variations::<1>::new(0x0033); // 4 squares
pub const PIECE_L: Variations<8> = Variations::<8>::rotations_and_reflections(0x0017); // 4 squares
pub const PIECE_T: Variations<4> = Variations::<4>::rotations(0x0027); // 4 squares
pub const PIECE_LINE: Variations<2> = Variations::<2>::rotations(0x000F); // 4 squares
pub const PIECE_Z: Variations<4> = Variations::<4>::rotations_and_reflections(0x0036); // 4 squares


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Variations<const N: usize>(pub [BitPiece; N]);

#[derive(Debug, Clone, Copy, PartialEq)]
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
    const fn new(shape: u16) -> Variations<1> {
        Variations([BitPiece::new(shape)])
    }
}

impl Variations<2> {
    /// Generate variations for a piece that only has 1 extra unique rotation
    const fn rotations(shape: u16) -> Variations<2> {
        Variations([BitPiece::new(shape), BitPiece::new(shape).rotate()])
    }
}

impl Variations<4> {
    /// Generate variations for pieces that have 4 unique orientations (reflections are not unique)
    const fn rotations(shape: u16) -> Variations<4> {
        let mut variations = [BitPiece::new(shape); 4];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].rotate();
        variations[3] = variations[2].rotate();
        Variations(variations)
    }

    /// Generate variations for pieces that have 4 unique orientations (2 rotations and their reflections)
    const fn rotations_and_reflections(shape: u16) -> Variations<4> {
        let mut variations = [BitPiece::new(shape); 4];
        variations[1] = variations[0].rotate();
        variations[2] = variations[1].flip();
        variations[3] = variations[2].rotate();
        Variations(variations)
    }
}

impl Variations<8> {
    /// Generate variations for pieces that have 8 unique orientations
    const fn rotations_and_reflections(shape: u16) -> Variations<8> {
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
