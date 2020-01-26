use std::collections::HashSet;
use std::iter::FromIterator;

use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

lazy_static! {
    static ref ETAOIN: HashSet<&'static u8> = HashSet::from_iter(b"etaoin");
    static ref SHRDLU: HashSet<&'static u8> = HashSet::from_iter(b"shrdlu");
    static ref OTHERS: HashSet<&'static u8> = HashSet::from_iter(b"bcfgjkmpqvwxyz .,");
}

fn native_score_bytes_as_english(text: &[u8]) -> f64 {
    let mut score = 0.0;

    if text.is_empty() {
        return -1.0;
    }

    for c in text {
        if ETAOIN.get(c).is_some() {
            score += 2.0;
            continue;
        } else if SHRDLU.get(c).is_some() {
            score += 1.0;
            continue;
        } else if OTHERS.get(c).is_some() {
            continue;
        } else {
            score -= 1.0;
        }
    }

    score / (2.0 * (text.len() as f64))
}

fn native_fixed_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext
        .iter()
        .zip(key.iter())
        .map(|(p, k)| p ^ k)
        .collect()
}

fn native_single_byte_xor(plaintext: &[u8], key: u8) -> Vec<u8> {
    plaintext.iter().map(|p| p ^ key).collect()
}

#[pyfunction]
fn fixed_xor(plaintext: &[u8], key: &[u8]) -> PyResult<Vec<u8>> {
    Ok(native_fixed_xor(plaintext, key))
}

#[pyfunction]
fn single_byte_xor(plaintext: &[u8], key: u8) -> PyResult<Vec<u8>> {
    Ok(native_single_byte_xor(plaintext, key))
}

#[pyfunction]
fn score_bytes_as_english(text: &[u8]) -> PyResult<f64> {
    Ok(native_score_bytes_as_english(text))
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(fixed_xor))?;
    m.add_wrapped(wrap_pyfunction!(single_byte_xor))?;
    m.add_wrapped(wrap_pyfunction!(score_bytes_as_english))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_fixed_xor() {
        let plaintext = [1, 2, 3, 4, 5, 6];
        let key = [1, 1, 1, 1, 1, 1];
        let expected = [0, 3, 2, 5, 4, 7];

        let ciphertext = native_fixed_xor(&plaintext, &key);

        assert_eq!(ciphertext, expected);
    }

    #[test]
    fn test_native_single_byte_xor() {
        let plaintext = [1, 2, 3, 4, 5, 6];
        let key = 1;
        let expected = [0, 3, 2, 5, 4, 7];

        let ciphertext = native_single_byte_xor(&plaintext, key);

        assert_eq!(ciphertext, expected);
    }
}
