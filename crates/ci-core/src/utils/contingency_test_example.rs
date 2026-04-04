use ndarray::{Array2, Axis};
use scirs2_core::Pow;
use statrs::distribution::{ChiSquared, ContinuousCDF};

fn contingency_test(observed: &Array2<f64>, lambda: f64) -> (f64, f64, usize) {
    let (nrows, ncols) = observed.dim();
    let row_sums = observed.sum_axis(Axis(1));
    let col_sums = observed.sum_axis(Axis(0));
    let total: f64 = row_sums.sum();

    //deceded to calculate this throughout statistic computation for performance, leaving it for now for reference
    /* 
    let mut expected_frequencies = Array2::<f64>::zeros((nrows, ncols));
    for i in 0..nrows {
        for j in 0..ncols {
            expected_frequencies[[i, j]] = row_sums[i] * col_sums[j] / total;
        }
    }
    */

    // Calculate the Cressie-Read statistic, then all lambda values work right away
    let statistic: f64 = if lambda.abs() < 1e-12 {
        // Implement G-test here: https://en.wikipedia.org/wiki/G-test
        let mut tempstat: f64 = 0.0;
        for i in 0..nrows {
            for j in 0..ncols {
            tempstat = tempstat + observed[[i,j]]*((observed[[i,j]]/(row_sums[i] * col_sums[j] / total))).ln();
            }
        }
        2.0*tempstat
    } else {
        let mut tempstat: f64 = 0.0;
        for i in 0..nrows {
            for j in 0..ncols {
            tempstat = tempstat + observed[[i,j]]*(((observed[[i,j]]/(row_sums[i] * col_sums[j] / total))).pow(lambda)-1.0);
            }
        }
        (2.0*tempstat)/(lambda*(lambda+1.0))
    };

    let degrees_of_freedom = (nrows - 1) * (ncols - 1);
    let chi2_distribution = ChiSquared::new(degrees_of_freedom as f64).unwrap();
    let p_value = chi2_distribution.sf(statistic);

    (statistic, p_value, degrees_of_freedom)
}
