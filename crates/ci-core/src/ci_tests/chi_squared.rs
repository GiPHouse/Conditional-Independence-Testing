use crate::strategy::{CITest, CITestDataType, TestResult};
use crate::utils::power_divergence::power_divergence;
use ndarray::{Array1, Array2};

const CHI_SQUARED_LAMBDA: f64 = 1.0;

pub struct ChiSquared {}

impl CITest for ChiSquared {
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
            CHI_SQUARED_LAMBDA,
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
        let t = ChiSquared {};
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 2., 1., 2., 1., 2., 1., 2.];
        let empty = Array2::<f64>::zeros((0, 0));

        let (p, stat, dof) = unwrap_correlated(&t.run_test(empty, x, y, false, 0.05).unwrap());
        assert!(stat.abs() < 1e-9, "stat should be ~0, got {stat}");
        assert!(p > 0.99);
        assert_eq!(dof, 1);
    }

    // scipy: chi2_contingency([[4,0],[0,4]], lambda_=1, correction=False) -> stat=8.0, p=0.00468
    #[test]
    fn unconditional_dependent_data_is_rejected() {
        let t = ChiSquared {};
        let x = array![1., 1., 1., 1., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 2., 2., 2., 2.];
        let empty = Array2::<f64>::zeros((0, 0));

        let (p, stat, dof) = unwrap_correlated(&t.run_test(empty, x, y, false, 0.05).unwrap());
        assert!((stat - 8.0).abs() < 1e-9, "got {stat}");
        assert!((p - 0.004_677_734_981_047_276).abs() < 1e-12, "got {p}");
        assert_eq!(dof, 1);
    }

    #[test]
    fn unconditional_boolean_mode() {
        let t = ChiSquared {};
        let empty = Array2::<f64>::zeros((0, 0));
        // independent data -> should return true (fail to reject)
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 2., 1., 2., 1., 2., 1., 2.];
        let r = t.run_test(empty.clone(), x, y, true, 0.05).unwrap();
        assert!(matches!(r, TestResult::Boolean(true)));

        // dependent data -> should return false (reject)
        let x = array![1., 1., 1., 1., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 2., 2., 2., 2.];
        let r = t.run_test(empty, x, y, true, 0.05).unwrap();
        assert!(matches!(r, TestResult::Boolean(false)));
    }
}
