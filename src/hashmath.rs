use crate::SIZE;
use crate::hash::{Hash, SubArea};

const HASHLEN: usize = (SIZE*SIZE) as usize;

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

    for (i, value) in hash1.grayimage256.iter().enumerate() {
        let median = match hash1.get_subarea(i) {
            SubArea::TopLeft => hash1.subarea_medians[0][0],
            SubArea::TopRight => hash1.subarea_medians[1][0],
            SubArea::BottomLeft => hash1.subarea_medians[0][1],
            SubArea::BottomRight => hash1.subarea_medians[1][1],
        };

        let diff = ((median as i32) - (*value as i32)).pow(2) as f64;

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

    var_diff / var_same * 1000.0 * (hamming_distance(hash1, hash2) as f64)
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


#[cfg(test)]
mod hashmath_tests {
    use super::*;

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

    fn create_testing_hash_for_weighted_distance() -> Hash {
        let mut hash = Hash::new();

        for i in 0..HASHLEN {
            hash.grayimage256[i] = i as u8;
        }
        hash.set_subarea_medians();
        hash.set_binary_hash_from_grayimage();

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
        let hash1 = create_testing_hash_for_weighted_distance();
        let mut hash2 = hash1.clone();

        for i in 0..8 {
            hash2.grayimage256[HASHLEN-1-i] = 0;
        }
        hash2.set_subarea_medians();
        hash2.set_binary_hash_from_grayimage();

        assert_eq!(hash1.subarea_medians[0][0], 64);
        assert_eq!(hash1.subarea_medians[1][0], 72);
        assert_eq!(hash1.subarea_medians[0][1], 192);
        assert_eq!(hash1.subarea_medians[1][1], 200);

        println!("{}", hash1.to_string());
        println!("{}", hash2.to_string());

        let hdist = hamming_distance(&hash1, &hash2);
        assert_eq!(hdist, 16);

        let wdist = weighted_distance(&hash1, &hash2);
        let wdist_correct = 1000.0 * (hdist as f64) * (22552.0/16.0) / (328040.0/240.0); // This was calculated by using excel... >_>
        assert_eq!(wdist, wdist_correct);
    }

    #[test]
    fn test_hex_to_binary() {
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
        assert_eq!(hex_to_binary(&'G'), None);
    }
}