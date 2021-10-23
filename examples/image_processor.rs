use std::path::PathBuf;
use forbild_hashing::editing::*;

fn main() {
    println!("Running example...");

    let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");

    let img = import_image_from_file(&path);

    let img = color_to_grayscale(img);
    img.save("./data/original/temp/1.jpg").unwrap();

    let img = downsample(img);
    img.save("./data/original/temp/2.jpg").unwrap();

    let mut img = grayscale_to_luma(img);
    img.save("./data/original/temp/3.jpg").unwrap();

    let img = mirror_by_brightest_pixel(&mut img);
    img.save("./data/original/temp/4.jpg").unwrap();

    let img = to_binary_image_by_quadrant(img.to_owned());
    img.save("./data/original/temp/5.jpg").unwrap();
}