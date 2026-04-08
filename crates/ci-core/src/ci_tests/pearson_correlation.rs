use crate::strategy::{CITest, TestResult};
use ndarray::{Array1, Array2};

pub struct PearsonCorrelation {
    // Object traits
}

impl CITest for PearsonCorrelation {
    fn run_test(
        &self,
        _conditioning_set: Array2<f64>,
        _x_values: Array1<f64>,
        _y_values: Array1<f64>,
        _boolean: bool,
    ) -> anyhow::Result<TestResult> {
        todo!()
    }
}
