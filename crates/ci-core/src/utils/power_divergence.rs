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
    TestResult::Correlated2((p_value, coefficient, degrees_of_freedom))
}
