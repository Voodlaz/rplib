use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, import_exception};
use pyo3::exceptions::*;
use pyo3::types::*;

#[pyfunction]
fn arg(m: &PyFloat) -> &PyFloat {m}

#[pymodule]
fn rplib(err: &PyObject, py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(arg))?;

    match err.arg(/*???*/) {} 

    Ok(())
}
