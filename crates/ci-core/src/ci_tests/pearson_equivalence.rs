use crate::strategy::{CITest, CITestDataType, TestResult};
use crate::ci_tests::pearson_correlation::{PearsonCorrelation, wrap_result};
use ndarray::{Array1, Array2, Axis};
use libm::{atanh, sqrt};
use statrs::distribution::{ContinuousCDF, Normal};

#[allow(dead_code)]
pub struct PearsonEquivalence {
    pub boolean: bool,
    pub significance_level: f64,
}

impl PearsonEquivalence {
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
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        z: Array2<f64>,
    ) -> anyhow::Result<TestResult> {
        // placeholder, need to ask about hyperparameters
        let delta_treshold = 0.1;

        let n = x_values.len() as f64;
        let s = z.axis_iter(Axis(1)).len() as f64;

        let pearsonr = PearsonCorrelation{}.run_test(x_values, y_values, z, false, significance_level);
        let statistic = match pearsonr {
            Ok(TestResult::PValue(_, statistic)) => statistic,
            Ok(_) => 0.,
            Err(e) => return Err(e)
        };
        let rho = if statistic <= -1. { 
            -0.9999999 
        } else {
            if statistic >= 1. {
                0.99999999 
            } 
            else {statistic}
        };

        let coefficient = atanh(rho);
        let z_delta = atanh(delta_treshold);

        let std_error_factor = sqrt(n - s - 3.);

        let z_score_lower = std_error_factor * (coefficient + z_delta);
        let p_value_lower = 1. - Normal::new(0.0, 0.1).unwrap().cdf(z_score_lower);

        let z_score_upper = std_error_factor * (coefficient - z_delta);
        let p_value_upper = 1. - Normal::new(0.0, 0.1).unwrap().cdf(z_score_upper);

        let p_value = if p_value_lower > p_value_upper { 
            p_value_lower 
        } else {
            p_value_upper
        };
        
        Ok(wrap_result(boolean, p_value, coefficient, significance_level))
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Continuous]
    }
}
