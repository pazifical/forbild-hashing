use std::path::PathBuf;
use image::GenericImageView;
use forbild_hashing::editing::*;
use forbild_hashing::hashing::*;
use forbild_hashing::SIZE;

fn main() {
    println!("Running example...");

    let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");

    let img = import_image_from_file(&path);

    let img = color_to_grayscale(img);
    img.save("./data/original/temp/1.jpg");

    let img = downsample(img);
    img.save("./data/original/temp/2.jpg");

    let mut img = grayscale_to_luma(img);
    img.save("./data/original/temp/3.jpg");

    let img = mirror_by_brightest_pixel(&mut img);
    img.save("./data/original/temp/4.jpg");

    let img = to_binary_image_by_quadrant(img.to_owned());
    img.save("./data/original/temp/5.jpg");

    let hash = image_to_binary_hash(img.to_owned());
    println!("{}", binary_hash_to_string(&hash));

    let hash = binary_hash_to_hex_hash(&hash);
    println!("{}", hex_hash_to_string(&hash));

}