use itertools::iproduct;
use rayon::prelude::*;

use crate::ciphers::single_byte_xor;
use crate::heuristics::looks_like_english;

pub(crate) fn detect_single_byte_xor(lines: Vec<&[u8]>) -> Option<Vec<u8>> {
    iproduct!(lines, 0..=255)
        .par_bridge()
        .map(|(ciphertext, key)| {
            single_byte_xor(ciphertext, key)
        })
        .find_any(|plaintext| {
            looks_like_english(plaintext, 0.3).unwrap()
        })
}
