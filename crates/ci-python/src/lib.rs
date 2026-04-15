use pyo3::prelude::*;
use ci_core::{registry::Registry, strategy::CITest, strategy::TestResult};
use numpy::{PyReadonlyArray1, PyReadonlyArray2};

#[pyclass(frozen, name = "Registry")]
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

    fn list_all(&self) -> PyResult<Vec<String>> {
        let tests = self
            .0
            .list_all_tests()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(tests.into_iter().cloned().collect())
    }
}

#[pyclass(frozen, name = "CITest")]
struct PyCITest {
    test: Box<dyn CITest>,
}

#[pymethods]
impl PyCITest {
    pub fn __call__(
        &self,
        py: Python,
        array: PyReadonlyArray2<f64>,
        x_value: PyReadonlyArray1<f64>,
        y_value: PyReadonlyArray1<f64>,
        boolean: bool,
    ) -> PyResult<Py<PyAny>> {
        let arr = array.as_array().to_owned();
        let x = x_value.as_array().to_owned();
        let y = y_value.as_array().to_owned();

        let result = self
            .test
            .run_test(arr, x, y, boolean)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        match result {
            TestResult::Correlated(Ok(t)) => Ok(t.into_pyobject(py)?.into_any().unbind()),
            TestResult::Boolean(Ok(b)) => Ok(b.into_pyobject(py)?.to_owned().into_any().unbind()),
            TestResult::Boolean(Err(e)) | TestResult::Correlated(Err(e)) => {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    e.to_string(),
                ))
            }
        }
    }
}

#[pymodule]
fn _ci_python(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyRegistry>()?;
    m.add_class::<PyCITest>()?;
    Ok(())
}
