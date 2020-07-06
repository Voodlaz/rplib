use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::*;
use pyo3::types::*;

#[pyfunction]
fn arg(m: &PyAny) -> PyResult<&PyFloat> {m}

#[pymodule]
fn rplib(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(arg))?;
    Ok(())
}
