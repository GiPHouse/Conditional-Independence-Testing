use crate::strategy::{CITest, TestResult};
use ndarray::{Array1, Array2};

pub struct GTest {
    // Object traits
}

impl CITest for GTest {
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
        significance_level: f64,
    ) -> anyhow::Result<TestResult> {
        Ok(power_divergence(conditioning_set, x_values, y_values, boolean, significance_level, LAMBDA))
    }
}
