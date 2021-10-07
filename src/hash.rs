use std::path::PathBuf;
use image::GrayImage;
use crate::{step1_preprocess_image, step2_flip_image_by_brightest_pixel, step3_create_binary_hash};
use crate::editing::*;
use crate::hashing::*;

pub struct Hash {
    grayimage256: [u8; 256],
    binary256: [u8; 256],
}

impl Hash {
    pub fn new() -> Hash {
        Hash {
            grayimage256: [0; 256],
            binary256: [0; 256]
        }
    }

    pub fn from_path(&self, path: PathBuf) -> Hash {
        // Processing raw image
        let mut img = self.preprocess_image(path);
        let img = self.flip_image_by_brightest_pixel(&mut img);

        // Saving grayscale image to array (necessary for weighted distance calculation)
        let mut gray = [0; 256];
        for (x, y, pix) in img.enumerate_pixels() {
            gray[(x + 16*y) as usize] = pix[0];
        }

        // Calculating Hash from grayscale image
        let hash = self.create_binary_hash(img);

        Hash {
            grayimage256: gray,
            binary256: hash
        }
    }

    fn preprocess_image(&self, path: PathBuf) -> GrayImage {
        let img = import_image_from_file(&path);
        let img = color_to_grayscale(img);
        let img = downsample(img);
        let img = grayscale_to_luma(img);
        img
    }

    fn flip_image_by_brightest_pixel(&self, img: &mut GrayImage) -> GrayImage {
        let img = mirror_by_brightest_pixel(img);
        img.to_owned()
    }

    fn create_binary_hash(&self, img: GrayImage) -> [u8; 256] {
        let img = to_binary_image_by_quadrant(img);
        image_to_binary_hash(img)
    }
}