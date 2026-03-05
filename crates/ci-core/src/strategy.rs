use scirs2_core::ndarray::Array1;
use scirs2_core::ndarray::Array2;
/// Trait defining the interface for conditional independence tests.
///
/// All statistical tests for conditional independence must implement this trait
/// to be compatible with the registry system.

pub enum TestResult {
    Correlated(anyhow::Result<(f64, f64)>),
    Boolean(anyhow::Result<bool>),
}

pub trait CITest: Send + Sync{
    //fn name(&self) -> &'static str;
    //fn data_types(&self) -> &'static [&'static str];
    fn run_test(
        &self,
        array: Array2<f64>,
        x_value: Array1<f64>,
        y_value: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult>;
}
