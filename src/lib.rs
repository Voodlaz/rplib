use pyo3::prelude::*;
use pyo3::{wrap_pyfunction,create_exception};
use pyo3::exceptions::*;
use pyo3::types::*;

create_exception!(module, MyError, Exception);

#[pyfunction]
fn arg(m: &PyAny) -> PyResult<&PyFloat> {
    m.downcast::<PyFloat>().map_err(|_| MyError::py_err(()));

    match m.downcast::<PyFloat>() {
        Err(_) => Err(TypeError::py_err("AAAA")),
        Ok(value) => value,
    };
}

#[pymodule]
fn rplib(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(arg))?;

    Ok(())
}
