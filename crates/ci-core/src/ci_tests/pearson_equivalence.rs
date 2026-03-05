use crate::strategy::CITest;
use crate::strategy::TestResult;
use scirs2_core::ndarray::Array1;
use scirs2_core::ndarray::Array2;

pub struct PearsonEquivalence {
    // Object traits
}

impl CITest for PearsonEquivalence {
    fn run_test(
        &self,
        array: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult> {
        todo!()
    }
    //Other necessary stuff
}
