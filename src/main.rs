mod base_structures;

use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut t = Instant::now();
    for _ in 0..1_000_000_000 {
    }
    println!("Map done in {}ms", t.elapsed().as_millis());
    t = Instant::now();
    for _ in 0..1_000_000_000 {
    }
    println!("Bare done in {}ms", t.elapsed().as_millis());
}