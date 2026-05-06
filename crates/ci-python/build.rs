use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // Struct and impl definitions
    let defs_path = Path::new(&out_dir).join("ci_tests.rs");
    fs::write(
        &defs_path,
        r#"
        #[gen_stub_pyclass]
        #[pyclass(frozen, name = "ChiSquared", module = "ci_python._ci_python")]
        pub struct PyChiSquared {
            registry: Arc<Registry>,
            test_name: String,
        }

        #[gen_stub_pymethods]
        #[pymethods]
        impl PyChiSquared {
            #[new]
            #[must_use]
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self { registry: Arc::new(Registry::new()), test_name: "chi_squared".to_string() }
            }

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
        "#,
    )
    .unwrap();

    // Module registration calls – included inside the #[pymodule_init] fn
    let init_path = Path::new(&out_dir).join("ci_tests_init.rs");
    fs::write(
        &init_path,
        r#"
        use pyo3::prelude::*;

        pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            m.add_class::<super::_ci_python::PyChiSquared>()?;
            Ok(())
        }
        "#,
    )
    .unwrap();
}
