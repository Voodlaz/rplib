use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn some_fn() -> PyResult<usize> {
    Ok(999)
}

#[pymodule]
fn rplib(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(some_fn))?;
    Ok(())
}
