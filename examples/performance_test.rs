use std::path::PathBuf;
use std::time::Instant;
use image::GenericImageView;
use forbild_hashing::editing::*;
use forbild_hashing::hashing::*;
use forbild_hashing::{parse_args_to_paths, step1_preprocess_image, step2_flip_image_by_brightest_pixel, step3_create_binary_hash};
use forbild_hashing::SIZE;

fn main() {
    let n = 1000;

    let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg".to_string());
    let now = Instant::now();

    for _ in 0..n {
        let mut img = step1_preprocess_image(path.to_owned());

        let img = step2_flip_image_by_brightest_pixel(&mut img);

        let _hash = step3_create_binary_hash(img);
    }

    let elapsed_time = now.elapsed();
    println!("Creating {} hashes took {} seconds.", n, elapsed_time.as_secs());
    println!("{} hashes per second.", n / elapsed_time.as_secs());
}