use std::path::PathBuf;
use image::DynamicImage;
use image::imageops::FilterType;
use forbild_hashing::editing::*;

fn main() {
    println!("Running example...");

    let path = PathBuf::from("./data/original/2015_Japan_Tokyo_Traffic.jpg");

    let img = import_image_from_file(&path);

    let img = color_to_grayscale(img);
    img.save("./data/original/out/01_gray.jpg").unwrap();

    let img = downsample(img);
    img.resize(100, 100, FilterType::Nearest).save("./data/original/out/02_downsampled.jpg").unwrap();

    let mut img = grayscale_to_luma(img);

    let img = mirror_by_brightest_pixel(&mut img);
    DynamicImage::ImageLuma8(img.clone()).resize(100, 100, FilterType::Nearest).save("./data/original/out/03_flipped.jpg").unwrap();

    let img = to_binary_image_by_quadrant(img.to_owned());
    DynamicImage::ImageLuma8(img).resize(100, 100, FilterType::Nearest).save("./data/original/out/04_binary.jpg").unwrap();
}