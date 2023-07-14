use base64::Engine;
use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{pair} is not a valid hex character pair")]
    InvalidHexChunkSize { pair: String },

    #[error(transparent)]
    InvalidHexPair(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Error>;

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    hex.chars()
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let pair: String = chunk.collect();
            if pair.len() != 2 {
                Err(Error::InvalidHexChunkSize { pair })
            } else {
                Ok(u8::from_str_radix(&pair, 16)?)
            }
        })
        .collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

fn bytes_to_base64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub fn hex_to_base64(hex: &str) -> Result<String> {
    Ok(bytes_to_base64(&hex_to_bytes(hex)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64_works() -> anyhow::Result<()> {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let base64 = hex_to_base64(hex)?;

        assert_eq!(base64, expected);

        Ok(())
    }
}
