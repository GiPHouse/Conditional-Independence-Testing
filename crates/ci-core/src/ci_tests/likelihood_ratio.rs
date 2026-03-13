use crate::strategy::CITest;
use crate::strategy::TestResult;
use scirs2_core::ndarray::Array1;
use scirs2_core::ndarray::Array2;

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
    ) -> anyhow::Result<TestResult> {
        todo!()
    }
    //Other necessary stuff
}
