use anyhow::Result;
use chrono::{Datelike, Days, Local, NaiveDate as Date};
use clap::Parser;
use std::str::FromStr;
use today_puzzle::solve;

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
            solve_and_print(d.into(), args.print);
            d = d.checked_add_days(Days::new(1)).unwrap();
        }
    } else {
        let d = args.date.unwrap_or_else(MonthDay::today);
        solve_and_print(d, args.print);
    }

    Ok(())
}

fn solve_and_print(MonthDay(month, day): MonthDay, print: Print) {
    match print {
        Print::Count => {}
        _ => println!("**** {:02}-{:02} ****", month, day),
    }

    let only_first = match print {
        Print::First => true,
        _ => false,
    };
    let solutions = solve(month, day, only_first);

    for solution in &solutions {
        match print {
            Print::First | Print::Summary => {
                println!("{}", solution);
                break;
            }
            Print::All => println!("{}", solution),
            Print::Count => {}
        }
    }

    match print {
        Print::First => {}
        _ => println!("{:02}-{:02} has {} solutions", month, day, solutions.len()),
    }
}
