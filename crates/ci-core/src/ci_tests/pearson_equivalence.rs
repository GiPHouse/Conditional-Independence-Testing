use crate::ci_tests::pearson_correlation::{wrap_result, PearsonCorrelation};
use crate::strategy::{CITest, CITestDataType, TestResult};
use libm::{atanh, sqrt};
use ndarray::{Array1, Array2, Axis};
use statrs::distribution::{ContinuousCDF, Normal};

pub struct PearsonEquivalence {
    pub boolean: bool,
    pub significance_level: f64,
    pub delta_threshold: f64,
}

impl PearsonEquivalence {
    pub fn new(boolean: bool, significance_level: f64, delta_threshold: f64) -> Self {
        Self {
            boolean,
            significance_level,
            delta_threshold,
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
        let n = x_values.len();
        let s = z.axis_iter(Axis(1)).len();

        let pearsonr = PearsonCorrelation {
            boolean: false,
            significance_level: self.significance_level,
        }
        .run_test(x_values, y_values, z);
        let statistic = match pearsonr {
            Ok(TestResult::PValue(_, statistic)) => statistic,
            Ok(_) => 0.0,
            Err(e) => return Err(e),
        };
        let rho = if statistic <= -1.0 {
            -1.0 + 1e-12
        } else if statistic >= 1.0 {
            1.0 - 1e-12
        } else {
            statistic
        };

        let coefficient = atanh(rho);
        let z_delta = atanh(self.delta_threshold);

        let std_error_factor = sqrt((n - s - 3) as f64);

        let z_score_lower = std_error_factor * (coefficient + z_delta);
        let p_value_lower = 1.0 - Normal::new(0.0, 1.0).unwrap().cdf(z_score_lower);

        let z_score_upper = std_error_factor * (coefficient - z_delta);
        let p_value_upper = Normal::new(0.0, 1.0).unwrap().cdf(z_score_upper);

        let p_value = if p_value_lower > p_value_upper {
            p_value_lower
        } else {
            p_value_upper
        };

        Ok(wrap_result(
            self.boolean,
            p_value,
            coefficient,
            self.significance_level,
        ))
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Continuous]
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn basic_test() {
        let x_vals = array![1.0, 2.0, 3.0, 4.0];
        let y_vals = array![1.0, 1.0, 2.0, 2.0];
        let empty_z = array![[]];

        let test = PearsonEquivalence {
            boolean: false,
            significance_level: 0.05,
            delta_threshold: 0.1,
        };
        let result = test.run_test(x_vals, y_vals, empty_z);

        let (p_value, statistic) = match result {
            Ok(TestResult::PValue(a, b)) => (a, b),
            _ => (0.0, 0.0),
        };

        // values taken from pgmpy
        assert!((p_value - 0.910_412_594_569_001_1).abs() < 1e-8);
        assert!((statistic - 1.443_635_475_178_810_7).abs() < 1e-8);
    }
}
