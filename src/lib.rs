use pyo3::prelude::*;

pub fn func_a() -> i32 {
    1
}

pub fn func_b() -> char {
    'a'
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_version() -> PyResult<String> {
    let s = "0.0.1";
    Ok(s.to_string())
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
