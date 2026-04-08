use ci_core::registry::Registry;
use ci_core::strategy::TestResult;
use ndarray::{Array1, Array2};
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass(frozen)]
pub struct PyRegistry(Arc<Registry>);

#[pymethods]
impl PyRegistry {
    #[new]
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Arc::new(Registry::new()))
    }

    fn list_all_tests(&self) -> PyResult<Vec<&str>> {
        let tests = self
            .0
            .all_tests()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(tests.collect())
    }

    fn get_test(&self, test_name: &str) -> PyResult<PyCITest> {
        self.0
            .get_test(test_name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyCITest {
            registry: self.0.clone(),
            test_name: test_name.to_string(),
        })
    }
}

#[pyclass(frozen)]
pub struct PyCITest {
    registry: Arc<Registry>,
    test_name: String,
}

#[pymethods]
impl PyCITest {
    /// Run the conditional independence test on the given data.
    ///
    /// # Errors
    ///
    /// Returns `PyRuntimeError` if the test lookup fails or the test itself returns an error.
    #[allow(clippy::needless_pass_by_value)]
    #[pyo3(signature = (array, x, y, boolean=true, significance_level=0.05))]
    pub fn __call__(
        &self,
        py: Python<'_>,
        array: PyReadonlyArray2<'_, f64>,
        x: PyReadonlyArray1<'_, f64>,
        y: PyReadonlyArray1<'_, f64>,
        boolean: bool,
        significance_level: f64,
    ) -> PyResult<Py<PyAny>> {
        let array: Array2<f64> = array.as_array().to_owned();
        let x: Array1<f64> = x.as_array().to_owned();
        let y: Array1<f64> = y.as_array().to_owned();

        let test = self
            .registry
            .get_test(&self.test_name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        let result = test
            .run_test(array, x, y, boolean, significance_level)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        match result {
            TestResult::Boolean(Ok(b)) => Ok(b.into_pyobject(py)?.to_owned().into_any().unbind()),
            TestResult::Correlated(Ok((p_value, coefficient))) => Ok((p_value, coefficient)
                .into_pyobject(py)?
                .into_any()
                .unbind()),
            TestResult::Correlated2(Ok((p_value, statistic, dof))) => Ok((p_value, statistic, dof)
                .into_pyobject(py)?
                .into_any()
                .unbind()),
            TestResult::Boolean(Err(e))
            | TestResult::Correlated(Err(e))
            | TestResult::Correlated2(Err(e)) => Err(PyErr::new::<
                pyo3::exceptions::PyRuntimeError,
                _,
            >(e.to_string())),
        }
    }
}

#[pymodule]
fn ci_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRegistry>()?;
    m.add_class::<PyCITest>()?;
    Ok(())
}
