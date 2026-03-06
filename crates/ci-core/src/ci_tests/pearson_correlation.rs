use crate::strategy::CITest;
use crate::strategy::TestResult;
use scirs2::stats::pearsonr;
use scirs2_core::ndarray::Array1;
use scirs2_core::ndarray::Array2;
use scirs2_linalg::lstsq;

const SIGNIFICANCE_LEVEL: f64 = 0.05;

///Compute Pearson correlation coefficient and p-value for testing non-correlation.
/// 
///Should be used only on continuous data. In case when :math:`Z \\neq \\emptyset` uses
///linear regression and computes pearson coefficient on residuals.
/// 
///# Parameters
///----------
///- `x_values` : Array1<f64>
///     The first variable for testing the independence condition X \u27c2 Y | Z.
/// 
///- `y_values` : Array1<f64>
///     The second variable for testing the independence condition X \u27c2 Y | Z.
/// 
///- `array` : lArray2<f64>
///     A list of conditional variables for testing the condition X \u27c2 Y | Z.
/// 
///- `boolean` : bool, default=True
///     If True, returns a boolean indicating independence (based on `significance_level`).
///     If False, returns the test statistic and p-value.
/// 
///# Returns
///-------
///- result : bool or tuple
///     If boolean=True, returns True if p-value >= significance_level, else False.
///     If boolean=False, returns a tuple of (Pearson's correlation Coefficient, p-value).
/// 
///# References
///----------
///[1] https://en.wikipedia.org/wiki/Pearson_correlation_coefficient
///
///[2] https://en.wikipedia.org/wiki/Partial_correlation#Using_linear_regression
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
        // Step 1: If array is non-empty, use linear regression to compute residuals and test independence on it.
        if array.len() == 0 {
            let (coefficient, p_value) = pearsonr(&x_values.view(), &y_values.view(), "two-sided")?;
            Ok(result(boolean, p_value, coefficient)?)
        } else {
            let x_coefficient = lstsq(&array.view(), &x_values.view(), None)?.x;
            let y_coefficient = lstsq(&array.view(), &y_values.view(), None)?.x;
            let residual_x = x_values - array.dot(&x_coefficient);
            let residual_y = y_values - array.dot(&y_coefficient);
            let (coefficient, p_value) =
                pearsonr(&residual_x.view(), &residual_y.view(), "two-sided")?;
            Ok(result(boolean, p_value, coefficient)?)
        }
    }
}

///     Compute final result
///     # Parameters
///     - `boolean` : bool, default=True
///         If True, returns a boolean indicating independence (based on `significance_level`).
///         If False, returns the test statistic and p-value.
///     - `coeeficient`: f64
///         Pearson's correlation Coefficient
///     # Returns
///     -------
///     - result : bool or tuple
///         If boolean=True, returns True if p-value >= significance_level, else False.
///         If boolean=False, returns a tuple of (Pearson's correlation Coefficient, p-value).
fn result(boolean: bool, p_value: f64, coefficient: f64) -> anyhow::Result<TestResult> {
    if boolean {
        return Ok(TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL)));
    } 
    return Ok(TestResult::Correlated(Ok((p_value, coefficient))));
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use scirs2_core::ndarray::{Array1, Array2, Axis};
    use scirs2_core::random::{rngs::SmallRng, Distribution, Normal, SeedableRng};

    const N: usize = 200; // Can't have N greater than or equal to 300 due to scirs2 bug

    fn seeded_rng() -> SmallRng {
        SmallRng::seed_from_u64(42)
    }

    fn gen_normal(n: usize, mean: f64, std_dev: f64, rng: &mut SmallRng) -> Array1<f64> {
        let dist = Normal::new(mean, std_dev).unwrap();
        Array1::from_vec((0..n).map(|_| dist.sample(rng)).collect())
    }

    fn empty_array() -> Array2<f64> {
        Array2::zeros((0, 0))
    }

    fn pearson() -> PearsonCorrelation {
        PearsonCorrelation {}
    }

    #[test]
    fn debug_pearsonr_sizes() {
        let mut rng = seeded_rng();
        for n in [200, 300, 350, 400, 450, 500] {
            let x = gen_normal(n, 0.0, 1.0, &mut rng);
            let y = gen_normal(n, 0.0, 1.0, &mut rng);
            let raw = pearsonr(&x.view(), &y.view(), "two-sided");
            eprintln!("N={}: {:?}", n, raw);
        }
    }

    // --- 1. Empty array + independent X, Y + boolean=false ---
    // X and Y are independently generated, no conditioning variables.
    // Expected: high p_value (> 0.05), low |coefficient| (< 0.1)
    #[test]
    fn test_empty_array_independent_boolean_false() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);

        let result = pearson().run_test(empty_array(), x, y, false).unwrap();
        match result {
            TestResult::Correlated(Ok((p_value, coefficient))) => {
                assert!(
                    p_value > SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be > 0.05 for independent data"
                );
                assert!(
                    coefficient.abs() < 0.1,
                    "coefficient {coefficient} should be near 0 for independent data"
                );
            }
            _ => panic!("Expected TestResult::Correlated"),
        }
    }

    // --- 2. Empty array + independent X, Y + boolean=true ---
    // Expected: true (variables are independent)
    #[test]
    fn test_empty_array_independent_boolean_true() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);

        let result = pearson().run_test(empty_array(), x, y, true).unwrap();
        match result {
            TestResult::Boolean(Ok(independent)) => {
                assert!(independent, "Independent data should return true");
            }
            _ => panic!("Expected TestResult::Boolean"),
        }
    }

    // --- 3. Empty array + correlated X, Y + boolean=false ---
    // Y = 3*X + small noise, so they are strongly correlated.
    // Expected: low p_value (< 0.05), high |coefficient| (> 0.9)
    #[test]
    fn test_empty_array_correlated_boolean_false() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let y = &x * 3.0 + &noise;

        let result = pearson().run_test(empty_array(), x, y, false).unwrap();
        match result {
            TestResult::Correlated(Ok((p_value, coefficient))) => {
                assert!(
                    p_value < SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be < 0.05 for correlated data"
                );
                assert!(
                    coefficient.abs() > 0.9,
                    "coefficient {coefficient} should be high for correlated data"
                );
            }
            _ => panic!("Expected TestResult::Correlated"),
        }
    }

    // --- 4. Empty array + correlated X, Y + boolean=true ---
    // Expected: false (variables are NOT independent)
    #[test]
    fn test_empty_array_correlated_boolean_true() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let y = &x * 3.0 + &noise;

        let result = pearson().run_test(empty_array(), x, y, true).unwrap();
        match result {
            TestResult::Boolean(Ok(independent)) => {
                assert!(!independent, "Correlated data should return false");
            }
            _ => panic!("Expected TestResult::Boolean"),
        }
    }

    // --- 5. Non-empty array + conditionally independent + boolean=false ---
    // Z is a confounder: X = 3*Z + noise, Y = 2*Z + noise.
    // After conditioning on Z, residuals should be independent.
    // Expected: high p_value (> 0.05), low |coefficient| (< 0.1)
    #[test]
    fn test_conditioned_independent_boolean_false() {
        let mut rng = seeded_rng();
        let z = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise_x = gen_normal(N, 0.0, 0.1, &mut rng);
        let noise_y = gen_normal(N, 0.0, 0.1, &mut rng);
        let x = &z * 3.0 + &noise_x;
        let y = &z * 2.0 + &noise_y;
        let array = z.insert_axis(Axis(1));

        let result = pearson().run_test(array, x, y, false).unwrap();
        match result {
            TestResult::Correlated(Ok((p_value, coefficient))) => {
                assert!(
                    p_value > SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be > 0.05 after conditioning"
                );
                assert!(
                    coefficient.abs() < 0.1,
                    "coefficient {coefficient} should be near 0 after conditioning"
                );
            }
            _ => panic!("Expected TestResult::Correlated"),
        }
    }

    // --- 6. Non-empty array + conditionally independent + boolean=true ---
    // Expected: true (conditionally independent given Z)
    #[test]
    fn test_conditioned_independent_boolean_true() {
        let mut rng = seeded_rng();
        let z = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise_x = gen_normal(N, 0.0, 0.1, &mut rng);
        let noise_y = gen_normal(N, 0.0, 0.1, &mut rng);
        let x = &z * 3.0 + &noise_x;
        let y = &z * 2.0 + &noise_y;
        let array = z.insert_axis(Axis(1));

        let result = pearson().run_test(array, x, y, true).unwrap();
        match result {
            TestResult::Boolean(Ok(independent)) => {
                assert!(
                    independent,
                    "Conditionally independent data should return true"
                );
            }
            _ => panic!("Expected TestResult::Boolean"),
        }
    }

    // --- 7. Non-empty array + conditionally dependent (v-structure) + boolean=false ---
    // X and Y are independent, but Z = 2*X + 2*Y + noise (collider).
    // Conditioning on Z makes X and Y dependent.
    // Expected: low p_value (< 0.05), high |coefficient|
    #[test]
    fn test_conditioned_dependent_boolean_false() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let z = &x * 2.0 + &y * 2.0 + &noise;
        let array = z.insert_axis(Axis(1));

        let result = pearson().run_test(array, x, y, false).unwrap();
        match result {
            TestResult::Correlated(Ok((p_value, coefficient))) => {
                assert!(
                    p_value < SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be < 0.05 for v-structure"
                );
                assert!(
                    coefficient.abs() > 0.9,
                    "coefficient {coefficient} should be high for v-structure"
                );
            }
            _ => panic!("Expected TestResult::Correlated"),
        }
    }

    // --- 8. Non-empty array + conditionally dependent (v-structure) + boolean=true ---
    // Expected: false (NOT independent after conditioning on collider)
    #[test]
    fn test_conditioned_dependent_boolean_true() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let z = &x * 2.0 + &y * 2.0 + &noise;
        let array = z.insert_axis(Axis(1));

        let result = pearson().run_test(array, x, y, true).unwrap();
        match result {
            TestResult::Boolean(Ok(independent)) => {
                assert!(
                    !independent,
                    "V-structure conditioned on collider should return false"
                );
            }
            _ => panic!("Expected TestResult::Boolean"),
        }
    }
}

// Potential bug in scirs2 returns NaN for p_value when N>=300. Is this a library bug
// in its t-distribution CDF calculation.
