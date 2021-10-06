use std::path::PathBuf;
use image::GenericImageView;
use forbild_hashing::editing::*;
use forbild_hashing::hashing::*;
use forbild_hashing::{parse_args_to_paths, step1_preprocess_image, step2_flip_image_by_brightest_pixel, step3_create_binary_hash};
use forbild_hashing::SIZE;

fn main() {
    let paths = parse_args_to_paths();

    for path in &paths {
        let mut img = step1_preprocess_image(path.to_owned());

        let img = step2_flip_image_by_brightest_pixel(&mut img);

        let hash = step3_create_binary_hash(img);

        println!("{};{}", path.to_str().unwrap(), binary_hash_to_string(&hash));
    }
}