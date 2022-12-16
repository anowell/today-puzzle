#![allow(dead_code)]

use crate::variants::{CreaMakerspace, DragonFjord, JarringWords, Tetromino, Variant, Weekday};
use chrono::NaiveDateTime;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Finds the first solution for a given variant, and returns an array of piece bitmaps
pub fn solve_once(epoch_ms: i64, variant: u32) -> Result<Box<[u64]>, String> {
    let date = NaiveDateTime::from_timestamp_millis(epoch_ms)
        .unwrap()
        .date();
    let solution = match variant {
        0 => DragonFjord::solve_once(date),
        1 => JarringWords::solve_once(date),
        2 => CreaMakerspace::solve_once(date),
        3 => Tetromino::solve_once(date),
        4 => Weekday::solve_once(date),
        _ => unimplemented!("Unsupported variant"),
    };

    match solution {
        Some(s) => Ok(s
            .0
            .iter()
            .map(|bitboard| bitboard.0)
            .collect::<Vec<u64>>()
            .into_boxed_slice()),
        None => Err(format!("No solution for variant {variant} on {date}")),
    }
}

// #[wasm_bindgen]
// pub fn solve_fully(month: u32, day: u32, variant: u32) -> Board {
//     let date = NaiveDate::from_ymd_opt(2020, month, day).unwrap();
//     let solution = match variant {
//         0 => DragonFjord::solve_once(date),
//         1 => CreaMakerspace::solve_once(date),
//     };

//     solution.unwrap()
// }
