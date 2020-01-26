use std::sync::mpsc::channel;

use rayon::prelude::*;

use crate::ciphers::single_byte_xor;
use crate::heuristics::looks_like_english;

pub(crate) fn detect_single_byte_xor(lines: Vec<&[u8]>) -> Option<Vec<u8>> {
    let (sender, receiver) = channel();
    lines.par_iter().for_each_with(sender, |s, c| {
        for key in 0..=255 {
            let plaintext = single_byte_xor(c, key);
            if looks_like_english(&plaintext, 0.3).unwrap() {
                s.send(plaintext).unwrap();
            }
        }
    });

    let mut res: Vec<_> = receiver.iter().collect();

    res.pop()
}
