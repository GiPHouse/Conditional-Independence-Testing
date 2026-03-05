use pyo3::prelude::*;

mod pyregistry;

pub use crate::pyregistry::PyRegistry;

#[pymodule]
fn _ci_python(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyRegistry>()?;
    Ok(())
}
