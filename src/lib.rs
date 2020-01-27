mod ciphers;
mod heuristics;
mod utils;

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn score_bytes_as_english(text: &[u8]) -> PyResult<f64> {
    Ok(heuristics::score_bytes_as_english(text))
}

#[pyfunction]
fn detect_single_byte_xor(lines: Vec<&[u8]>) -> PyResult<Option<Vec<u8>>> {
    Ok(utils::detect_single_byte_xor(lines))
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "fixed_xor")]
    fn fixed_xor<'py>(py: Python<'py>, plaintext: &[u8], key: &[u8]) -> PyResult<&'py PyBytes> {
        let ciphertext = ciphers::fixed_xor(plaintext, key);
        Ok(PyBytes::new(py, &ciphertext))
    }

    #[pyfn(m, "single_byte_xor")]
    fn single_byte_xor<'py>(py: Python<'py>, plaintext: &[u8], key: u8) -> PyResult<&'py PyBytes> {
        let ciphertext = ciphers::single_byte_xor(plaintext, key);
        Ok(PyBytes::new(py, &ciphertext))
    }

    #[pyfn(m, "repeating_key_xor")]
    fn repeating_key_xor<'py>(
        py: Python<'py>,
        plaintext: &[u8],
        key: &[u8],
    ) -> PyResult<&'py PyBytes> {
        let ciphertext = ciphers::repeating_key_xor(plaintext, key);
        Ok(PyBytes::new(py, &ciphertext))
    }

    m.add_wrapped(wrap_pyfunction!(score_bytes_as_english))?;

    m.add_wrapped(wrap_pyfunction!(detect_single_byte_xor))?;

    Ok(())
}
