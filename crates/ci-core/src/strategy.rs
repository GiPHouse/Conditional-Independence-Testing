use ndarray::{Array1, Array2};
/// Trait defining the interface for conditional independence tests.
///
/// All statistical tests for conditional independence must implement this trait
/// to be compatible with the registry system.
pub enum TestResult {
    Correlated(anyhow::Result<(f64, f64)>),
    Boolean(anyhow::Result<bool>),
}

pub trait CITest: Send + Sync {
    /// Runs a conditional independence test on the given data.
    ///
    /// # Errors
    ///
    /// Returns an error if the test computation fails (e.g., invalid input dimensions or numerical issues).
    fn run_test(
        &self,
        array: Array2<f64>,
        x_value: Array1<f64>,
        y_value: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult>;
}
