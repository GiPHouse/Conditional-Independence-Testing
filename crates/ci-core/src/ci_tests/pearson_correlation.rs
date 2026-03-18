use crate::strategy::{CITest, TestResult};
use ndarray;
use ndarray_linalg::LeastSquaresSvd;
//use scirs2::stats::pearsonr;
use statrs;

const SIGNIFICANCE_LEVEL: f64 = 0.05;

/// Pearson correlation conditional independence test.
///
/// Should be used only on continuous data. When the conditioning set is non-empty,
/// uses linear regression to compute residuals and tests the Pearson correlation
/// on those residuals (partial correlation).
///
/// # References
///
/// - [Pearson correlation coefficient](https://en.wikipedia.org/wiki/Pearson_correlation_coefficient)
/// - [Partial correlation using linear regression](https://en.wikipedia.org/wiki/Partial_correlation#Using_linear_regression)
pub struct PearsonCorrelation {
    // Object traits
}

impl CITest for PearsonCorrelation {
    /// Test the independence condition X ⊥ Y | Z using Pearson correlation.
    ///
    /// # Parameters
    ///
    /// - `conditioning_set` - Conditioning variables Z for testing X ⊥ Y | Z.
    ///   Pass an empty array for unconditional testing.
    /// - `x_values` - The first variable X.
    /// - `y_values` - The second variable Y.
    /// - `boolean` - If true, returns a boolean indicating independence
    ///   (based on `SIGNIFICANCE_LEVEL`). If false, returns the (p-value, coefficient) tuple.
    ///
    /// # Returns
    ///
    /// - If `boolean=true`: `TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL))`
    /// - If `boolean=false`: `TestResult::Correlated(Ok((p_value, coefficient)))`
    fn run_test(
        &self,
        conditioning_set: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult> {
        if conditioning_set.is_empty() {
            let (coefficient, p_value) = pearsonr(&x_values.view(), &y_values.view())?;
            Ok(result(boolean, p_value, coefficient))
        } else {
            // If conditioning_set is non-empty, use linear regression to compute residuals and test independence on it.
            let x_coefficient = &conditioning_set
                .view()
                .least_squares(&x_values.view())?
                .solution;

            let y_coefficient = &conditioning_set
                .view()
                .least_squares(&y_values.view())?
                .solution;

            let residual_x = x_values - conditioning_set.dot(&x_coefficient);
            let residual_y = y_values - conditioning_set.dot(&y_coefficient);

            let (coefficient, p_value) = pearsonr(&residual_x.view(), &residual_y.view())?;
            Ok(result(boolean, p_value, coefficient))
        }
    }
}

/// Construct the appropriate [`TestResult`] variant based on the `boolean` flag.
fn result(boolean: bool, p_value: f64, coefficient: f64) -> TestResult {
    if boolean {
        return TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL));
    }
    TestResult::Correlated(Ok((p_value, coefficient)))
}

//fn pearsonr(residual_x: Array1, residual_y: Array1) -> anyhow::Result<(f64, f64)> {
//    Ok((1.0, 1.0))
//}

#[cfg(test)]
mod tests {
    use super::*;
    use scirs2_core::ndarray::{stack, Array1, Array2, Axis};
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

    // Testing scirs2's pearsonr limitations/bugs.
    #[test]
    #[ignore = "for future debugging"]
    fn debug_pearsonr_sizes() {
        let mut rng = seeded_rng();
        for n in [200, 300, 350, 400, 450, 500] {
            let x = gen_normal(n, 0.0, 1.0, &mut rng);
            let y = gen_normal(n, 0.0, 1.0, &mut rng);
            let raw = pearsonr(&x.view(), &y.view(), "two-sided");
            eprintln!("N={n}: {raw:?}");
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
    // --- 9. Multiple conditioning variables + conditionally independent + boolean=false ---
    // Z1, Z2, Z3 are confounders: X and Y both depend on them.
    // After conditioning on all three, residuals should be independent.
    // Expected: high p_value, low |coefficient|
    #[test]
    fn test_multiple_conditioned_independent_boolean_false() {
        let mut rng = seeded_rng();
        let z_1 = gen_normal(N, 0.0, 1.0, &mut rng);
        let z_2 = gen_normal(N, 0.0, 1.0, &mut rng);
        let z_3 = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise_x = gen_normal(N, 0.0, 0.1, &mut rng);
        let noise_y = gen_normal(N, 0.0, 0.1, &mut rng);
        let x = 0.5 * &z_1 + 0.5 * &z_2 + 0.5 * &z_3 + &noise_x;
        let y = 0.5 * &z_1 + 0.5 * &z_2 + 0.5 * &z_3 + &noise_y;

        let array = stack(Axis(1), &[z_1.view(), z_2.view(), z_3.view()]).unwrap();

        let result = pearson().run_test(array, x, y, false).unwrap();
        match result {
            TestResult::Correlated(Ok((p_value, coefficient))) => {
                assert!(
                    p_value >= SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be >= 0.05 after conditioning on all confounders"
                );
                assert!(
                    coefficient.abs() <= 0.1,
                    "coefficient {coefficient} should be near 0 after conditioning on all confounders"
                );
            }
            _ => panic!("Expected TestResult::Correlated"),
        }
    }
}
