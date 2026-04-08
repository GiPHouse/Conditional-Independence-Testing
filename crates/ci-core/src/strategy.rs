use ndarray::{Array1, Array2};
/// Trait defining the interface for conditional independence tests.
///
/// All statistical tests for conditional independence must implement this trait
/// to be compatible with the registry system.
pub enum TestResult {
    Correlated(anyhow::Result<(f64, f64)>),
    Boolean(anyhow::Result<bool>),
}

/// Data types that a `CITest` can be performed on.
///
/// Used by `Registry`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CITestDataType {
    Continuous,
    Discrete,
    Mixed,
}

pub trait CITest: Send + Sync {
    /// Runs a conditional independence test on the given data.
    ///
    /// # Errors
    ///
    /// Returns an error if the test computation fails (e.g., invalid input dimensions or numerical issues).
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
        significance_level: f64,
    ) -> anyhow::Result<TestResult>;

    /// Data types that a test supports.
    fn data_types(&self) -> &'static [CITestDataType];
}

