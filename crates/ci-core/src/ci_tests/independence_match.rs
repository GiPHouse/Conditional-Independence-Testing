use crate::strategy::CITest;
use crate::strategy::TestResult;
use ndarray::{Array1, Array2};

pub struct IndependenceMatch {
    // Object traits
}

impl CITest for IndependenceMatch {
    fn run_test(
        &self,
        _array: Array2<f64>,
        _x_values: Array1<f64>,
        _y_values: Array1<f64>,
        _boolean: bool,
    ) -> anyhow::Result<TestResult> {
        todo!()
    }
    //Other necessary stuff
}
