mod measurements;
mod stats;

use crate::stats::compute_statistics;
use clap::Parser;
use memmap2::Mmap;
use std::fs::File;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mmap = get_memory_mapped_file(&args.file);
    let results = compute_statistics(&mmap);
    println!("{{{}}}", results.join(", "));
}

fn get_memory_mapped_file(file: &str) -> Mmap {
    let file = File::open(file).unwrap();
    unsafe { Mmap::map(&file).unwrap() }
}
