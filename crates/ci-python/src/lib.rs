//! Python bindings for the conditional independence testing library.
//!
//! Exposes CI test functions to Python via the `pyo3` framework.
//! Each CI test accepts paired observation vectors and a conditioning matrix, returning
//! a Python object whose shape depends on whether the test runs in boolean or numeric mode.

mod util;

use crate::util::test_result_to_pyobj;
use ci_core::ci_tests::{
    ChiSquared, CressieRead, FreemanTukey, LogLikelihood, ModifiedLikelihood, PearsonCorrelation,
    PearsonEquivalence,
};
use ci_core::strategy::CITest as CITestTrait;
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;


#[pyclass(frozen)]
pub struct CITest {
    inner: Box<dyn CITestTrait>,
}

#[pymethods]
impl CITest {
    /// # Errors
    ///
    /// Returns `PyValueError` if `name` does not match any known CI test.
    #[new]
    #[pyo3(signature = (name, boolean = false, significance_level = 0.05, delta_threshold = 0.1))]
    pub fn new(
        name: &str,
        boolean: bool,
        significance_level: f64,
        delta_threshold: f64,
    ) -> PyResult<Self> {
        let inner: Box<dyn CITestTrait> = match name {
            "chi_squared" => Box::new(ChiSquared::new(boolean, significance_level)),
            "log_likelihood" => Box::new(LogLikelihood::new(boolean, significance_level)),
            "cressie_read" => Box::new(CressieRead::new(boolean, significance_level)),
            "pearson_correlation" => Box::new(PearsonCorrelation::new(boolean, significance_level)),
            "freeman_tukey" => Box::new(FreemanTukey::new(boolean, significance_level)),
            "modified_likelihood" => Box::new(ModifiedLikelihood::new(boolean, significance_level)),
            "pearson_equivalence" => Box::new(PearsonEquivalence::new(
                boolean,
                significance_level,
                delta_threshold,
            )),
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown test: '{name}'"
                )))
            }
        };
        Ok(Self { inner })
    }

    /// # Errors
    ///
    /// Returns `PyRuntimeError` if the test computation fails.
    #[allow(clippy::needless_pass_by_value)]
    #[pyo3(signature = (x, y, z))]
    pub fn __call__(
        &self,
        py: Python<'_>,
        x: PyReadonlyArray1<'_, f64>,
        y: PyReadonlyArray1<'_, f64>,
        z: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<Py<PyAny>> {
        let result = self
            .inner
            .run_test(
                x.as_array().to_owned(),
                y.as_array().to_owned(),
                z.as_array().to_owned(),
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        test_result_to_pyobj(&result, py)
    }
}

#[pymodule]
fn ci_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CITest>()?;
    Ok(())
}
