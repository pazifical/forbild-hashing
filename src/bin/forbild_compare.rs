use forbild_hashing::hashmath::*;
use forbild_hashing::{parse_args_to_paths, step1_preprocess_image, step2_flip_image_by_brightest_pixel, step3_create_binary_hash};
use forbild_hashing::SIZE;

fn main() {
    let directory: String = std::env::args().skip(1).take(1).collect();

    let globpaths = glob::glob(&directory)
        .expect("ERROR: Cannot interpret directory as a pattern.");

    let mut paths = Vec::new();
    for globpath in globpaths {
        match globpath {
            Ok(path) => paths.push(path),
            Err(e) => {
                println!("ERROR: Not a valid glob pattern: {}", e);
                std::process::exit(1);
            }
        }
    }
    // println!("Pfade: {}", paths.len());

    // Creating hashes for all valid photos
    let mut hashs = Vec::new();
    for path in &paths {
        let mut img = step1_preprocess_image(path.to_owned());
        let img = step2_flip_image_by_brightest_pixel(&mut img);
        let hash = step3_create_binary_hash(img);
        hashs.push(hash);
    }

    // Comparing all hashes with each other
    for i in 0..hashs.len() {
        for j in (i+1)..hashs.len() {
            if i == j { continue; }

            println!("{};{};{}",
                     paths[i].to_str().unwrap(),
                     paths[j].to_str().unwrap(),
                     hamming_distance_binary_hash(&hashs[i], &hashs[j])
            );
        }
    }
}
