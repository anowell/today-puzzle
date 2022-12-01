use anyhow::Result;
use bitboard::BitBoard;
use chrono::{Datelike, Days, NaiveDate as Date};
use clap::Parser;
use piece::{PieceRef, PIECES};
use std::fmt;

mod bitboard;
mod piece;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board {
    combined: BitBoard,
    pieces: [BitBoard; PIECES.len()],
    piece_count: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [BitBoard(0); PIECES.len()],
            combined: bitboard::EMPTY,
            piece_count: 0,
        }
    }

    pub fn from_month_day(month: u32, day: u32) -> Board {
        let date_bits = !BitBoard::from_date(Date::from_ymd_opt(2020, month, day).unwrap());
        Board {
            pieces: [BitBoard(0); PIECES.len()],
            combined: bitboard::EMPTY | date_bits,
            piece_count: 0,
        }
    }

    pub fn append_valid_placements(&self, piece: PieceRef, buf: &mut Vec<Board>) {
        // println!("COMBINED\n{}", self.combined);

        // FIXME: 0..6 can wrap around; really should know width/height of a given variation
        for variation in piece.variations {
            let w = variation.width();
            let h = variation.height();
            // println!("w x h = {} x {}", w, h);
            for x in 0..(9 - w) {
                for y in 0..(9 - h) {
                    let piece_bb = variation.to_bitboard(x, y);

                    if piece_bb & self.combined == BitBoard(0) {
                        // println!("PIECE_BB\n{}", piece_bb);
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
        for i in 0..PIECES.len() {
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Date to solve for (year will be ignored)
    #[arg(short, long)]
    date: Option<Date>,

    /// Count solutions for every day of the year
    #[arg(short, long)]
    all_dates: bool,

    /// Specifies with solutions to print
    #[arg(short, long, value_enum, default_value_t=Print::First)]
    print: Print,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum Print {
    First,   // no count - faster
    Summary, // first plus count
    All,     // with counts
    Count,   // only count
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(d) = args.date {
        let solutions = solve(d.month(), d.day(), args.print);
        println!(
            "{:02}-{:02} has {} solutions",
            d.month(),
            d.day(),
            solutions
        );
    }

    if args.all_dates {
        let mut d = Date::from_ymd_opt(2020, 1, 1).unwrap();
        while d.year() < 2021 {
            match args.print {
                Print::Count => {}
                _ => println!("\n**** {:02}-{:02} ****", d.month(), d.day()),
            }
            let solutions = solve(d.month(), d.day(), args.print);
            match args.print {
                Print::First => {}
                _ => println!(
                    "{:02}-{:02} has {} solutions",
                    d.month(),
                    d.day(),
                    solutions
                ),
            }
            d = d.checked_add_days(Days::new(1)).unwrap();
        }
    }

    Ok(())
}

fn solve(month: u32, day: u32, print: Print) -> u32 {
    let mut dfs = vec![Board::from_month_day(month, day)];
    let mut solutions = 0; // = Vec::new();

    while !dfs.is_empty() {
        let board = dfs.pop().unwrap();
        if board.piece_count == PIECES.len() {
            match print {
                Print::First => {
                    println!("{}\n", board);
                    return 1;
                }
                Print::Summary if solutions == 0 => println!("{}\n", board),
                Print::All => println!("{}\n", board),
                _ => {}
            }
            solutions += 1;
        } else {
            board.append_valid_placements(PIECES[board.piece_count], &mut dfs);
        }
    }
    solutions
}
