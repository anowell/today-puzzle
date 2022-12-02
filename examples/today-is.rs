use anyhow::Result;
use chrono::{Datelike, Days, Local, NaiveDate as Date};
use clap::Parser;
use std::str::FromStr;
use today_puzzle::{solve, Variant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Date to solve in M-D format [default: today]
    #[arg(short, long)]
    date: Option<MonthDay>,

    /// Count solutions for every day of the year
    #[arg(short, long)]
    all_dates: bool,

    /// Specifies with solutions to print
    #[arg(short, long, value_enum, default_value_t=Print::First)]
    print: Print,

    /// Puzzle variant - possible values: original, hard
    #[arg(short, long, default_value_t=Variant::Original)]
    variant: Variant
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

#[derive(Clone, Copy, Debug)]
struct MonthDay(u32, u32);

impl MonthDay {
    fn today() -> MonthDay {
        let d = Local::now().date_naive();
        MonthDay(d.month(), d.day())
    }
}

impl From<Date> for MonthDay {
    fn from(d: Date) -> MonthDay {
        MonthDay(d.month(), d.day())
    }
}

impl FromStr for MonthDay {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Prepend 2020 since it's a leap year
        let s2020 = format!("2020-{}", s);
        let d = Date::parse_from_str(&s2020, "%Y-%m-%d")?;
        Ok(MonthDay(d.month(), d.day()))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.all_dates {
        let mut d = Date::from_ymd_opt(2020, 1, 1).unwrap();
        while d.year() < 2021 {
            solve_and_print(args.variant, d.into(), args.print);
            d = d.checked_add_days(Days::new(1)).unwrap();
        }
    } else {
        let d = args.date.unwrap_or_else(MonthDay::today);
        solve_and_print(args.variant, d, args.print);
    }

    Ok(())
}

fn solve_and_print(variant: Variant, MonthDay(month, day): MonthDay, print: Print) {
    match print {
        Print::Count | Print::Check => {}
        _ => println!("**** {:02}-{:02} ****", month, day),
    }

    let only_first = match print {
        Print::First | Print::Check => true,
        Print::All | Print::Count | Print::Summary => false,
    };
    let solutions = solve(variant, month, day, only_first);

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
        Print::Check if solutions.len() > 0 => println!("{:02}-{:02} has solutions", month, day),
        Print::Check => {
            println!("{:02}-{:02} has NO solutions", month, day);
            std::process::exit(0)
        }
        Print::All | Print::Summary | Print::Count => {
            println!("{:02}-{:02} has {} solutions", month, day, solutions.len())
        }
    }
}
