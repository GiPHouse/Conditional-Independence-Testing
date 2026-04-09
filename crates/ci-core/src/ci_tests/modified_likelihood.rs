use crate::strategy::{CITest, CITestDataType, TestResult};
use crate::utils::power_divergence::power_divergence;
use ndarray::{Array1, Array2};

const MODIFIED_LIKELIHOOD_LAMBDA: f64 = -1.0;

pub struct ModifiedLikelihood {}

impl CITest for ModifiedLikelihood {
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
            MODIFIED_LIKELIHOOD_LAMBDA,
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
            TestResult::Correlated2(t) => *t,
            _ => panic!("expected Correlated2"),
        }
    }

    #[test]
    fn unconditional_independent_data_is_not_rejected() {
        let t = ModifiedLikelihood {};
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 2., 1., 2., 1., 2., 1., 2.];
        let empty = Array2::<f64>::zeros((0, 0));

        let (p, stat, dof) = unwrap_correlated(&t.run_test(empty, x, y, false, 0.05).unwrap());
        assert!(stat.abs() < 1e-9);
        assert!(p > 0.99);
        assert_eq!(dof, 1);
    }

    // scipy: power_divergence([[5,1],[1,5]], lambda_=-1) -> stat=7.053439978825427
    #[test]
    fn unconditional_dependent_data_is_rejected() {
        let t = ModifiedLikelihood {};
        let x = array![1., 1., 1., 1., 1., 1., 2., 2., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 1., 2., 1., 2., 2., 2., 2., 2.];
        let empty = Array2::<f64>::zeros((0, 0));

        let (p, stat, dof) = unwrap_correlated(&t.run_test(empty, x, y, false, 0.05).unwrap());
        assert!((stat - 7.053_439_978_825_427).abs() < 1e-9, "got {stat}");
        assert!((p - 0.007_911_317_670_556_329).abs() < 1e-12, "got {p}");
        assert_eq!(dof, 1);
    }

    #[test]
    fn unconditional_boolean_rejects_dependent() {
        let t = ModifiedLikelihood {};
        let x = array![1., 1., 1., 1., 1., 1., 2., 2., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 1., 2., 1., 2., 2., 2., 2., 2.];
        let empty = Array2::<f64>::zeros((0, 0));
        let r = t.run_test(empty, x, y, true, 0.05).unwrap();
        assert!(matches!(r, TestResult::Boolean(false)));
    }
}
