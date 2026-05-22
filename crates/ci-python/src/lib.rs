//! Python bindings for the conditional independence testing library.
//!
//! Exposes CI test functions to Python via the `pyo3` framework.
//! Each CI test accepts paired observation vectors and a conditioning matrix, returning
//! a Python object whose shape depends on whether the test runs in boolean or numeric mode.
mod util;
use pyo3_stub_gen::define_stub_info_gatherer;
mod ci_tests_init;

#[pyo3::pymodule]
mod _ci_python {
    use ci_core::strategy::TestResult;
    use ndarray::{Array1, Array2};
    use numpy::{PyReadonlyArray1, PyReadonlyArray2};
    use pyo3::prelude::*;
    use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
    use std::sync::Arc;

    use crate::ci_tests_init;

    include!(concat!(env!("OUT_DIR"), "/ci_tests.rs"));

    // #[gen_stub_pyclass]
    // #[pyclass]
    // struct PyPearsonCorrelation {
    //     inner: ci_core::ci_tests::PearsonCorrelation,
    // }

    // #[gen_stub_pymethods]
    // #[pymethods]
    // impl PyPearsonCorrelation {
    //     #[new]
    //     fn new(significance_level: f64) -> Self {
    //         Self{inner: ci_core::ci_tests::PearsonCorrelation{
    //             boolean: true,
    //             significance_level: significance_level,
    //         }}
    //         }

    //     #[getter]
    //     fn significance_level(&self) -> PyResult<f64> {
    //         Ok(self.inner.significance_level)
    //     }

    //     #[setter]
    //     fn set_significance_level(&mut self, significance_level: f64) -> PyResult<()> {
    //         self.inner.significance_level = significance_level;
    //         Ok(())
    //     }
    // }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        ci_tests_init::init(m)
    }
}

define_stub_info_gatherer!(stub_info);
