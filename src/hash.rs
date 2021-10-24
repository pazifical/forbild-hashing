use std::path::PathBuf;
use image::GrayImage;

use crate::editing::{preprocess_image, mirror_by_brightest_pixel};
use crate::hashmath::hex_to_binary;
use crate::SIZE;

const HASHLEN: usize = (SIZE*SIZE) as usize;

#[derive(Clone, Copy)]
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
        let img = mirror_by_brightest_pixel(&mut img);

        Hash::from_grayimage(img.to_owned())
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

    pub fn from_hexhash(hexhash: &[char; HASHLEN/4]) -> Hash {
        let mut binaryhash = [0; HASHLEN];

        for (i, hexval) in hexhash.iter().enumerate() {
            let binaries = hex_to_binary(hexval).unwrap();
            for (j, b) in binaries.iter().enumerate() {
                binaryhash[(i*4)+j] = *b;
            }
        }

        let mut hash = Hash::new();
        hash.binary256 = binaryhash;
        hash
    }

    fn set_grayimage(&mut self, img: GrayImage) {
        for (x, y, pix) in img.enumerate_pixels() {
            self.grayimage256[(x + SIZE*y) as usize] = pix[0];
        }
    } 

    pub fn get_subarea(&self, i: usize) -> SubArea {
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

    pub fn to_string(&self) -> String {
        self.binary256
            .iter()
            .map(|b| b.to_string())
            .collect()
    }

    pub fn to_hex(&self) -> [char; HASHLEN/4] {
        let mut hex_hash: [char; HASHLEN/4] = ['0'; HASHLEN/4];

        for i in 0..(HASHLEN/4) {
            let hexval = match self.binary256[(4*i)..(4*i+4)] {
                [0, 0, 0, 0] => Some('0'),
                [0, 0, 0, 1] => Some('1'),
                [0, 0, 1, 0] => Some('2'),
                [0, 0, 1, 1] => Some('3'),
                [0, 1, 0 ,0] => Some('4'),
                [0, 1, 0, 1] => Some('5'),
                [0, 1, 1, 0] => Some('6'),
                [0, 1, 1, 1] => Some('7'),
                [1, 0, 0, 0] => Some('8'),
                [1, 0, 0, 1] => Some('9'),
                [1, 0, 1, 0] => Some('A'),
                [1, 0, 1, 1] => Some('B'),
                [1, 1, 0, 0] => Some('C'),
                [1, 1, 0, 1] => Some('D'),
                [1, 1, 1, 0] => Some('E'),
                [1, 1, 1, 1] => Some('F'),
                _ => None
            };

            if hexval.is_some() {
            hex_hash[i] = hexval.unwrap();
            } else {
                eprintln!("ERROR: A part of the binary hash cannot be converted to hexadecimal.");
                std::process::exit(1);
            }
        }
        hex_hash
    }

    pub fn to_string_hex(&self) -> String {
        let hash = self.to_hex();
        hash.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .concat()
    }
}

#[derive(Debug)]
pub enum SubArea {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}


#[cfg(test)]
mod hash_tests {
    use image::{ImageBuffer, Luma};
    use super::*;

    fn create_grayimage() -> GrayImage {
        let mut img: ImageBuffer<Luma<u8> , Vec<u8>> = ImageBuffer::new(16, 16);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            pix[0] = (x + y) as u8;
        }

        GrayImage::from(img)
    }

    #[test]
    fn test_hash_new() {
        let hash = Hash::new();
        assert_eq!(hash.grayimage256, [0; HASHLEN]);
        assert_eq!(hash.binary256, [0; HASHLEN]);
        assert_eq!(hash.subarea_medians, [[0; 2]; 2]);
    }

    #[test]
    fn test_hash_from_path() {
        let path = PathBuf::from("./data/original/2017_China_Chongqing_Boats.jpg");
        let hash = Hash::from_path(&path);

        let correct_binary = [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        let correct_grayimage = [42, 34, 33, 34, 36, 51, 77, 99, 103, 91, 90, 91, 86, 82, 69, 53, 102, 53, 46, 39, 45, 62, 74, 80, 80, 71, 71, 76, 78, 80, 78, 62, 117, 64, 54, 50, 78, 102, 84, 71, 65, 65, 67, 69, 71, 71, 76, 54, 107, 66, 62, 68, 113, 136, 92, 72, 63, 61, 57, 58, 57, 58, 73, 45, 91, 66, 68, 85, 145, 178, 101, 73, 66, 66, 63, 59, 46, 46, 53, 31, 76, 51, 51, 81, 136, 179, 94, 74, 69, 70, 66, 59, 43, 45, 44, 29, 59, 41, 40, 68, 100, 131, 85, 82, 81, 78, 69, 60, 45, 46, 39, 31, 42, 27, 42, 61, 78, 100, 85, 88, 116, 97, 70, 60, 51, 50, 41, 35, 37, 36, 49, 68, 83, 115, 112, 115, 139, 85, 64, 57, 52, 50, 42, 37, 39, 43, 62, 103, 123, 139, 146, 121, 106, 79, 61, 49, 51, 51, 47, 46, 38, 46, 70, 111, 130, 123, 88, 74, 63, 48, 45, 53, 68, 65, 57, 63, 41, 50, 48, 62, 67, 62, 81, 79, 56, 52, 60, 70, 69, 64, 66, 53, 56, 39, 21, 25, 50, 55, 45, 47, 53, 70, 71, 68, 61, 60, 59, 29, 51, 28, 26, 41, 75, 70, 40, 44, 65, 69, 67, 56, 60, 64, 32, 15, 61, 27, 24, 33, 53, 51, 52, 65, 65, 62, 54, 43, 53, 40, 17, 16, 37, 37, 26, 22, 27, 42, 63, 60, 51, 45, 37, 38, 43, 19, 14, 16];
        let correct_subarea_medians = [[74, 52], [65, 56]];

        assert_eq!(hash.binary256, correct_binary);
        assert_eq!(hash.grayimage256, correct_grayimage);
        assert_eq!(hash.subarea_medians, correct_subarea_medians);
    }

    #[test]
    fn test_hash_from_hexhash() {
        let binaries = "0001100101011100000001111001110101010011010100101100110010110110110101101010011001110011110011101110011001000110110010110101010000011010100010110010001011000101111011101110100010110011101010101000010111010111000110001101101010010111101010101011111111111101".to_string();
        let mut binaryhash = [0; HASHLEN];

        for (i, b) in binaries.chars().enumerate() {
            if b == '1' {
                binaryhash[i] = 1;
            }
        }
        let hexes = "195C079D5352CCB6D6A673CEE646CB541A8B22C5EEE8B3AA85D718DA97AABFFD".to_string();
        let mut hexhash = ['0'; HASHLEN/4];
        for (i, h) in hexes.chars().enumerate() {
            hexhash[i] = h;
        }

        let hash = Hash::from_hexhash(&hexhash);

        assert_eq!(hash.binary256, binaryhash);
    }

    #[test]
    fn test_set_grayimage() {
        let img = create_grayimage();
        let mut hash = Hash::new();

        hash.set_grayimage(img.clone());

        assert_eq!(hash.grayimage256[0], 0);
        assert_eq!(hash.grayimage256[2], 2);
        assert_eq!(hash.grayimage256[16], 1);
        assert_eq!(hash.grayimage256[254], 15+14);
        assert_eq!(hash.grayimage256[255], 15+15);
    }

    #[test]
    fn test_get_subarea() {
        let hash = Hash::new();

        matches!(hash.get_subarea(2), SubArea::TopLeft);
        matches!(hash.get_subarea(250), SubArea::BottomRight);
        matches!(hash.get_subarea(17), SubArea::TopRight);
        matches!(hash.get_subarea(210), SubArea::BottomLeft);
    }

    #[test]
    fn test_set_subarea_medians() {
        let img = create_grayimage();
        let mut hash = Hash::from_grayimage(img.clone());

        assert_eq!(hash.subarea_medians, [[7, 15], [15, 23]]);

        hash.set_subarea_medians();

        assert_eq!(hash.subarea_medians, [[7, 15], [15, 23]]);
    }

    #[test]
    fn test_set_binary_hash_from_grayimage() {
        let img = create_grayimage();
        let mut hash = Hash::from_grayimage(img);
        let correct_binary = [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        hash.set_binary_hash_from_grayimage();

        assert_eq!(hash.binary256, correct_binary);
    }

    #[test]
    fn test_to_string() {
        let img = create_grayimage();
        let mut hash = Hash::from_grayimage(img);

        let correct_string = "0000000100000001000000110000001100000111000001110000111100001111000111110001111100111111001111110111111101111111111111111111111100000001000000010000001100000011000001110000011100001111000011110001111100011111001111110011111101111111011111111111111111111111".to_string();

        assert_eq!(hash.to_string(), correct_string);
    }

    #[test]
    fn test_to_hex() {
        let binaries = "0001100101011100000001111001110101010011010100101100110010110110110101101010011001110011110011101110011001000110110010110101010000011010100010110010001011000101111011101110100010110011101010101000010111010111000110001101101010010111101010101011111111111101".to_string();
        let mut binaryhash = [0; HASHLEN];

        for (i, b) in binaries.chars().enumerate() {
            if b == '1' {
                binaryhash[i] = 1;
            }
        }
        let hexes = "195C079D5352CCB6D6A673CEE646CB541A8B22C5EEE8B3AA85D718DA97AABFFD".to_string();
        let mut hexhash = ['0'; HASHLEN/4];
        for (i, h) in hexes.chars().enumerate() {
            hexhash[i] = h;
        }

        let mut hash = Hash::new();
        hash.binary256 = binaryhash;

        assert_eq!(hash.to_hex(), hexhash);

        assert_eq!(hash.to_string_hex(), hexes);
    }
}