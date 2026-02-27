use ci_core::registry::Registry;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass(frozen)]
pub struct PyRegistry(Arc<Registry>);

#[pymethods]
impl PyRegistry {
    #[new]
    pub fn new() -> Self {
        Self(Arc::new(Registry::new()))
    }

    fn list_all_tests(&self) -> PyResult<Vec<String>> {
        let tests = self.0.list_all_tests().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(tests.into_iter().cloned().collect())
    }

    fn get_test(&self, test_name: &str) -> PyResult<PyCITest> {
        self.0.get_test(test_name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyCITest { registry: self.0.clone(), test_name: test_name.to_string() })
}

}

#[pyclass(frozen)]
pub struct PyCITest {
    registry: Arc<Registry>,
    test_name: String
}

#[pymethods]
impl PyCITest {
    pub fn __call__(
        &self, // TODO: Add additional parameters (DataFrame, etc.) required for running the tests.
    ) -> PyResult<()> {
        let test = self.registry
            .get_test(&self.test_name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        test.run_test();
        Ok(())
    }
}