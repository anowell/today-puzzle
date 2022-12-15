use anyhow::Result;
use chrono::{Datelike, Days, Local, NaiveDate, Utc};
use clap::Parser;
use std::str::FromStr;
use today_puzzle::variants::{CreaMakerspace, DragonFjord, JarringWords, Tetromino, Variant, Weekday};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Date to solve in Y-M-D or M-D format [default: today]
    #[arg(short, long)]
    date: Option<LazyDate>,

    /// Count solutions for every day of the year
    #[arg(short, long)]
    all_dates: bool,

    /// Specifies with solutions to print
    #[arg(short, long, value_enum, default_value_t=Print::First)]
    print: Print,

    /// Puzzle variant
    #[arg(short, long, value_enum, default_value_t=VariantOpt::DragonFjord)]
    variant: VariantOpt,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum Print {
    /// Display first solution, but no count (fastest)
    First,
    /// Display first solution and count of solutions
    Summary,
    /// Display all solutions and count of solutions
    All,
    /// Display only the count of solutions
    Count,
    /// Only prints indicator if solution exists (exits early if any day is unsolvable)
    Check,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum VariantOpt {
    DragonFjord,
    CreaMakerspace,
    JarringWords,
    Tetromino,
    Weekday,
}

// Date structure that we can parse as either M-D or Y-M-D
#[derive(Clone, Copy, Debug)]
struct LazyDate(NaiveDate);

impl LazyDate {
    fn today() -> LazyDate {
        let d = Local::now().date_naive();
        LazyDate(d)
    }
}

impl From<NaiveDate> for LazyDate {
    fn from(d: NaiveDate) -> LazyDate {
        LazyDate(d)
    }
}

impl FromStr for LazyDate {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(d) => d,
            Err(err) => {
                // Prepend the current year
                let year = Utc::now().year();
                let maybe_ymd = format!("{}-{}", year, s);
                NaiveDate::parse_from_str(&maybe_ymd, "%Y-%m-%d").map_err(|_| err)?
            }
        };

        Ok(LazyDate(d))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.all_dates {
        let mut d = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        while d.year() < 2021 {
            solve_and_print(args.variant, d.into(), args.print);
            d = d.checked_add_days(Days::new(1)).unwrap();
        }
    } else {
        let d = args.date.unwrap_or_else(LazyDate::today);
        solve_and_print(args.variant, d, args.print);
    }

    Ok(())
}

fn solve_and_print(variant: VariantOpt, LazyDate(date): LazyDate, print: Print) {
    match print {
        Print::Count | Print::Check => {}
        _ => println!("**** {:02}-{:02} ****", date.month(), date.day()),
    }

    let only_first = match print {
        Print::First | Print::Check => true,
        Print::All | Print::Count | Print::Summary => false,
    };

    let solutions = match variant {
        VariantOpt::DragonFjord => {
            DragonFjord::board(date).solve(&DragonFjord::pieces(), only_first)
        }
        VariantOpt::CreaMakerspace => {
            CreaMakerspace::board(date).solve(&CreaMakerspace::pieces(), only_first)
        }
        VariantOpt::JarringWords => {
            JarringWords::board(date).solve(&JarringWords::pieces(), only_first)
        }
        VariantOpt::Tetromino => Tetromino::board(date).solve(&Tetromino::pieces(), only_first),
        VariantOpt::Weekday => Weekday::board(date).solve(&Weekday::pieces(), only_first),
    };

    for solution in &solutions {
        match print {
            Print::First | Print::Summary => {
                println!("{}", solution);
                break;
            }
            Print::All => println!("{}", solution),
            Print::Count | Print::Check => {}
        }
    }

    match print {
        Print::First => {}
        Print::Check if solutions.is_empty() => {
            println!("{:02}-{:02} has NO solutions", date.month(), date.day());
            std::process::exit(0)
        }
        Print::Check => println!("{:02}-{:02} has solutions", date.month(), date.day()),
        Print::All | Print::Summary | Print::Count => {
            println!("{:02}-{:02} has {} solutions", date.month(), date.day(), solutions.len())
        }
    }
}
