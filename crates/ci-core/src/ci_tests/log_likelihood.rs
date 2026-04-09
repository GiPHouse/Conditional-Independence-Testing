use crate::strategy::{CITest, CITestDataType, TestResult};
use crate::utils::power_divergence::power_divergence;
use ndarray::{Array1, Array2};

const G_TEST_LAMBDA: f64 = 0.0;

pub struct LogLikelihood {}

impl CITest for LogLikelihood {
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
        significance_level: f64,
    ) -> anyhow::Result<TestResult> {
        power_divergence(
            &conditioning_set,
            &x_values,
            &y_values,
            boolean,
            significance_level,
            G_TEST_LAMBDA,
        )
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Discrete]
    }
}
