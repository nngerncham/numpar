use pyo3::prelude::*;
mod cwslice;
mod my_util;

mod vector_ops;
use vector_ops::*;

mod matrix_ops;
use matrix_ops::*;

/// A Python module implemented in Rust.
#[pymodule]
fn numpar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dot, m)?)?;
    m.add_function(wrap_pyfunction!(norm, m)?)?;
    m.add_function(wrap_pyfunction!(outer, m)?)?;

    m.add_function(wrap_pyfunction!(trace, m)?)?;
    m.add_function(wrap_pyfunction!(transpose, m)?)?;
    m.add_function(wrap_pyfunction!(det, m)?)?;

    m.add_function(wrap_pyfunction!(matmul, m)?)?;
    m.add_function(wrap_pyfunction!(matrix_power, m)?)?;
    Ok(())
}
