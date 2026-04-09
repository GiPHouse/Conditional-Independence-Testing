use crate::strategy::TestResult;
use crate::utils::contingency_table::{
    build_global_category_map, contingency_table, contingency_table_with_categories,
};
use crate::utils::contingency_test::contingency_test;
use crate::utils::partition_by::partition_by;
use ndarray::{Array1, Array2};
use statrs::distribution::{ChiSquared, ContinuousCDF};

/// Run a power-divergence based conditional independence test.
///
/// # Errors
/// Returns an error if the underlying contingency test or chi-squared
/// distribution construction fails.
pub fn power_divergence(
    conditioning_set: &Array2<f64>,
    x_values: &Array1<f64>,
    y_values: &Array1<f64>,
    boolean: bool,
    significance_level: f64,
    lambda: f64,
) -> anyhow::Result<TestResult> {
    if conditioning_set.ncols() == 0 {
        let table = contingency_table(x_values, y_values);
        let (statistic, p_value, degrees_of_freedom) = contingency_test(&table, lambda)?;
        Ok(wrap_result(
            boolean,
            p_value,
            statistic,
            degrees_of_freedom,
            significance_level,
        ))
    } else {
        let x_categories = build_global_category_map(x_values);
        let y_categories = build_global_category_map(y_values);

        let mut statistic = 0.0;
        let mut degrees_of_freedom = 0;
        for indices in partition_by(conditioning_set) {
            let x_sub: Array1<f64> = indices.iter().map(|&i| x_values[i]).collect();
            let y_sub: Array1<f64> = indices.iter().map(|&i| y_values[i]).collect();
            let table =
                contingency_table_with_categories(&x_sub, &y_sub, &x_categories, &y_categories);
            let (stat, _p, dof) = contingency_test(&table, lambda)?;
            if dof == 0 {
                continue;
            }
            statistic += stat;
            degrees_of_freedom += dof;
        }
        let p_value = if degrees_of_freedom == 0 {
            1.0
        } else {
            #[allow(clippy::cast_precision_loss)]
            ChiSquared::new(degrees_of_freedom as f64)?.sf(statistic)
        };
        Ok(wrap_result(
            boolean,
            p_value,
            statistic,
            degrees_of_freedom,
            significance_level,
        ))
    }
}

fn wrap_result(
    boolean: bool,
    p_value: f64,
    coefficient: f64,
    degrees_of_freedom: usize,
    significance_level: f64,
) -> TestResult {
    if boolean {
        return TestResult::Boolean(p_value >= significance_level);
    }
    TestResult::Statistic(p_value, coefficient, degrees_of_freedom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array2};

    const LAMBDA: f64 = 1.0;
    const SIGNIFICANCE_LEVEL: f64 = 0.05;

    fn empty_z() -> Array2<f64> {
        Array2::zeros((0, 0))
    }

    fn unwrap_statistic(r: &TestResult) -> (f64, f64, usize) {
        match r {
            TestResult::Statistic(a, b, c) => (*a, *b, *c),
            _ => panic!("expected Statistic"),
        }
    }

    // Mismatched x and y lengths — contingency_table indexes by the
    // longer array, causing an out-of-bounds panic.
    #[test]
    #[should_panic(expected = "index 2 is out of bounds")]
    fn unconditional_mismatched_lengths_panics() {
        let x = array![1., 2., 3.];
        let y = array![1., 2.];
        let _ = power_divergence(&empty_z(), &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA);
    }

    // A single observation produces a 1x1 table with dof=0.
    #[test]
    fn unconditional_single_element_has_zero_dof() {
        let x = array![1.];
        let y = array![1.];
        let result =
            power_divergence(&empty_z(), &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA).unwrap();
        let (p, stat, dof) = unwrap_statistic(&result);
        assert_eq!(dof, 0);
        assert!((p - 1.0).abs() < 1e-12);
        assert!(stat.abs() < 1e-12);
    }

    // When X has only one distinct value the table has one row → dof=0.
    #[test]
    fn unconditional_single_category_in_x_has_zero_dof() {
        let x = array![1., 1., 1., 1.];
        let y = array![1., 2., 1., 2.];
        let result =
            power_divergence(&empty_z(), &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA).unwrap();
        let (p, _stat, dof) = unwrap_statistic(&result);
        assert_eq!(dof, 0);
        assert!((p - 1.0).abs() < 1e-12);
    }

    // When Y has only one distinct value the table has one column → dof=0.
    #[test]
    fn unconditional_single_category_in_y_has_zero_dof() {
        let x = array![1., 2., 1., 2.];
        let y = array![1., 1., 1., 1.];
        let result =
            power_divergence(&empty_z(), &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA).unwrap();
        let (p, _stat, dof) = unwrap_statistic(&result);
        assert_eq!(dof, 0);
        assert!((p - 1.0).abs() < 1e-12);
    }

    // NaN in x_values becomes its own category via OrderedFloat.
    // The result is a valid (though likely meaningless) statistic, not a panic.
    #[test]
    fn unconditional_nan_in_x_does_not_panic() {
        let x = array![1., f64::NAN, 2., 1.];
        let y = array![1., 2., 1., 2.];
        let result = power_divergence(&empty_z(), &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA);
        assert!(result.is_ok());
    }

    // NaN in the conditioning set creates its own partition group.
    // Global category maps include both X categories, so a singleton
    // stratum gets a table with a zero-expected cell → error.
    #[test]
    fn conditional_nan_in_z_returns_error() {
        let x = array![1., 1., 2., 2.];
        let y = array![1., 2., 1., 2.];
        let z = Array2::from_shape_vec((4, 1), vec![0., f64::NAN, 0., 1.]).unwrap();
        let result = power_divergence(&z, &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA);
        assert!(result.is_err());
    }

    // Each Z-group has only one X value. Global category maps force a 2x2
    // table per stratum with a zero row → zero expected frequency → error.
    #[test]
    fn conditional_all_strata_single_category_returns_error() {
        let x = array![1., 1., 2., 2.];
        let y = array![1., 2., 1., 2.];
        let z = Array2::from_shape_vec((4, 1), vec![0., 0., 1., 1.]).unwrap();
        let result = power_divergence(&z, &x, &y, false, SIGNIFICANCE_LEVEL, LAMBDA);
        assert!(result.is_err());
    }
}
