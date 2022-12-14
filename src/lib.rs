pub use board::Board;

// Lightweight allocator for smaller wasm size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod bitboard;
mod bitpiece;
mod board;
pub mod piece;
pub mod variants;

#[cfg(feature = "wasm")]
mod wasm;
