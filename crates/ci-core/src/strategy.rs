use ndarray::{Array1, Array2};

pub enum TestResult {
    Correlated(anyhow::Result<(f64, f64, usize)>),
    Boolean(anyhow::Result<bool>),
}

pub trait CITest {
    /// Run the conditional independence test.
    ///
    /// # Errors
    /// Returns an error if the underlying statistical computation fails
    /// (for example when constructing a chi-squared distribution with
    /// invalid parameters).
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
        significance_level: f64,
    ) -> anyhow::Result<TestResult>;
}
