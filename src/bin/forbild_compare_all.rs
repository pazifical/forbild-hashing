use std::path::PathBuf;
use std::time::Instant;
use forbild_hashing::hashmath::*;
use forbild_hashing::hash::Hash;

#[cfg(not(tarpaulin_include))]
fn main() {
    let extensions: Vec<String> = vec!["jpg", "JPG", "jpeg", "JPEG", "bmp", "BMP", "gif", "GIF", "png", "PNG"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let directory: String = std::env::args().skip(1).take(1).collect();

    let mut paths = Vec::new();
    for ext in extensions {
        let glob_pattern = PathBuf::from(directory.clone())
            .join(format!("**/*.{}", ext));

        // println!("{}", glob_pattern.to_str().unwrap());

        let globpaths = glob::glob(&glob_pattern.to_str().unwrap())
            .expect("ERROR: Cannot interpret directory as a pattern.");

        for globpath in globpaths {
            match globpath {
                Ok(path) => paths.push(path),
                Err(e) => {
                    println!("ERROR: Not a valid glob pattern: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // Creating hashes for all valid photos
    println!("\nHashing all {} images...", paths.len());
    let now = Instant::now();
    let mut hashs = Vec::new();
    for path in &paths {
        let hash = Hash::from_path(path);
        hashs.push(hash);
    }
    let elapsed_time = now.elapsed();
    println!("Finished hashing after {} seconds.", elapsed_time.as_secs());

    // Comparing all hashes with each other
    println!("\nComparing all hashes with each other. That means {} comparisons...", paths.len()*paths.len());
    let now = Instant::now();
    for i in 0..hashs.len() {
        for j in (i+1)..hashs.len() {
            if i == j { continue; }

            let hdist = hamming_distance(&hashs[i], &hashs[j]);

            let wdist = weighted_distance(&hashs[i], &hashs[j]);

            println!("Hamming distance: {}\t\t\tWeighted distance: {}", hdist, wdist);
            // println!("{};{};{}",
            //          paths[i].to_str().unwrap(),
            //          paths[j].to_str().unwrap(),
            //          hamming_distance(&hashs[i], &hashs[j])
            // );
        }
    }
    let elapsed_time = now.elapsed();
    println!("Comparison took {} seconds.", elapsed_time.as_secs());
}
