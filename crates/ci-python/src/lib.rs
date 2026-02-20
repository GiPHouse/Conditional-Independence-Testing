use pyo3::prelude::*;

mod pyregistry;

pub use crate::pyregistry::PyRegistry;

#[pymodule]
fn ci_python(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyRegistry>()?;
    Ok(())
}
