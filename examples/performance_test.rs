use std::path::PathBuf;
use std::time::Instant;
use forbild_hashing::hash::Hash;

#[cfg(not(tarpaulin_include))]
fn main() {
    let n = 1000;

    let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg".to_string());
    let now = Instant::now();

    for _ in 0..n {
        let _img = Hash::from_path(&path);
    }

    let elapsed_time = now.elapsed();
    println!("Creating {} hashes took {} seconds.", n, elapsed_time.as_secs());
    println!("{} hashes per second.", n / elapsed_time.as_secs());
}