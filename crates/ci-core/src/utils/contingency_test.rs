use ndarray::{Array2, Axis};
use scirs2_core::Pow;
use statrs::distribution::{ChiSquared, ContinuousCDF};

/// Test whether the row and column variables of a count table look
/// independent. Returns `(statistic, p_value, degrees_of_freedom)`:
/// the statistic grows when the data look more dependent, and a small
/// p-value means "probably not independent".
///
/// `lambda` picks a member of the Cressie-Read family: `0` is the
/// G-test, `1` is the usual chi-squared test.
pub fn contingency_test(observed: &Array2<f64>, lambda: f64) -> (f64, f64, usize) {
    let row_sums = observed.sum_axis(Axis(1));
    let col_sums = observed.sum_axis(Axis(0));
    let total: f64 = row_sums.sum();

    // Sum the per-cell contribution. Empty cells (and cells in an empty
    // row or column) contribute zero, and skipping them avoids 0/0 when
    // a table was padded to a shape shared with other groups.
    let mut stat = 0.0;
    for i in 0..observed.nrows() {
        for j in 0..observed.ncols() {
            let observed_count = observed[[i, j]];
            if observed_count == 0.0 || row_sums[i] == 0.0 || col_sums[j] == 0.0 {
                continue;
            }
            let expected_count = row_sums[i] * col_sums[j] / total;
            let ratio = observed_count / expected_count;
            stat += if lambda.abs() < 1e-12 {
                observed_count * ratio.ln()
            } else {
                observed_count * (ratio.pow(lambda) - 1.0)
            };
        }
    }
    let statistic = if lambda.abs() < 1e-12 {
        2.0 * stat
    } else {
        2.0 * stat / (lambda * (lambda + 1.0))
    };

    // Only rows and columns that actually contain data count toward the
    // degrees of freedom. With fewer than two populated rows or columns
    // there is nothing to compare, and we report no evidence against
    // independence.
    let populated_rows = row_sums.iter().filter(|&&s| s > 0.0).count();
    let populated_cols = col_sums.iter().filter(|&&s| s > 0.0).count();
    let degrees_of_freedom = if populated_rows < 2 || populated_cols < 2 {
        0
    } else {
        (populated_rows - 1) * (populated_cols - 1)
    };
    let p_value = if degrees_of_freedom == 0 {
        1.0
    } else {
        ChiSquared::new(degrees_of_freedom as f64)
            .unwrap()
            .sf(statistic)
    };

    (statistic, p_value, degrees_of_freedom)
}
