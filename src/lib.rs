pub mod encoding;

use encoding::{bytes_to_hex, hex_to_bytes};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidEncoding(#[from] encoding::Error),

    #[error("the plaintext and key lengths are mismatched")]
    InvalidKeyLength { plain: usize, key: usize },
}

type Result<T> = std::result::Result<T, Error>;

fn fixed_xor(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if plaintext.len() != key.len() {
        return Err(Error::InvalidKeyLength {
            plain: plaintext.len(),
            key: key.len(),
        });
    }

    Ok(plaintext
        .iter()
        .zip(key.iter())
        .map(|(p, k)| p ^ k)
        .collect())
}

pub fn fixed_xor_hex(plaintext: &str, key: &str) -> Result<String> {
    let plaintext_bytes = hex_to_bytes(plaintext)?;
    let key_bytes = hex_to_bytes(key)?;

    let ciphertext_bytes = fixed_xor(&plaintext_bytes, &key_bytes)?;

    Ok(bytes_to_hex(&ciphertext_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_xor_works() -> anyhow::Result<()> {
        let plaintext = "1c0111001f010100061a024b53535009181c";
        let key = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let ciphertext = fixed_xor_hex(plaintext, key)?;

        assert_eq!(ciphertext, expected);

        Ok(())
    }
}
