use std::path::PathBuf;
use std::time::Instant;
use forbild_hashing::hash::Hash;
use forbild_hashing::hashmath::{hamming_distance, weighted_distance};

#[cfg(not(tarpaulin_include))]
fn main() {
    let n = 500;
    let c = 100_000_000;

    let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg".to_string());
    let now = Instant::now();

    println!("\nCalculating hashes...");
    for _ in 0..n {
        let _img = Hash::from_path(&path);
    }

    let elapsed_time = now.elapsed();
    let elapsed_s = elapsed_time.as_millis() / 1000;
    println!("Creating {} hashes took {} seconds.", n, elapsed_s);
    println!("{} hashes per second.", n / (elapsed_s));

    println!("\nComparing hashes...");
    let hash = Hash::from_path(&path);
    let now = Instant::now();
    for _ in 0..c {
        let _hd = weighted_distance(&hash, &hash);
    }
    let elapsed_time = now.elapsed();
    let elapsed_s = elapsed_time.as_millis() / 1000;
    println!("{} hash comparisons took {} seconds..", c, elapsed_s);
    println!("{} hash comparisons per second.", c / elapsed_s);
}