use crate::ciphers::single_byte_xor;
use crate::heuristics::looks_like_english;

pub(crate) fn detect_single_byte_xor(lines: Vec<&[u8]>) -> Option<Vec<u8>> {
    for ciphertext in lines {
        for key in 0..=255 {
            let plaintext = single_byte_xor(ciphertext, key);
            if looks_like_english(&plaintext, 0.3).unwrap() {
                return Some(plaintext);
            }
        }
    }

    None
}
