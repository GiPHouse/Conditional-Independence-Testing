use ndarray::{Array1, Array2};

use crate::strategy::{CITest, TestResult};

pub struct ChiSquared {
    // Object traits
}

impl CITest for ChiSquared {
    fn run_test(
        &self,
        _array: Array2<f64>,
        _x_values: Array1<f64>,
        _y_values: Array1<f64>,
        _boolean: bool,
        _significance_level: f64,
    ) -> anyhow::Result<TestResult> {
        todo!()
    }
}
