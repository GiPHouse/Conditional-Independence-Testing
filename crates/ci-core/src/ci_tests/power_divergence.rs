use crate::strategy::{CITest,TestResult};
use crate::utils::{
    partition_by::partition_by,
    contingency_test::contingency_test,
    contingency_table::{contingency_table, build_unique_value_map, contingency_table_with_categories},
};
use ndarray::{Array1, Array2, array};
use statrs::distribution::{ChiSquared, ContinuousCDF};

const SIGNIFICANCE_LEVEL: f64 = 0.05;

pub struct PowerDivergence {
    pub lambda: f64,
}

fn wrap_result(boolean: bool, p_value: f64, statistic: f64, degrees_of_freedom: usize) -> TestResult {
    if boolean {
        return TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL));
    }
    return TestResult::Correlated(Ok((p_value, statistic, degrees_of_freedom)));
}

impl CITest for PowerDivergence {
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult> {

        if conditioning_set.ncols() == 0 {
            let table = contingency_table(&x_values, &y_values);
            let (statistic, p_value, degrees_of_freedom) =
                contingency_test(&table, self.lambda);
            Ok(wrap_result(boolean, p_value, statistic, degrees_of_freedom))
        }
        else {
            let x_categories = build_global_category_map(&x_values);
            let y_categories = build_global_category_map(&y_values);

            let mut statistic = 0.0;
            let mut degrees_of_freedom = 0;
            for indices in partition_by(&conditioning_set) {
                let x_sub: Array1<f64> = indices.iter().map(|&i| x_values[i]).collect();
                let y_sub: Array1<f64> = indices.iter().map(|&i| y_values[i]).collect();
                let table = contingency_table_with_categories(
                    &x_sub, &y_sub, &x_categories, &y_categories,
                );
                let (stat, _p, dof) = contingency_test(&table, self.lambda);
                if dof == 0 { continue; }
                statistic += stat;
                degrees_of_freedom += dof;
            }
            let p_value = if degrees_of_freedom == 0 {
                1.0
            } else {
                ChiSquared::new(degrees_of_freedom as f64)?.sf(statistic)
            };
            Ok(wrap_result(boolean, p_value, statistic, degrees_of_freedom))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contingency_table() {
        // basic test
        let test1_x: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = array![1.,2.,3.,1.,1.];
        let test1_y: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = array![1.,2.,3.,1.,2.];
        let test1_expected: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> = array![[2.,1.,0.],[0.,1.,0.],[0.,0.,1.]];
        assert_eq!(test1_expected, contingency_table(&test1_x, &test1_y));

        // order independence
        let test2_x: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = array![2.,1.,1.,3.,1.];
        let test2_y: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = array![2.,1.,2.,3.,1.];
        assert_eq!(test1_expected, contingency_table(&test2_x, &test2_y));

        // single value
        let test3_x = array![1., 1., 1.];
        let test3_y = array![2., 2., 2.];
        let test3_expected = array![[3.]];
        assert_eq!(test3_expected, contingency_table(&test3_x, &test3_y));

    }

    fn unwrap_correlated(result: TestResult) -> (f64, f64, usize) {
        match result {
            TestResult::Correlated(Ok(triple)) => triple,
            other => panic!("expected Correlated(Ok), got {:?}", match other {
                TestResult::Correlated(_) => "Correlated(Err)",
                TestResult::Boolean(_) => "Boolean",
            }),
        }
    }

    fn unwrap_boolean(result: TestResult) -> bool {
        match result {
            TestResult::Boolean(Ok(b)) => b,
            _ => panic!("expected Boolean(Ok)"),
        }
    }

    // Unconditional case: X and Y are perfectly independent (uniform 2x2 table).
    // The chi-squared statistic should be 0 and the test should not reject independence.
    #[test]
    fn unconditional_independent_data_is_not_rejected() {
        let test = PowerDivergence { lambda: 1.0 };
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 2., 1., 2., 1., 2., 1., 2.];
        let empty_z = Array2::<f64>::zeros((0, 0));

        let (p_value, statistic, dof) = unwrap_correlated(
            test.run_test(empty_z.clone(), x.clone(), y.clone(), false).unwrap(),
        );
        assert!(statistic.abs() < 1e-9, "expected statistic ~0, got {}", statistic);
        assert!(p_value > 0.99, "expected p ~1, got {}", p_value);
        assert_eq!(dof, 1);

        let independent = unwrap_boolean(test.run_test(empty_z, x, y, true).unwrap());
        assert!(independent, "expected fail-to-reject (independent=true)");
    }

    // Unconditional case: X and Y are perfectly dependent (X == Y).
    // The statistic should be large and the test should reject independence.
    #[test]
    fn unconditional_dependent_data_is_rejected() {
        let test = PowerDivergence { lambda: 1.0 };
        let x = array![1., 1., 1., 1., 2., 2., 2., 2.];
        let y = array![1., 1., 1., 1., 2., 2., 2., 2.];
        let empty_z = Array2::<f64>::zeros((0, 0));

        let (p_value, statistic, _dof) = unwrap_correlated(
            test.run_test(empty_z.clone(), x.clone(), y.clone(), false).unwrap(),
        );
        assert!(statistic > 5.0, "expected large statistic, got {}", statistic);
        assert!(p_value < SIGNIFICANCE_LEVEL, "expected p < {}, got {}", SIGNIFICANCE_LEVEL, p_value);

        let independent = unwrap_boolean(test.run_test(empty_z, x, y, true).unwrap());
        assert!(!independent, "expected reject (independent=false)");
    }

    // Conditional case: within each Z stratum X and Y are independent.
    // Marginally X and Y look correlated, but conditioning on Z makes them independent.
    // Z=0: X=[1,1,2,2], Y=[1,2,1,2]  (independent)
    // Z=1: X=[1,1,2,2], Y=[1,2,1,2]  (independent)
    #[test]
    fn conditional_independence_holds_within_strata() {
        let test = PowerDivergence { lambda: 1.0 };
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 2., 1., 2., 1., 2., 1., 2.];
        let z = Array2::from_shape_vec((8, 1), vec![0., 0., 0., 0., 1., 1., 1., 1.]).unwrap();

        let (p_value, statistic, dof) = unwrap_correlated(
            test.run_test(z.clone(), x.clone(), y.clone(), false).unwrap(),
        );
        assert!(statistic.abs() < 1e-9, "expected statistic ~0, got {}", statistic);
        assert!(p_value > 0.99, "expected p ~1, got {}", p_value);
        // Two strata, each contributing dof = (2-1)*(2-1) = 1.
        assert_eq!(dof, 2);

        let independent = unwrap_boolean(test.run_test(z, x, y, true).unwrap());
        assert!(independent);
    }

    // Conditional case: within each Z stratum X and Y are perfectly dependent.
    // Z=0: X=Y=[1,1,2,2]; Z=1: X=Y=[1,1,2,2]. Should reject.
    #[test]
    fn conditional_dependence_within_strata_is_rejected() {
        let test = PowerDivergence { lambda: 1.0 };
        let x = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let y = array![1., 1., 2., 2., 1., 1., 2., 2.];
        let z = Array2::from_shape_vec((8, 1), vec![0., 0., 0., 0., 1., 1., 1., 1.]).unwrap();

        let (p_value, statistic, _dof) = unwrap_correlated(
            test.run_test(z, x, y, false).unwrap(),
        );
        assert!(statistic > 5.0, "expected large statistic, got {}", statistic);
        assert!(p_value < SIGNIFICANCE_LEVEL, "expected p < {}, got {}", SIGNIFICANCE_LEVEL, p_value);
    }
}