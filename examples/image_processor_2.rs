use std::path::PathBuf;
use image::GenericImageView;
use forbild_hashing::editing::*;
use forbild_hashing::hash::Hash;
use forbild_hashing::SIZE;

fn main() {
    println!("Running example...");

    let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");

    let hash = Hash::from_path(&path);

    println!("Binary: {}", hash.to_string());
    println!("Hex: {}", hash.to_string_hex());
}