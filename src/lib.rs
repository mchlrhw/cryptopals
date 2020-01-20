use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn foo() -> PyResult<String> {
    Ok("foo".to_string())
}

#[pymodule]
fn cryptopals(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(foo))?;

    Ok(())
}
