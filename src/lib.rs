use std::path::PathBuf;
use image::GrayImage;

pub mod editing;
pub mod hashing;
pub mod hashmath;

pub const SIZE: u32 = 16;

pub fn step1_preprocess_image(path: PathBuf) -> GrayImage {
    let img = editing::import_image_from_file(&path);
    let img = editing::color_to_grayscale(img);
    let img = editing::downsample(img);
    let img = editing::grayscale_to_luma(img);
    img
}

pub fn step2_flip_image_by_brightest_pixel(img: &mut GrayImage) -> GrayImage {
    let img = editing::mirror_by_brightest_pixel(img);
    img.to_owned()
}

pub fn step3_create_binary_hash(img: GrayImage) -> [u8; (SIZE*SIZE) as usize] {
    let img = editing::to_binary_image_by_quadrant(img);
    hashing::image_to_binary_hash(img)
}

pub fn parse_args_to_paths() -> Vec<PathBuf> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        eprintln!("ERROR: No image paths were given. Exiting program.");
        std::process::exit(1);
    }

    // Converting arguments to Paths and checking their validity
    let mut paths= Vec::new();
    for arg in &args {
        let path = PathBuf::from(arg);
        if path.exists() {
            paths.push(path);
        } else {
            eprintln!("WARNING: No file found with path: {:?}", path);
            eprintln!("         File will be skipped.");
        }
    }
    paths
}
