use crate::strategy::CITest;
use crate::strategy::TestResult;
use ndarray::{Array1, Array2};

pub struct LikelihoodRatio {
    // Object traits
}

impl CITest for LikelihoodRatio {
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
    //Other necessary stuff
}
