use crate::strategy::{CITest, CITestDataType, TestResult};
use crate::utils::power_divergence::power_divergence;
use ndarray::{Array1, Array2};

const LOG_LIKELIHOOD_LAMBDA: f64 = 0.0;

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
            LOG_LIKELIHOOD_LAMBDA,
        )
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Discrete]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array2};

    fn unwrap_correlated(r: &TestResult) -> (f64, f64, usize) {
        match r {
            TestResult::Statistic(a, b, c) => (*a, *b, *c),
            _ => panic!("expected Correlated2"),
        }
    }

    #[test]
    fn unconditional_independent_data_is_not_rejected() {
        let t = LogLikelihood {};
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 2., 1., 2., 1., 2., 1., 2.];
        let empty = Array2::<f64>::zeros((0, 0));

        let (p, stat, dof) = unwrap_correlated(&t.run_test(empty, x, y, false, 0.05).unwrap());
        assert!(stat.abs() < 1e-9);
        assert!(p > 0.99);
        assert_eq!(dof, 1);
    }

    // Can't test perfectly dependent (zero cells -> ln(0)), use skewed table instead.
    // scipy: power_divergence([[5,1],[1,5]], lambda_=0) -> stat=5.822063320647374
    #[test]
    fn unconditional_dependent_data_is_rejected() {
        let t = LogLikelihood {};
        let x = array![1., 1., 1., 1., 1., 1., 2., 2., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 1., 2., 1., 2., 2., 2., 2., 2.];
        let empty = Array2::<f64>::zeros((0, 0));

        let (p, stat, dof) = unwrap_correlated(&t.run_test(empty, x, y, false, 0.05).unwrap());
        assert!((stat - 5.822_063_320_647_374).abs() < 1e-9, "got {stat}");
        assert!((p - 0.015_826_368_796_540_195).abs() < 1e-12, "got {p}");
        assert_eq!(dof, 1);
    }

    #[test]
    fn unconditional_boolean_rejects_dependent() {
        let t = LogLikelihood {};
        let x = array![1., 1., 1., 1., 1., 1., 2., 2., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 1., 2., 1., 2., 2., 2., 2., 2.];
        let empty = Array2::<f64>::zeros((0, 0));
        let r = t.run_test(empty, x, y, true, 0.05).unwrap();
        assert!(matches!(r, TestResult::Boolean(false)));
    }
}
