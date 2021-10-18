use std::path::PathBuf;
use crate::editing::{preprocess_image, flip_image_by_brightest_pixel};
use crate::hashing::{create_binary_hash};

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

    pub fn from_path(path: &PathBuf) -> Hash {
        // Processing raw image
        let mut img = preprocess_image(path);
        let img = flip_image_by_brightest_pixel(&mut img);

        // Saving grayscale image to array (necessary for weighted distance calculation)
        let mut gray = [0; 256];
        for (x, y, pix) in img.enumerate_pixels() {
            gray[(x + 16*y) as usize] = pix[0];
        }

        // Calculating Hash from grayscale image
        let hash = create_binary_hash(img);

        Hash {
            grayimage256: gray,
            binary256: hash
        }
    }

    pub fn binary256_to_string(&self) -> String {
        self.binary256
            .iter()
            .map(|b| b.to_string())
            .collect()
    }

    pub fn hamming_distance(&self, other: &Hash) -> usize {
        let mut dist = 0;
        for (bit1, bit2) in self.binary256.iter().zip(&other.binary256) {
            if *bit1 != *bit2 {
                dist += 1;
            }
        }
        dist
    }

    // TODO: Implement weighted distance calculation
    pub fn weighted_distance(&self, other: &Hash) -> usize {
        // Get same and different indices of both hashes
        let mut same_indices = Vec::new();
        let mut diff_indices = Vec::new();
        let mut i = 0;
        for (bit1, bit2) in self.binary256.iter().zip(&other.binary256) {
            if *bit1 == *bit2 {
                same_indices.push(i);
            } else {
                diff_indices.push(i);
            }
            i += 1;
        }

        // Calculate variance
        let medians = self.calc_subarea_medians();
        let mut var_same = 0.0;
        let mut var_diff = 0.0;
        // TODO: Implement!
        for i in same_indices {
            var_same += 1;
        }
        for i in diff_indices {
            var_diff += 1;
        }

        var_same / var_diff * self.hamming_distance(&other) * 1000
    }

    // TODO: Implement!
    fn calc_subarea_medians(&self) -> [f32; 4] {
        [10.; 4]
    }

}