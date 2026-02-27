use crate::strategy::TestResult;
use scirs2::stats::pearsonr;
use scirs2_core::ndarray::Array1;
use scirs2_core::ndarray::Array2;
use scirs2_linalg::lstsq;

use crate::strategy::CITest;

const SIGNIFICANCE_LEVEL: f64 = 0.05;

pub struct PearsonCorrelation {
    // Object traits
}

impl CITest for PearsonCorrelation {
    fn run_test(
        &self,
        array: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult> {
        if array.len() == 0 {
            let (coef, p_value) = pearsonr(&x_values.view(), &y_values.view(), "Two-sided")?;
            if boolean {
                return Ok(TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL)));
            } else {
                return Ok(TestResult::Correlated(Ok((p_value, coef))));
            }
        } else {
            let x_coef = lstsq(&array.view(), &x_values.view(), None)?.x;
            let y_coef = lstsq(&array.view(), &y_values.view(), None)?.x;
            let residual_x = x_values - array.dot(&x_coef);
            let residual_y = y_values - array.dot(&y_coef);
            let (coef, p_value) = pearsonr(&residual_x.view(), &residual_y.view(), "two-sided")?;
            if boolean {
                return Ok(TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL)));
            } else {
                return Ok(TestResult::Correlated(Ok((p_value, coef))));
            }
        }
    }
}

#[test]
fn test_conditional_independence() -> anyhow::Result<()> {
    let test = PearsonCorrelation {};

    // Z variable
    let z = Array1::from_vec(vec![1., 2., 3., 4., 5.]);

    // X and Y both depend on Z
    let x = &z * 2.0;
    let y = &z * 3.0;

    // Conditioning matrix must be (n_samples, n_features)
    let array = z.clone().insert_axis(scirs2_core::ndarray::Axis(1));

    let result = test.run_test(array, x, y, false)?;

    match result {
        TestResult::Correlated(Ok((_, r))) => {
            // After conditioning, correlation should drop
            assert!(r.abs() < 0.01);
        }
        _ => panic!("Unexpected result type"),
    }

    Ok(())
}
//     # Step 2: If Z is empty compute a non-conditional test.
//     if len(Z) == 0: --> if array.len() == 0 {
//         coef, p_value = stats.pearsonr(data.loc[:, X], data.loc[:, Y]) --> let coef, p_value = scirs.stats.pearsonr(&x_values, &y_values, "Two-sided")?;

//     # Step 3: If Z is non-empty, use linear regression to compute residuals and test independence on it.
//     else:
//         X_coef = np.linalg.lstsq(data.loc[:, Z], data.loc[:, X], rcond=None)[0] --> let X_coef = array.least_squares(&x_values)?;
//         Y_coef = np.linalg.lstsq(data.loc[:, Z], data.loc[:, Y], rcond=None)[0] --> let Y_coef = array.least_squares(&y_values)?;

//         residual_X = data.loc[:, X] - data.loc[:, Z].dot(X_coef) --> let residual_X = x_values - array.dot(&X_coef)?;
//         residual_Y = data.loc[:, Y] - data.loc[:, Z].dot(Y_coef) --> let residual_Y = y_values - array.dot(&Y_coef)?;
//         correlation_coef, p_value = stats.pearsonr(residual_X, residual_Y) ----> let coef, p_value = scirs.stats.pearsonr(&residual_X, &residual_Y, "Two-sided")?;

//     if what_is_this_boolean:
//         return p_value >= kwargs["significance_level"]
//     else:
//         return coef, p_value
//          }
