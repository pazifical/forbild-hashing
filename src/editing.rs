use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use image::io::{Reader as ImageReader, Reader};
use image::{DynamicImage, GrayImage, GenericImageView, ImageBuffer, Luma, ImageError};
use image::imageops::{Gaussian, flip_vertical_in_place, flip_horizontal_in_place};
use crate::SIZE;

pub fn preprocess_image(path: &PathBuf) -> GrayImage {
    let img = import_image_from_file(path);
    let img = color_to_grayscale(img);
    let img = downsample(img);
    let img = grayscale_to_luma(img);
    img
}

pub fn import_image_from_file(path: &Path) -> DynamicImage {
    let img_reader = match ImageReader::open(path) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("ERROR: {}.", err);
            eprintln!("Exiting program.");
            std::process::exit(1);
        }
    };

    match img_reader.decode() {
        Ok(img) => img,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            eprintln!("Exiting program.");
            std::process::exit(1);
        }
    }
}

pub fn color_to_grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}

pub fn downsample(img: DynamicImage) -> DynamicImage {
    img.resize_exact(SIZE, SIZE, Gaussian)
}

pub fn grayscale_to_luma(img: DynamicImage) -> GrayImage {
    return img.into_luma8()
}

pub fn mirror_by_brightest_pixel(img: &mut GrayImage) -> &mut GrayImage {
    let max_00 = brightest_pixel_of_quadrant(img, 0, 0);
    let max_01 = brightest_pixel_of_quadrant(img, 0, 1);
    let max_10 = brightest_pixel_of_quadrant(img, 1, 0);
    let max_11 = brightest_pixel_of_quadrant(img, 1, 1);

    // Finding out which subarea has the brightest pixel
    let mut max_index = 0;
    let mut max_value: u8 = 0;
    for (i, value) in [max_00, max_01, max_10, max_11].iter().enumerate() {
        if *value > max_value {
            max_value = *value;
            max_index = i;
        }
    }

    // Flipping the image, so that the brightest pixel is in the top left
    if max_index == 1  {
        flip_vertical_in_place(img);
    } else if max_index == 2  {
        flip_horizontal_in_place(img);
    } else if max_index == 3 {
        flip_horizontal_in_place(img);
        flip_vertical_in_place(img);
    }
    img
}

fn brightest_pixel_of_quadrant(img: &GrayImage, col: u32, row: u32) -> u8 {
    let (mut half_x, mut half_y) = img.dimensions();
    half_x /= 2;
    half_y /= 2;

    img.view(col*half_x, row*half_y, half_x, half_y)
        .pixels()
        .map(|(_x, _y, p)| p[0])
        .max()
        .unwrap() as u8
}

pub fn to_binary_image_by_quadrant(img: GrayImage) -> GrayImage {
    let (n_cols, n_rows) = img.dimensions();
    let mut img_out = ImageBuffer::new(n_cols, n_rows);
    let halfcols = n_cols/2;
    let halfrows = n_rows/2;


    for quad_col in 0..=1 {
        for quad_row in 0..=1 {
            let quadrant = img.view(
                quad_col*halfcols,
                quad_row*halfrows,
                halfcols,
                halfrows
            );

            // Finding the median
            let mut values: Vec<u8> = quadrant.pixels()
                .map(|p| (p.2[0] as u8))
                .collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = values[values.len() / 2];

            // Writing binary image (all pixel values higher than the median will be 255, lower 0
            for col_i in 0..halfcols {
                for row_i in 0..halfrows {
                    let pix = quadrant.get_pixel(col_i, row_i);

                    if pix[0] >= median {
                        img_out.put_pixel(
                            col_i + quad_col*halfcols,
                            row_i + quad_row*halfrows,
                            Luma::<u8>([255; 1])
                        );
                    }
                }
            }
        }
    }
    GrayImage::from(img_out)
}

#[cfg(test)]
mod editing_tests {
    use image::Rgba;
    use super::*;

    fn create_dynamicimage() -> DynamicImage {
        let mut img = ImageBuffer::from_pixel(
            16,
            16,
            Rgba([255; 4])
        );

        for (x, y, pix) in img.enumerate_pixels_mut() {
            pix[0] = (x + y) as u8;
            pix[1] = (x + y) as u8;
            pix[2] = (x + y) as u8;
        }

       DynamicImage::ImageRgba8(img)
    }

    fn create_grayimage() -> GrayImage {
        let mut img: ImageBuffer<Luma<u8> , Vec<u8>> = ImageBuffer::new(16, 16);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            pix[0] = (x + y) as u8;
        }

        GrayImage::from(img)
    }

    fn rgb_to_luma(r: u8, g: u8, b: u8) -> u8 {
        (0.299*(r as f32) + 0.587*(g as f32) + 0.114*(b as f32)) as u8
    }

    #[test]
    fn test_color_to_grayscale() {
        let img = create_dynamicimage();

        let img = color_to_grayscale(img);

        let pix = img.get_pixel(10, 10);
        assert_eq!(rgb_to_luma(pix[0], pix[1], pix[2]), 20);
    }

    #[test]
    fn test_grayscale_to_luma() {
        let img = create_dynamicimage();
        let img = color_to_grayscale(img);

        let img = grayscale_to_luma(img);
        assert_eq!(img.get_pixel(10, 10)[0], 20);
    }

    #[test]
    fn test_brightest_pixel_of_quadrant() {
        let img = create_grayimage();

        let bright = brightest_pixel_of_quadrant(&img, 0, 0);
        assert_eq!(bright, 7+7);

        let bright = brightest_pixel_of_quadrant(&img, 1, 0);
        assert_eq!(bright, 7+15);

        let bright = brightest_pixel_of_quadrant(&img, 0, 1);
        assert_eq!(bright, 7+15);

        let bright = brightest_pixel_of_quadrant(&img, 1, 1);
        assert_eq!(bright, 15+15);
    }

    #[test]
    fn test_mirror_by_brightest_pixel() {
        let mut img = create_grayimage();

        let img = mirror_by_brightest_pixel(&mut img);

        assert_eq!(img.get_pixel(0, 0)[0], 15+15);
    }

    #[test]
    fn test_to_binary_image() {
        let img = create_grayimage();

        let img = to_binary_image_by_quadrant(img);

        assert_eq!(img.get_pixel(0, 0)[0], 0);
        assert_eq!(img.get_pixel(3, 0)[0], 0);
        assert_eq!(img.get_pixel(7, 7)[0], 255);

        assert_eq!(img.get_pixel(8, 8)[0], 0);
        assert_eq!(img.get_pixel(15, 15)[0], 255);
    }

    #[test]
    fn test_import_image() {
        let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");
        let img = import_image_from_file(&path);

        let pix = img.get_pixel(0, 0);
        assert_eq!(pix[0], 12);
        assert_eq!(pix[1], 11);
        assert_eq!(pix[2], 42);
    }

    #[test]
    fn test_downsample() {
        let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");
        let img = import_image_from_file(&path);
        let img = downsample(img);

        let pix = img.get_pixel(0, 0);
        assert_eq!(pix[0], 39);
        assert_eq!(pix[1], 41);
        assert_eq!(pix[2], 70);
    }

    #[test]
    fn test_preprocess_image() {
        let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");
        let img = preprocess_image(&path);

        let pix = img.get_pixel(0, 0);
        assert_eq!(pix[0], 42);
    }
}