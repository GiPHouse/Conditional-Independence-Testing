use ci_core::{registry::Registry, strategy::CITest};
use pyo3::prelude::*;

#[pyclass(frozen, name = "_RustRegistry")]
pub struct PyRegistry(Registry);

#[pymethods]
impl PyRegistry {
    #[new]
    pub fn new() -> Self {
        Self(Registry::new())
    }

    fn get_test(&self, test_name: &str) -> PyResult<PyCITest> {
        Ok(PyCITest {
            test: self
                .0
                .get_test(test_name)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?,
        })
    }

    fn list_all(&self) -> PyResult<Vec<String>>{
        let tests = self.0.list_all_tests().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(tests.into_iter().cloned().collect())
    }
}

#[pyclass(frozen, name = "_RustCITest")]
struct PyCITest {
    test: Box<dyn CITest>,
}

#[pymethods]
impl PyCITest {
    pub fn __call__(
        &self, // TODO: Add additional parameters (DataFrame, etc.) required for running the tests.
    ) -> PyResult<()> {
        self.test.run_test();
        Ok(())
    }
}
