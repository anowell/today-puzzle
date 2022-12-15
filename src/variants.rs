use crate::bitboard::BitBoard;
use crate::board::{Board, Solution};
use crate::piece::*;
use chrono::{Datelike, NaiveDate};

/// Board use by:
/// - DragonFjord [A-Puzzle-A-Day](https://www.dragonfjord.com/product/a-puzzle-a-day/)
/// - JarringWords [Calendar Puzzle](https://www.etsy.com/jp/listing/1032608229/)
/// - CreaMakerspace [Calendar Puzzle](https://anowell.com/posts/calendar-puzzle)
///
/// Board is shaped as follows:
///
/// ```ignore
/// Ja Fe Ma Ap Ma Ju XX XX
/// Ju Au Se Oc No De XX XX
/// 01 02 03 04 05 06 07 XX
/// 08 09 10 11 12 13 14 XX
/// 15 16 17 18 19 20 21 XX
/// 22 23 24 25 26 27 28 XX
/// 29 30 31 XX XX XX XX XX
/// XX XX XX XX XX XX XX XX
/// ```
pub const BITBOARD_STANDARD: BitBoard = BitBoard(0x0303_0101_0101_1FFF);

/// Board for Tetromino [Puzzle containing quad pieces](https://puzzleparadise.net/listing/puzzle-calendar-solve-for-each-day-of-the-year-cherry-pieces-and-walnut-border/107535)
///
/// Board is shaped as follows:
///
/// ```ignore
/// Ja Fe Ma Ap Ma Ju XX XX
/// Ju Au Se Oc No De XX XX
/// 01 02 03 04 05 06 07 XX
/// 08 09 10 11 12 13 14 XX
/// 15 16 17 18 19 20 21 XX
/// 22 23 24 25 26 27 28 XX
/// XX XX XX XX 29 30 31 XX
/// XX XX XX XX XX XX XX XX
/// ```
pub const BITBOARD_TETROMINO: BitBoard = BitBoard(0x0303_0101_0101_F1FF);

pub trait Variant<const N: usize>: Sized {
    fn board(date: NaiveDate) -> Board<N>;
    fn pieces() -> [Piece; N];

    fn solve_once(date: NaiveDate) -> Option<Solution> {
        Self::board(date)
            .solve(&Self::pieces(), true)
            .first()
            .cloned()
    }

    fn solve_fully(date: NaiveDate) -> Vec<Solution> {
        Self::board(date).solve(&Self::pieces(), false)
    }
}

pub struct DragonFjord;
impl Variant<8> for DragonFjord {
    fn board(date: NaiveDate) -> Board<8> {
        Board::new(BITBOARD_STANDARD, standard_bitboard_from_date(date))
    }

    fn pieces() -> [Piece; 8] {
        [
            PIECE_RECT.as_ref(),
            PIECE_U.as_ref(),
            PIECE_CORNER.as_ref(),
            PIECE_TALL_S.as_ref(),
            PIECE_TALL_L.as_ref(),
            PIECE_LONG_Z.as_ref(),
            PIECE_UNEVEN_T.as_ref(),
            PIECE_SIX.as_ref(),
        ]
    }
}

pub struct JarringWords;
impl Variant<8> for JarringWords {
    fn board(date: NaiveDate) -> Board<8> {
        Board::new(BITBOARD_STANDARD, standard_bitboard_from_date(date))
    }

    fn pieces() -> [Piece; 8] {
        [
            PIECE_RECT.as_ref(),
            PIECE_U.as_ref(),
            PIECE_CORNER.as_ref(),
            PIECE_TALL_T.as_ref(),
            PIECE_TALL_L.as_ref(),
            PIECE_LONG_Z.as_ref(),
            PIECE_UNEVEN_T.as_ref(),
            PIECE_SIX.as_ref(),
        ]
    }
}

pub struct CreaMakerspace;
impl Variant<8> for CreaMakerspace {
    fn board(date: NaiveDate) -> Board<8> {
        Board::new(BITBOARD_STANDARD, standard_bitboard_from_date(date))
    }

    fn pieces() -> [Piece; 8] {
        [
            PIECE_H.as_ref(),
            PIECE_U.as_ref(),
            PIECE_CORNER.as_ref(),
            PIECE_W.as_ref(),
            PIECE_TALL_L.as_ref(),
            PIECE_LONG_Z.as_ref(),
            PIECE_UNEVEN_T.as_ref(),
            PIECE_SIX.as_ref(),
        ]
    }
}

pub struct Tetromino;
impl Variant<9> for Tetromino {
    fn board(date: NaiveDate) -> Board<9> {
        Board::new(BITBOARD_TETROMINO, tetromino_bitboard_from_date(date))
    }

    fn pieces() -> [Piece; 9] {
        [
            PIECE_SQUARE.as_ref(),
            PIECE_LINE.as_ref(),
            PIECE_RECT.as_ref(),
            PIECE_U.as_ref(),
            PIECE_CORNER.as_ref(),
            PIECE_Z.as_ref(),
            PIECE_L.as_ref(),
            PIECE_SIX.as_ref(),
            PIECE_T.as_ref(),
        ]
    }
}

/// Generates a standard bitboard with only the month and day cleared
pub(crate) fn standard_bitboard_from_date(d: NaiveDate) -> BitBoard {
    let month_part = match d.month() {
        m @ 1..=6 => 1 << (16 - m),
        m @ 7..=12 => 1 << (14 - m),
        _ => unreachable!("Invalid month"),
    };
    let day_part = match d.day() {
        d @ 1..=7 => 1 << (48 - d),
        d @ 8..=14 => 1 << (47 - d),
        d @ 15..=21 => 1 << (46 - d),
        d @ 22..=28 => 1 << (45 - d),
        d @ 29..=31 => 1 << (44 - d),
        _ => unreachable!("Invalid day"),
    };

    BitBoard(!((month_part << 48) | day_part))
}

/// Generates a tetromino bitboard with only the month and day cleared
pub(crate) fn tetromino_bitboard_from_date(d: NaiveDate) -> BitBoard {
    let month_part = match d.month() {
        m @ 1..=6 => 1 << (16 - m),
        m @ 7..=12 => 1 << (14 - m),
        _ => unreachable!("Invalid month"),
    };
    let day_part = match d.day() {
        d @ 1..=7 => 1 << (48 - d),
        d @ 8..=14 => 1 << (47 - d),
        d @ 15..=21 => 1 << (46 - d),
        d @ 22..=28 => 1 << (45 - d),
        d @ 29..=31 => 1 << (40 - d),
        _ => unreachable!("Invalid day"),
    };

    BitBoard(!((month_part << 48) | day_part))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_date() -> NaiveDate {
        chrono::NaiveDate::from_ymd_opt(20, 12, 1).unwrap()
    }

    fn assert_solution(solution: Solution, range: impl Iterator<Item = char>) {
        for letter in range {
            assert!(
                solution.to_string().contains(letter),
                "\n{}\nsolution missing piece {}",
                solution,
                letter
            )
        }
    }

    #[test]
    fn dragon_fjord() {
        let solution = DragonFjord::solve_once(test_date()).expect("did not find solution");
        assert_solution(solution, 'A'..='H')
    }

    #[test]
    fn jarring_words() {
        let solution = JarringWords::solve_once(test_date()).expect("did not find solution");
        assert_solution(solution, 'A'..='H')
    }

    #[test]
    fn crea_makerspace() {
        let solution = CreaMakerspace::solve_once(test_date()).expect("did not find solution");
        assert_solution(solution, 'A'..='H')
    }

    #[test]
    fn tetromino() {
        let solution = Tetromino::solve_once(test_date()).expect("did not find solution");
        assert_solution(solution, 'A'..='I')
    }
}
