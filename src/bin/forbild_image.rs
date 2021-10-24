use forbild_hashing::{editing, parse_args_to_paths};
use forbild_hashing::editing::to_binary_image_by_quadrant;

#[cfg(not(tarpaulin_include))]
fn main() {
    let paths = parse_args_to_paths();

    let mut i = 0;
    for path in &paths {
        let img = editing::import_image_from_file(&path);
        let img = editing::color_to_grayscale(img);
        let img = editing::downsample(img);
        let mut img = editing::grayscale_to_luma(img);
        let img = editing::mirror_by_brightest_pixel(&mut img);
        let img = to_binary_image_by_quadrant(img.to_owned());

        println!("{}: {}", i, path.to_str().unwrap());

        img.save(std::env::current_dir()
            .unwrap()
            .join(format!("{}.jpg", i)))
            .expect("cant write");
        i = i + 1;
    }
}