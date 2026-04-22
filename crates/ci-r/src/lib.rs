use extendr_api::prelude::*;
use ci_core::strategy::{CITest, TestResult};
use ci_core::ci_tests::chi_squared::ChiSquared;
use ndarray::{ArrayView1, ArrayView2};
use anyhow;


#[extendr]
pub struct RChiSquared {
    citest: ChiSquared,
}

#[extendr]
impl RChiSquared {
    fn new(
        boolean: bool,
        significance_level: f64
    ) -> Self {
        let citest = ChiSquared::new(boolean, significance_level);
        RChiSquared{citest: citest}
    }

    fn run_test(
        &self,
        x_values: ArrayView1<f64>,
        y_values: ArrayView1<f64>,
        z: ArrayView2<f64>,
    ) -> anyhow::Result<()> {

        let result = self.citest.run_test(
            x_values.to_owned(), 
            y_values.to_owned(), 
            z.to_owned())?;
        Ok(self.test_result_to_robj(result))
    }
}

fn test_result_to_robj(r: TestResult) -> Robj {
    match r {
        TestResult::PValue(p, coef) => list!(
            kind = "pvalue",
            p_value = p,
            coefficient = coef,
        ).into(),
        TestResult::Statistic(stat, p, df) => list!(
            kind = "statistic",
            statistic = stat,
            p_value = p,
            df = df as i32,
        ).into(),
        TestResult::Boolean(b) => list!(
            kind = "boolean",
            independent = b,
        ).into(),
    }
}