use crate::hashing::hex_to_binary;
use crate::SIZE;
use crate::hash::{Hash, SubArea};

const HASHLEN: usize = (SIZE*SIZE) as usize;

pub fn hamming_distance_binary_hash(
    hash1: &[u8; HASHLEN],
    hash2: &[u8; HASHLEN]
) -> usize {
    let mut dist = 0;
    for (bit1, bit2) in hash1.iter().zip(hash2) {
        if bit1 != bit2 {
            dist += 1;
        }
    }
    dist
}

pub fn hamming_distance_hex_hash(
    hash1: &[char; HASHLEN/4],
    hash2: &[char; HASHLEN/4]
) -> usize {
    let mut hashdist = 0;
    for (h1, h2) in hash1.iter().zip(hash2) {
        let bin1 = hex_to_binary(&h1);
        let bin2 = hex_to_binary(&h2);

        match (bin1, bin2) {
            (Some(b1), Some(b2)) => {
                for (a, b) in b1.iter().zip(&b2) {
                    if *a != *b { hashdist += 1; }
                }
            }
            _ => {
                eprintln!("ERROR: Either {:?} or {:?} don't contain only valid hexadecimal values.", bin1, bin2);
                std::process::exit(1);
            }
        }
    }
    hashdist
}


pub fn hamming_distance(hash1: &Hash, hash2: &Hash) -> usize {
    let mut dist = 0;
    for (bit1, bit2) in hash1.binary256.iter().zip(&hash2.binary256) {
        if *bit1 != *bit2 {
            dist += 1;
        }
    }
    dist
}

pub fn weighted_distance(hash1: &Hash, hash2: &Hash) -> f64 {
    // Get same and different indices of both hashes
    let mut is_hashbit_identical = [false; HASHLEN];
    let mut i = 0;
    let mut identical_count = 0;
    for (bit1, bit2) in hash1.binary256.iter().zip(&hash2.binary256) {
        if *bit1 == *bit2 {
            is_hashbit_identical[i] = true;
            identical_count += 1;
        } 
        i += 1;
    }

    if identical_count == HASHLEN { return 0.0; }

    // Calculate variance
    let mut var_same = 0.0;
    let mut var_diff = 0.0;
    // TODO: Implement!
    for (i, value) in hash1.grayimage256.iter().enumerate() {
        let median = match hash1.get_subarea(i) {
            SubArea::TopLeft => hash1.subarea_medians[0][0],
            SubArea::TopRight => hash1.subarea_medians[1][0],
            SubArea::BottomLeft => hash1.subarea_medians[0][1],
            SubArea::BottomRight => hash1.subarea_medians[1][1],
        };

        let diff = ((median as i32) - (*value as i32)).abs() as f64;

        match is_hashbit_identical[i] {
            true => { 
                var_same += diff;
            },
            false => {
                var_diff += diff;
            },
        }
    }

    var_same /= identical_count as f64;
    var_diff /= (HASHLEN - identical_count) as f64;

    // println!("var_same: {}, var_diff: {}", var_same, var_diff);

    var_diff / var_same * 1000.0 * (hamming_distance(hash1, hash2) as f64)
}


#[cfg(test)]
mod hashmath_tests {
    use crate::hashmath::*;
    use image::ImageBuffer;

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

    fn create_testing_hash_for_hamming_distance() -> Hash {
        let mut hash = Hash::new();
        hash.binary256 = create_testing_binary_hash();
        hash
    }

    #[test]
    fn test_hamming_distance() {
        let hash1 = create_testing_hash_for_hamming_distance();
        let mut hash2 = hash1.clone();

        assert_eq!(hamming_distance(&hash1, &hash2), 0);

        hash2.binary256[0] = 1;
        assert_eq!(hamming_distance(&hash1, &hash2), 1);

        hash2.binary256[1] = 0;
        assert_eq!(hamming_distance(&hash1, &hash2), 2);

        let mut hash3 = hash1.clone();
        for h in hash3.binary256.iter_mut() {
            if *h == 1 { *h = 0; }
            else if *h == 0 { *h = 1; }
        }
        assert_eq!(hamming_distance(&hash1, &hash3), 256);
    }

    #[test]
    fn test_weighted_distance() {
        // TODO: Implement
    }
}