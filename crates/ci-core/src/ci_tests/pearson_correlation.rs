use ndarray::Array2;

use crate::strategy::CITest;

pub struct PearsonCorrelation {
    // Object traits
}

impl CITest for PearsonCorrelation {
    fn run_test(&self, array: Array2<>) {
        
    }
    
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