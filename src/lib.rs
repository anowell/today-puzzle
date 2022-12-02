use bitboard::BitBoard;
use chrono::NaiveDate as Date;
pub use piece::Variant;
use piece::{PieceRef, PIECE_COUNT};
use std::fmt;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// Lightweight allocator for smaller wasm size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod bitboard;
pub mod piece;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board {
    combined: BitBoard,
    pieces: [BitBoard; PIECE_COUNT],
    piece_count: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [BitBoard(0); PIECE_COUNT],
            combined: bitboard::EMPTY,
            piece_count: 0,
        }
    }

    pub fn from_month_day(month: u32, day: u32) -> Board {
        let date_bits = !BitBoard::from_date(Date::from_ymd_opt(2020, month, day).unwrap());
        Board {
            pieces: [BitBoard(0); PIECE_COUNT],
            combined: bitboard::EMPTY | date_bits,
            piece_count: 0,
        }
    }

    pub fn append_valid_placements(&self, piece: PieceRef, buf: &mut Vec<Board>) {
        for variation in piece.variations {
            let w = variation.width();
            let h = variation.height();
            for x in 0..(9 - w) {
                for y in 0..(9 - h) {
                    let piece_bb = variation.to_bitboard(x, y);

                    // Check if piece_bb can be placed on the board without overlap
                    if piece_bb & self.combined == BitBoard(0) {
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
}

impl fmt::Display for Board {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = vec!['.'; 64];

        for x in 0..64 {
            if self.combined.0 & (1u64 << x) == (1u64 << x) {
                map[63 - x] = 'X'
            }
        }
        for i in 0..PIECE_COUNT {
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

pub fn solve(variant: Variant, month: u32, day: u32, only_first: bool) -> Vec<Board> {
    let mut dfs = vec![Board::from_month_day(month, day)];
    let mut solutions = Vec::new();
    let pieces = variant.pieces();

    while !dfs.is_empty() {
        let board = dfs.pop().unwrap();
        if board.piece_count == PIECE_COUNT {
            solutions.push(board);
            if only_first {
                break;
            }
        } else {
            board.append_valid_placements(pieces[board.piece_count], &mut dfs);
        }
    }

    solutions
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn solve_date(month: u32, day: u32, variant: u32) -> Board {
    let variant = match variant {
        0 => Variant::Original,
        1 => Variant::Hard,
        _ => unimplemented!("Only support variants 0 and 1"),
    };
    let solutions = solve(variant, month, day, true);
    solutions[0]
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_piece(board: &Board, n: usize) -> u64 {
    board.pieces[n].0
}
