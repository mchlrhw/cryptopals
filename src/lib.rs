use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

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

#[pymodule]
fn cryptopals(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(fixed_xor))?;
    m.add_wrapped(wrap_pyfunction!(single_byte_xor))?;

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
