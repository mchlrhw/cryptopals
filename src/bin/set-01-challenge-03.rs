use float_ord::FloatOrd;
use std::collections::HashMap;

use cryptopals::{
    ciphers::single_byte_xor, encoding::hex_to_bytes, heuristics::score_bytes_as_english,
};

fn main() -> anyhow::Result<()> {
    let ciphertext =
        hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let mut candidates = HashMap::new();
    for key in 0..=u8::MAX {
        let plaintext = single_byte_xor(&ciphertext, key);
        let score = score_bytes_as_english(&plaintext);

        candidates.insert(key, (plaintext, score));
    }

    let best_candidate = candidates
        .iter()
        .max_by_key(|item| FloatOrd((item.1).1))
        .unwrap();

    let plaintext = String::from_utf8_lossy(&(best_candidate.1).0);
    let key = best_candidate.0;

    println!("{key}: {plaintext}");

    Ok(())
}
