use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use today_puzzle::{bitboard::BitBoard, bitpiece::BitPiece};

pub fn bitpiece_to_bitboard(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_bitboard");
    let piece = BitPiece::new(0x33);

    for i in [0, 3, 8].iter() {
        group.bench_with_input(BenchmarkId::new("bitboard1", i), i, |b, i| {
            b.iter(|| to_bitboard1(black_box(piece), *i, *i))
        });
        group.bench_with_input(BenchmarkId::new("bitboard2", i), i, |b, i| {
            b.iter(|| to_bitboard2(black_box(piece), *i, *i))
        });
        group.bench_with_input(BenchmarkId::new("bitboard3", i), i, |b, i| {
            b.iter(|| to_bitboard3(black_box(piece), *i, *i))
        });
        group.bench_with_input(BenchmarkId::new("bitboard4", i), i, |b, i| {
            b.iter(|| to_bitboard4(black_box(piece), *i, *i))
        });
        group.bench_with_input(BenchmarkId::new("bitboard5", i), i, |b, i| {
            b.iter(|| to_bitboard5(black_box(piece), *i, *i))
        });
    }
}

criterion_group!(benches, bitpiece_to_bitboard);
criterion_main!(benches);

/// Creates an 8x8 bitboard with the piece at a specific coordinate
fn to_bitboard1(bp: BitPiece, x: usize, y: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);
    for i in 0..4 {
        let seg = (bp.0 & (0xF << (4 * i))) >> (4 * i);
        if seg != 0 {
            bb |= BitBoard::new((seg as u64) << (8 * (y + i) + x));
        }
    }
    bb
}

fn to_bitboard2(bp: BitPiece, x: usize, y: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);
    for i in 0..4 {
        let seg = (bp.0 & (0xF << (4 * i))) >> (4 * i);
        if seg != 0 {
            bb |= BitBoard::new((seg as u64) << (8 * i));
        }
    }
    BitBoard::new(bb.0 << (y * 8 + x))
}

fn to_bitboard3(bp: BitPiece, x: usize, y: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);
    for i in 0..4 {
        let seg = (bp.0 & (0xF << (4 * i))) >> (4 * i);
        bb |= BitBoard::new((seg as u64) << (8 * i));
    }
    BitBoard::new(bb.0 << (y * 8 + x))
}

fn to_bitboard4(bp: BitPiece, x: usize, y: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);
    for i in 0..4 {
        let seg = ((bp.0 & (0xF << (4 * i))) as u64) << (4 * i);
        bb |= BitBoard::new(seg);
    }
    BitBoard::new(bb.0 << (y * 8 + x))
}

/// Creates an 8x8 bitboard with the piece at a specific coordinate
fn to_bitboard5(bp: BitPiece, x: usize, y: usize) -> BitBoard {
    let val = bp.0 as u64;
    let mut bb = val & 0xF;
    bb |= (val & 0xF0) << 4;
    bb |= (val & 0xF00) << 8;
    bb |= (val & 0xF000) << 12;
    BitBoard::new(bb << (y * 8 + x))
}
