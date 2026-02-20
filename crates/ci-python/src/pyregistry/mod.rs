use ci_core::registry::Registry;
use pyo3::prelude::*;


#[pyclass(frozen)]
pub struct PyRegistry {
    registry: Registry,
}
