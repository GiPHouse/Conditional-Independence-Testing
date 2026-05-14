use ci_core::strategy::TestResult;
use ndarray::{Array1, Array2};
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use ci_core::ci_tests::{
    chi_squared::ChiSquared, cressie_read::CressieRead, freeman_tukey::FreemanTukey,
    log_likelihood::LogLikelihood, modified_likelihood::ModifiedLikelihood,
    pearson_correlation::PearsonCorrelation,
};
use ci_core::strategy::{CITest, CITestDataType};

macro_rules! python_ci_test {
    ($fn_name:ident, $inner:ty) => {
        #[extendr]
        fn $fn_name(
            x_values: PyReadonlyArray1<f64>,
            y_values: PyReadonlyArray1<f64>,
            z: PyReadonlyArray2<f64>,
            boolean: bool,
            significance_level: f64,
        ) -> PyResult<Py<PyAny>> {
            let citest = <$inner>::new(boolean, significance_level);
            let result = citest.run_test(x_values.to_owned(), y_values.to_owned(), z.to_owned())?;
            Ok(util::test_result_to_pyobj(result))
        }
    };
}

fn test_result_to_pyobj(result) -> Result<pyo3::Py<pyo3::PyAny>, _>{
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

#[pyfunction]
python_ci_test!(chi_squared_test. ChiSquared);

#[pymethods]
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

        
    }
}

#[pymodule]
fn ci_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCITest>()?;
    Ok(())
}
