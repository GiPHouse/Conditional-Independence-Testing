use ndarray::{Array2, Axis};
use statrs::distribution::{ChiSquared, ContinuousCDF};
use anyhow::{Result, bail};

pub fn contingency_test(observed: &Array2<f64>, lambda: f64) -> Result<(f64, f64, usize)> {
    let (nrows, ncols) = observed.dim();
    let row_sums = observed.sum_axis(Axis(1));
    let col_sums = observed.sum_axis(Axis(0));
    let total: f64 = row_sums.sum();

    // Check whether contingency test is applicable 
    if observed.is_empty() {
        bail!("No data; `observed` has size 0.");
    }
    if observed.iter().any(|&x| x < 0.0) {
        bail!("All values in `observed` must be nonnegative.");
    }
    if total == 0.0 {
        bail!("Total sum of observed frequencies must be > 0.");
    }

    let statistic: f64 = if lambda.abs() < 1e-12 {
        // G-test
        let mut temp_stat: f64 = 0.0;
        for i in 0..nrows {
            for j in 0..ncols {
                let temp_expected: f64 = row_sums[i] * col_sums[j] / total;
                let temp_observed = observed[[i, j]];
                if temp_expected == 0.0 {
                    bail!(
                        "Expected frequency is zero at position [{}, {}]",
                        i,
                        j
                    );
                }
                temp_stat += temp_observed * (temp_observed / temp_expected).ln();
            }
        }
        2.0 * temp_stat
    } else {
        // Cressie-Read
        let mut temp_stat: f64 = 0.0;
        for i in 0..nrows {
            for j in 0..ncols {
                let temp_expected: f64 = row_sums[i] * col_sums[j] / total;
                let temp_observed = observed[[i, j]];
                if temp_expected == 0.0 {
                    bail!(
                        "Expected frequency is zero at position [{}, {}]",
                        i,
                        j
                    );
                }
                temp_stat += temp_observed
                    * ((temp_observed / temp_expected).powf(lambda) - 1.0);
            }
        }
        (2.0 * temp_stat) / (lambda * (lambda - 1.0))
    };

    let degrees_of_freedom = if nrows < 2 || ncols < 2 {
        0
    } else {
        (nrows - 1) * (ncols - 1)
    };

    let p_value = if degrees_of_freedom == 0 {
        1.0
    } else {
        ChiSquared::new(degrees_of_freedom as f64)?.sf(statistic)
    };

    Ok((statistic, p_value, degrees_of_freedom))
}