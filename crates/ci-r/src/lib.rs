use extendr_api::prelude::*;
use ci_core::strategy::CITest;
use ci_core::ci_tests::chi_squared::ChiSquared;


#[extendr]
#[derive(Clone)]
pub struct RChiSquared {
    citest: Option<ChiSquared>,
}

#[extendr]
impl RChiSquared {
    fn new(
        boolean: bool,
        significance_level: f64
    ) -> Self {
        citest = ChiSquared::new(boolean, significance_level);
        RChiSquared{citest}
    }

    fn run_test(
        &self,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        z: Array2<f64>,
    ) -> anyhow::Result<TestResult> {
        self.citest?.run_test(x_values, y_values, z)
    }
}