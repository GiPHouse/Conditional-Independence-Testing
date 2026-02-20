use ci_core::registry::Registry;
use pyo3::prelude::*;


#[pyclass(frozen)]
pub struct PyRegistry {
    registry: Registry,
}

impl PyRegistry  {
    pub fn new() -> Self {
        let pyregistry = Self {
            registry: Registry::new()
        };
        pyregistry
    }
}
