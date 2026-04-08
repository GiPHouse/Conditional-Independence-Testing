use crate::strategy::{CITest, CITestDataType, TestResult};
use anyhow::{ensure, Context};
use ndarray::{Array1, Array2, ArrayView1};
use ndarray_linalg::LeastSquaresSvd;
use statrs::distribution::{ContinuousCDF, StudentsT};
use statrs::statistics::Statistics;

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
pub struct PearsonCorrelation {}

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
        significance_level: f64,
    ) -> anyhow::Result<TestResult> {
        if conditioning_set.is_empty() {
            let (coefficient, p_value) = pearsonr(&x_values.view(), &y_values.view())?;
            Ok(wrap_result(boolean, p_value, coefficient, significance_level))
        } else {
            // Use linear regression to compute residuals and test independence on it.
            let x_coefficient = conditioning_set
                .view()
                .least_squares(&x_values.view())?
                .solution;

            let y_coefficient = conditioning_set
                .view()
                .least_squares(&y_values.view())?
                .solution;

            let residual_x = x_values - conditioning_set.dot(&x_coefficient);
            let residual_y = y_values - conditioning_set.dot(&y_coefficient);

            let (coefficient, p_value) = pearsonr(&residual_x.view(), &residual_y.view())?;
            Ok(wrap_result(boolean, p_value, coefficient, significance_level))
        }
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Continuous]
    }
}

/// Construct the appropriate [`TestResult`] variant based on the `boolean` flag.
fn wrap_result(boolean: bool, p_value: f64, coefficient: f64, significance_level: f64) -> TestResult {
    if boolean {
        return TestResult::Boolean(Ok(p_value >= significance_level));
    }
    TestResult::Correlated(Ok((p_value, coefficient)))
}

/// Compute the Pearson correlation coefficient and its two-tailed p-value.
///
/// The coefficient measures linear dependence between `x_values` and `y_values`,
/// ranging from -1 (perfect negative) to +1 (perfect positive). The p-value tests
/// H₀: ρ = 0 using the t-distribution with n − 2 degrees of freedom.
///
/// Returns `(coefficient, p_value)`.
///
/// # Errors
///
/// Returns an error if the input has fewer than 3 elements (degrees of freedom < 1).
fn pearsonr(x_values: &ArrayView1<f64>, y_values: &ArrayView1<f64>) -> anyhow::Result<(f64, f64)> {
    ensure!(
        x_values.len() == y_values.len() && x_values.len() >= 3,
        "pearsonr requires equal-length inputs with n >= 3"
    );
    #[allow(
        clippy::cast_precision_loss,
        reason = "array length most likely won't exceed 2^53"
    )]
    let number_of_elements = x_values.len() as f64;

    let x_slice = x_values.as_slice().context("invalid array layout")?;
    let y_slice = y_values.as_slice().context("invalid array layout")?;

    let covariance = x_slice.covariance(y_slice);

    let x_stdev = x_slice.std_dev();
    let y_stdev = y_slice.std_dev();

    let coefficient = covariance / (x_stdev * y_stdev);

    let t_statistic =
        coefficient * (number_of_elements - 2.0).sqrt() / ((1.0 - coefficient.powi(2)).sqrt());
    let t_distribution = StudentsT::new(0.0, 1.0, number_of_elements - 2.0)?;
    let p_value = 2.0 * t_distribution.sf(t_statistic.abs());
    Ok((coefficient, p_value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{stack, Array1, Array2, Axis};
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};

    const SIGNIFICANCE_LEVEL: f64 = 0.05;

    const N: usize = 1000;

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

    // --- 1. Empty array + independent X, Y + boolean=false ---
    // X and Y are independently generated, no conditioning variables.
    // Expected: high p_value (> 0.05), low |coefficient| (< 0.1)
    #[test]
    fn test_empty_array_independent_boolean_false() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);

        let result = pearson().run_test(empty_array(), x, y, false, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(empty_array(), x, y, true, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(empty_array(), x, y, false, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(empty_array(), x, y, true, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(array, x, y, false, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(array, x, y, true, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(array, x, y, false, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(array, x, y, true, SIGNIFICANCE_LEVEL).unwrap();
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

        let result = pearson().run_test(array, x, y, false, SIGNIFICANCE_LEVEL).unwrap();
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

    #[test]
    fn test_pearsonr_errors_on_empty_input() {
        let x: Array1<f64> = Array1::zeros(0);
        let y: Array1<f64> = Array1::zeros(0);
        assert!(pearsonr(&x.view(), &y.view()).is_err());
    }

    #[test]
    fn test_pearsonr_errors_on_too_few_elements() {
        let x = Array1::from_vec(vec![1.0, 2.0]);
        let y = Array1::from_vec(vec![3.0, 4.0]);
        assert!(pearsonr(&x.view(), &y.view()).is_err());
    }

    #[test]
    fn test_pearsonr_errors_on_mismatched_lengths() {
        let x = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let y = Array1::from_vec(vec![1.0, 2.0]);
        assert!(pearsonr(&x.view(), &y.view()).is_err());
    }

    #[test]
    fn test_pearsonr_succeeds_with_minimum_input() {
        let x = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let y = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let (coefficient, p_value) = pearsonr(&x.view(), &y.view()).unwrap();
        assert!(
            (coefficient - 1.0).abs() < 1e-10,
            "perfect positive correlation"
        );
        assert!(p_value < 0.05, "should be significant");
    }
}
