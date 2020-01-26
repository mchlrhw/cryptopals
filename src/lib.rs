mod ciphers;
mod heuristics;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn fixed_xor(plaintext: &[u8], key: &[u8]) -> PyResult<Vec<u8>> {
    Ok(ciphers::fixed_xor(plaintext, key))
}

#[pyfunction]
fn single_byte_xor(plaintext: &[u8], key: u8) -> PyResult<Vec<u8>> {
    Ok(ciphers::single_byte_xor(plaintext, key))
}

#[pyfunction]
fn score_bytes_as_english(text: &[u8]) -> PyResult<f64> {
    Ok(heuristics::score_bytes_as_english(text))
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(fixed_xor))?;
    m.add_wrapped(wrap_pyfunction!(single_byte_xor))?;
    m.add_wrapped(wrap_pyfunction!(score_bytes_as_english))?;

    Ok(())
}
