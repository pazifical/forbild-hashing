use std::path::PathBuf;
use image::GrayImage;

use crate::editing::{preprocess_image, flip_image_by_brightest_pixel};
use crate::SIZE;

const HASHLEN: usize = (SIZE*SIZE) as usize;


pub struct Hash {
    pub grayimage256: [u8; HASHLEN],
    pub binary256: [u8; HASHLEN],
    pub subarea_medians: [[u8; 2]; 2],
}

impl Hash {
    pub fn new() -> Hash {
        Hash {
            grayimage256: [0; HASHLEN],
            binary256: [0; HASHLEN],
            subarea_medians: [[0; 2]; 2],
        }
    }

    pub fn from_path(path: &PathBuf) -> Hash {
        // Processing raw image
        let mut img = preprocess_image(path);
        let img = flip_image_by_brightest_pixel(&mut img);

        Hash::from_grayimage(img)
    }

    pub fn from_grayimage(img: GrayImage) -> Hash {
        let mut hash = Hash::new();

        // Saving grayscale image to array (necessary for weighted distance calculation)
        hash.set_grayimage(img);

        // Setting the subarea medians
        hash.set_subarea_medians();
        
        // Calculating Hash from grayscale image
        hash.set_binary_hash_from_grayimage();

        hash
    }

    fn set_grayimage(&mut self, img: GrayImage) {
        for (x, y, pix) in img.enumerate_pixels() {
            self.grayimage256[(x + SIZE*y) as usize] = pix[0];
        }
    } 

    fn get_subarea(&self, i: usize) -> SubArea {
        // Subarea top and bottom left
        if (i as u32)%SIZE < SIZE/2 {
            // Subarea top left
            if i < HASHLEN/2 {
                SubArea::TopLeft
            }
            // Subarea bottom left
            else {
                SubArea::BottomLeft
            }
        }
        // Subarea top and bottom right
        else {
            // Subarea top right
            if i < HASHLEN/2 {
                SubArea::TopRight
            }
            // Subarea bottom right
            else {
                SubArea::BottomRight
            }
        }
    }

    fn set_subarea_medians(&mut self) {
        let mut top_left = Vec::with_capacity(64);
        let mut top_right = Vec::with_capacity(64);
        let mut bot_left = Vec::with_capacity(64);
        let mut bot_right = Vec::with_capacity(64);

        for (i, val) in self.grayimage256.iter().enumerate() {
            match self.get_subarea(i) {
                SubArea::TopLeft => top_left.push(*val),
                SubArea::TopRight => top_right.push(*val),
                SubArea::BottomLeft => bot_left.push(*val),
                SubArea::BottomRight => bot_right.push(*val),
            }
        }

        // Sorting each subareas grayimage values
        top_left.sort_by(|a, b| a.partial_cmp(b).unwrap());
        top_right.sort_by(|a, b| a.partial_cmp(b).unwrap());
        bot_left.sort_by(|a, b| a.partial_cmp(b).unwrap());
        bot_right.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Setting the median value
        self.subarea_medians[0][0] = top_left[32];
        self.subarea_medians[1][0] = top_right[32];
        self.subarea_medians[0][1] = bot_left[32];
        self.subarea_medians[1][1] = bot_right[32];
    }

    fn set_binary_hash_from_grayimage(&mut self) {
        for (i, val) in self.grayimage256.iter().enumerate() {
            let median = match self.get_subarea(i) {
                SubArea::TopLeft => self.subarea_medians[0][0],
                SubArea::TopRight => self.subarea_medians[1][0],
                SubArea::BottomLeft => self.subarea_medians[0][1],
                SubArea::BottomRight => self.subarea_medians[1][1],
            };
            self.binary256[i] = match *val >= median {
                true => 1,
                false => 0
            }; 
        }
    }

    pub fn binary256_to_string(&self) -> String {
        self.binary256
            .iter()
            .map(|b| b.to_string())
            .collect()
    }
}


enum SubArea {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}