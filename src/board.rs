
use crate::bitboard::BitBoard;
use crate::bitpiece::BitPiece;
use crate::piece::{Piece};
use std::fmt;


/// Board that uses N pieces for it's solution
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board<const N: usize> {
    combined: BitBoard,
    pieces: [BitBoard; N],
    piece_count: usize,
}

/// Soolution is just a collection of bitboards that represent where each piece is
#[derive(Clone, PartialEq, Debug)]
pub struct Solution(pub Vec<BitBoard>);


impl<const N: usize> Board<N> {
    pub fn new(base: BitBoard, solved: BitBoard) -> Board<N> {
        Board {
            pieces: [BitBoard::new(0); N],
            combined: base | !solved,
            piece_count: 0,
        }
    }

    pub fn pieces(&self) -> &[BitBoard] {
        &self.pieces
    }

    pub fn place_piece(&self, piece: BitPiece, x: usize, y: usize) -> Result<Board<N>, &'static str> {
        let piece_bb = piece.to_bitboard(x, y);
        // Check if piece_bb can be placed on the board without overlap
        if piece_bb.intersects(self.combined) {
            // Create a new board that adds the piece
            let mut new_board = *self;
            new_board.pieces[self.piece_count] = piece_bb;
            new_board.piece_count += 1;
            new_board.combined |= piece_bb;
            Ok(new_board)
        } else {
            Err("Cannot place piece")
        }
    }

    pub fn solve(&self, pieces: &[Piece], only_first: bool) -> Vec<Solution> {
        let mut dfs = vec![*self];
        let mut solutions = Vec::new();

        while !dfs.is_empty() {
            let board = dfs.pop().unwrap();
            if board.is_solved() {
                solutions.push(Solution(board.pieces.into()));
                if only_first {
                    break;
                }
            } else {
                board.append_valid_placements(pieces[board.piece_count], &mut dfs);
            }
        }
        solutions
    }

    pub fn append_valid_placements(&self, piece: Piece, buf: &mut Vec<Board<N>>) {
        assert_ne!(self.piece_count, self.pieces.len());
        for variation in piece.variations {
            let w = variation.width();
            let h = variation.height();
            for x in 0..(9 - w) {
                for y in 0..(9 - h) {
                    let piece_bb = variation.to_bitboard(x, y);

                    // Check if piece_bb can be placed on the board without overlap
                    if piece_bb.intersects(self.combined) {
                        // Create a new board that adds the piece
                        let mut new_board = *self;
                        new_board.pieces[self.piece_count] = piece_bb;
                        new_board.piece_count += 1;
                        new_board.combined |= piece_bb;
                        buf.push(new_board);
                    }
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.piece_count == N
    }
}

impl<const N: usize> fmt::Display for Board<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = vec!['.'; 64];

        for x in 0..64 {
            if self.combined.0 & (1u64 << x) == (1u64 << x) {
                map[63 - x] = 'X'
            }
        }
        for i in 0..self.piece_count {
            for x in 0..64 {
                if self.pieces[i].0 & (1u64 << x) == (1u64 << x) {
                    map[63 - x] = char::from_u32(u32::from('A') + i as u32).unwrap();
                }
            }
        }
        let s = map
            .chunks(8)
            .map(|w| w.iter().map(|c| format!("{} ", c)).collect())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}


impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = vec!['.'; 64];

        for i in 0..self.0.len() {
            for x in 0..64 {
                if self.0[i].0 & (1u64 << x) == (1u64 << x) {
                    map[63 - x] = char::from_u32(u32::from('A') + i as u32).unwrap();
                }
            }
        }
        let s = map
            .chunks(8)
            .map(|w| w.iter().map(|c| format!("{} ", c)).collect())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}