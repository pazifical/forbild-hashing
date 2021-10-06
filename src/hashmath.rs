use crate::hashing::hex_to_binary;
use crate::SIZE;

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


#[cfg(test)]
mod hashmath_tests {
    use crate::hashmath::*;
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
    fn test_hamming_distance_binary() {
        let hash1 = create_testing_binary_hash();
        let mut hash2 = hash1.clone();

        assert_eq!(hamming_distance_binary_hash(&hash1, &hash2), 0);

        hash2[0] = 1;
        assert_eq!(hamming_distance_binary_hash(&hash1, &hash2), 1);

        hash2[1] = 0;
        assert_eq!(hamming_distance_binary_hash(&hash1, &hash2), 2);

        let mut hash3: [u8; 256] = hash1.clone();
        for h in hash3.iter_mut() {
            if *h == 1 { *h = 0; }
            else if *h == 0 { *h = 1; }
        }
        assert_eq!(hamming_distance_binary_hash(&hash1, &hash3), 256);
    }

    #[test]
    fn test_hamming_distance_hex_hash() {
        let hash1 = create_testing_hex_hash();
        let mut hash2 = hash1.clone();

        assert_eq!(hamming_distance_hex_hash(&hash1, &hash2), 0);

        hash2[0] = 'B';
        assert_eq!(hamming_distance_hex_hash(&hash1, &hash2), 1);

        hash2[1] = '0';
        assert_eq!(hamming_distance_hex_hash(&hash1, &hash2), 4);

        hash2[2] = '3';
        assert_eq!(hamming_distance_hex_hash(&hash1, &hash2), 8);

        hash2[3] = 'F';
        assert_eq!(hamming_distance_hex_hash(&hash1, &hash2), 9);

        hash2[4] = '3';
        assert_eq!(hamming_distance_hex_hash(&hash1, &hash2), 12);
    }
}