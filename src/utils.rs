use itertools::iproduct;
use rayon::prelude::*;

use crate::ciphers::single_byte_xor;
use crate::heuristics::score_bytes_as_english;

pub(crate) fn break_single_byte_xor(lines: Vec<&[u8]>) -> (Vec<u8>, u8) {
    let mut scored: Vec<_> = iproduct!(lines, 0..=255)
        .par_bridge()
        .map(|(ciphertext, key)| (single_byte_xor(ciphertext, key), key))
        .map(|(plaintext, key)| (plaintext.clone(), key, score_bytes_as_english(&plaintext)))
        .collect();

    scored.sort_by(|(_, _, score_a), (_, _, score_b)| score_a.partial_cmp(score_b).unwrap());
    let best = scored.last().unwrap();

    (best.0.clone(), best.1)
}
