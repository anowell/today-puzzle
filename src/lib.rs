pub use board::Board;

// Lightweight allocator for smaller wasm size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod piece;
pub mod variants;
mod board;
mod bitboard;
mod bitpiece;

#[cfg(feature = "wasm")]
mod wasm;
