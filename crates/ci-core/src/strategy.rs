use ndarray::{Array1, Array2};

pub enum TestResult {
    Correlated(anyhow::Result<(f64, f64, usize)>),
    Boolean(anyhow::Result<bool>),
}

pub trait CITest {
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult>;
}
