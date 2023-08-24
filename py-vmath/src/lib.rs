mod pyvec2;
use pyo3::prelude::*;

use crate::pyvec2::PyVec2;

/// Low-level bindings to the `vmath` Rust library
#[pymodule]
fn vmath_internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyVec2>().unwrap();
    Ok(())
}
