//! Times the element symbol lookup, so a claim that case folding is free can
//! be checked rather than asserted.
//!
//! `cargo run --release --example symbol_bench`

use chemistry_consts::*;
use std::time::Instant;

fn main() {
    // Every symbol, in the mixed case a real file uses, plus the uppercase the
    // lookup used to demand, plus misses.
    let mixed: Vec<String> = (1..=118u8)
        .map(|z| z.atomic_symbol().unwrap().to_string())
        .collect();
    let upper: Vec<String> = mixed.iter().map(|s| s.to_uppercase()).collect();
    let misses: Vec<String> = ["Xx", "Q", "Zzz", ""].iter().map(|s| s.to_string()).collect();

    let run = |name: &str, corpus: &[String]| {
        let rounds = 200_000;
        let mut best = f64::INFINITY;
        for _ in 0..7 {
            let start = Instant::now();
            let mut sink = 0u64;
            for _ in 0..rounds {
                for symbol in corpus {
                    sink += atomic_numbers(symbol).unwrap_or(0) as u64;
                }
            }
            std::hint::black_box(sink);
            let per = start.elapsed().as_secs_f64() / (rounds * corpus.len()) as f64;
            best = best.min(per);
        }
        println!("{name:<34} {:>6.2} ns/lookup", best * 1e9);
    };

    run("uppercase, as before", &upper);
    run("mixed case, as files write it", &mixed);
    run("misses", &misses);

    // The allocation the callers used to need to get here at all.
    let rounds = 200_000;
    let mut best = f64::INFINITY;
    for _ in 0..7 {
        let start = Instant::now();
        let mut sink = 0u64;
        for _ in 0..rounds {
            for symbol in &mixed {
                sink += symbol.to_uppercase().as_str().atomic_number().unwrap_or(0) as u64;
            }
        }
        std::hint::black_box(sink);
        let per = start.elapsed().as_secs_f64() / (rounds * mixed.len()) as f64;
        best = best.min(per);
    }
    println!("{:<34} {:>6.2} ns/lookup", "via to_uppercase() as callers did", best * 1e9);
}
