use pyo3_stub_gen::define_stub_info_gatherer;
mod ci_tests_init;

#[pyo3::pymodule]
mod _ci_python {
    use ci_core::registry::Registry;
    use ci_core::strategy::TestResult;
    use ndarray::{Array1, Array2};
    use numpy::{PyReadonlyArray1, PyReadonlyArray2};
    use pyo3::prelude::*;
    use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
    use std::sync::Arc;

    use crate::ci_tests_init;

    include!(concat!(env!("OUT_DIR"), "/ci_tests.rs"));

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        ci_tests_init::init(m)
    }

    #[gen_stub_pyclass]
    #[pyclass(frozen, name = "Registry", module = "ci_python._ci_python")]
    pub struct PyRegistry(Arc<Registry>);

    #[gen_stub_pymethods]
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

    #[gen_stub_pyclass]
    #[pyclass(frozen, name = "CITest", module = "ci_python._ci_python")]
    pub struct PyCITest {
        registry: Arc<Registry>,
        test_name: String,
    }

    #[gen_stub_pymethods]
    #[pymethods]
    impl PyCITest {
        /// Run the conditional independence test on the given data.
        ///
        /// # Errors
        ///
        /// Returns `PyRuntimeError` if the test lookup fails or the test itself returns an error.
        #[allow(clippy::needless_pass_by_value)]
        #[pyo3(signature = (z, x, y))]
        pub fn __call__(
            &self,
            py: Python<'_>,
            z: PyReadonlyArray2<'_, f64>,
            x: PyReadonlyArray1<'_, f64>,
            y: PyReadonlyArray1<'_, f64>,
        ) -> PyResult<Py<PyAny>> {
            let z: Array2<f64> = z.as_array().to_owned();
            let x: Array1<f64> = x.as_array().to_owned();
            let y: Array1<f64> = y.as_array().to_owned();

            let test = self
                .registry
                .get_test(&self.test_name)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            let result = test
                .run_test(x, y, z)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            match result {
                TestResult::Boolean(b) => Ok(b.into_pyobject(py)?.to_owned().into_any().unbind()),
                TestResult::PValue(p_value, coefficient) => Ok((p_value, coefficient)
                    .into_pyobject(py)?
                    .into_any()
                    .unbind()),
                TestResult::Statistic(p_value, statistic, dof) => Ok((p_value, statistic, dof)
                    .into_pyobject(py)?
                    .into_any()
                    .unbind()),
            }
        }
    }
}

define_stub_info_gatherer!(stub_info);
