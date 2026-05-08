use crate::strategy::{CITest, CITestDataType, TestResult};
use ndarray::{Array1, Array2};

#[allow(dead_code)]
pub struct PearsonEquivalence {
    pub boolean: bool,
    pub significance_level: f64,
}

impl PearsonEquivalence {
    #[must_use]
    pub fn new(boolean: bool, significance_level: f64) -> Self {
        Self {
            boolean,
            significance_level,
        }
    }
}

impl CITest for PearsonEquivalence {
    fn run_test(
        &self,
        _x_values: Array1<f64>,
        _y_values: Array1<f64>,
        _z: Array2<f64>,
    ) -> anyhow::Result<TestResult> {
        todo!()
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Continuous]
    }
}
