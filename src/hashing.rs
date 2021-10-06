use image::GrayImage;
use crate::editing::to_binary_image_by_quadrant;
use crate::SIZE;

const HASHLEN: usize = (SIZE*SIZE) as usize;

pub fn image_to_binary_hash_by_median(img: GrayImage) -> [u8; HASHLEN] {
    let img = to_binary_image_by_quadrant(img);
    image_to_binary_hash(img)
}

pub fn image_to_binary_hash(img: GrayImage) -> [u8; HASHLEN] {
    let mut hash = [0; HASHLEN];
    for (x, y, pix) in img.enumerate_pixels() {
        let p = match pix[0] {
            0 => 0,
            _ => 1
        };
        hash[(y*SIZE + x) as usize] = p;
    }
    hash
}

pub fn image_to_hex_hash(img: GrayImage) -> [char; HASHLEN/4] {
    let hash = image_to_binary_hash(img);
    binary_hash_to_hex_hash(&hash)
}

pub fn binary_hash_to_string(hash: &[u8; HASHLEN]) -> String {
    hash.iter()
        .map(|b| b.to_string())
        .collect()
}

pub fn binary_hash_to_hex_hash(binary_hash: &[u8; HASHLEN]) -> [char; HASHLEN/4] {
    let mut hex_hash: [char; HASHLEN/4] = ['0'; HASHLEN/4];

    for i in 0..(HASHLEN/4) {
        let hexval = match binary_hash[(4*i)..(4*i+4)] {
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
            eprintln!("{}", binary_hash_to_string(&binary_hash));
            std::process::exit(1);
        }
    }
    hex_hash
}

pub fn hex_to_binary(hex: &char) -> Option<[u8; 4]> {
    match hex {
        '0' => Some([0, 0, 0, 0]),
        '1' => Some([0, 0, 0, 1]),
        '2' => Some([0, 0, 1, 0]),
        '3' => Some([0, 0, 1, 1]),
        '4' => Some([0, 1, 0, 0]),
        '5' => Some([0, 1, 0, 1]),
        '6' => Some([0, 1, 1, 0]),
        '7' => Some([0, 1, 1, 1]),
        '8' => Some([1, 0, 0, 0]),
        '9' => Some([1, 0, 0, 1]),
        'A' => Some([1, 0, 1, 0]),
        'B' => Some([1, 0, 1, 1]),
        'C' => Some([1, 1, 0, 0]),
        'D' => Some([1, 1, 0, 1]),
        'E' => Some([1, 1, 1, 0]),
        'F' => Some([1, 1, 1, 1]),
        _ => None
    }
}

pub fn hex_hash_to_string(hash: &[char; HASHLEN/4]) -> String {
    hash.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .concat()
}

#[cfg(test)]
mod hashing_tests {
    use crate::hashing::*;
    use image::ImageBuffer;

    fn create_testing_hex_hash() -> [char; 64] {
        let hex_hash = [
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1',
            'A', 'B', 'C', 'D', 'E', 'F', '0', '1'
        ];
        hex_hash
    }

    fn create_testing_binary_hash() -> [u8; 256] {
        let hash = [
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1,
        ];
        hash
    }

    #[test]
    fn test_hex_to_binary_conversion() {
        assert_eq!(hex_to_binary(&'0'), Some([0, 0, 0, 0]));
        assert_eq!(hex_to_binary(&'1'), Some([0, 0, 0, 1]));
        assert_eq!(hex_to_binary(&'2'), Some([0, 0, 1, 0]));
        assert_eq!(hex_to_binary(&'3'), Some([0, 0, 1, 1]));
        assert_eq!(hex_to_binary(&'4'), Some([0, 1, 0, 0]));
        assert_eq!(hex_to_binary(&'5'), Some([0, 1, 0, 1]));
        assert_eq!(hex_to_binary(&'6'), Some([0, 1, 1, 0]));
        assert_eq!(hex_to_binary(&'7'), Some([0, 1, 1, 1]));
        assert_eq!(hex_to_binary(&'8'), Some([1, 0, 0, 0]));
        assert_eq!(hex_to_binary(&'9'), Some([1, 0, 0, 1]));
        assert_eq!(hex_to_binary(&'A'), Some([1, 0, 1, 0]));
        assert_eq!(hex_to_binary(&'B'), Some([1, 0, 1, 1]));
        assert_eq!(hex_to_binary(&'C'), Some([1, 1, 0, 0]));
        assert_eq!(hex_to_binary(&'D'), Some([1, 1, 0, 1]));
        assert_eq!(hex_to_binary(&'E'), Some([1, 1, 1, 0]));
        assert_eq!(hex_to_binary(&'F'), Some([1, 1, 1, 1]));
    }

    #[test]
    fn test_hex_hash_to_string_conversion() {
        let hash = create_testing_hex_hash();
        let correct = "ABCDEF01ABCDEF01ABCDEF01ABCDEF01ABCDEF01ABCDEF01ABCDEF01ABCDEF01".to_string();
        assert_eq!(hex_hash_to_string(&hash), correct);
    }

    #[test]
    fn test_binary_hash_to_string_conversion() {
        let hash = create_testing_binary_hash();
        let correct = "0110010110011011011001011001101101100101100110110110010110011011011001011001101101100101100110110110010110011011011001011001101101100101100110110110010110011011011001011001101101100101100110110110010110011011011001011001101101100101100110110110010110011011".to_string();
        assert_eq!(binary_hash_to_string(&hash), correct);
    }

    #[test]
    fn test_binary_to_hex_conversion() {
        let bin_hash = create_testing_binary_hash();
        let hex_hash = [
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
            '6', '5', '9', 'B', '6', '5', '9', 'B',
        ];
        assert_eq!(binary_hash_to_hex_hash(&bin_hash), hex_hash);
    }

    #[test]
    fn test_binary_image_to_binary_array() {
        use image::Luma;

        let imgbuf = ImageBuffer::from_fn(
            16,
            16,
            |x, y| Luma::<u8>([(x as u8) * (y as u8)])
        );
        let img = GrayImage::from(imgbuf);

        let hash = image_to_binary_hash(img);

        // Only first row and first col should have 0 values
        // First row
        assert_eq!(hash[0], 0);
        assert_eq!(hash[1], 0);
        assert_eq!(hash[15], 0);
        // Second row
        assert_eq!(hash[16], 0);
        assert_eq!(hash[17], 1);
        assert_eq!(hash[31], 1);
        // Third row
        assert_eq!(hash[32], 0);
        assert_eq!(hash[36], 1);
        assert_eq!(hash[47], 1);
    }
}