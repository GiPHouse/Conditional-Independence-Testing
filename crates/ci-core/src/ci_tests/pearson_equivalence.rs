use crate::strategy::{CITest, CITestDataType, TestResult};
use crate::ci_tests::pearson_correlation::{PearsonCorrelation, wrap_result};
use ndarray::{Array1, Array2, Axis};
use libm::{atanh, sqrt};
use statrs::distribution::{ContinuousCDF, Normal};

#[allow(dead_code)]
pub struct PearsonEquivalence {
    pub boolean: bool,
    pub significance_level: f64,
    pub delta_threshold: f64,
}

impl PearsonEquivalence {
    pub fn new(boolean: bool, significance_level: f64, delta_threshold: f64) -> Self {
        Self {
            boolean,
            significance_level,
            delta_threshold,
        }
    }
}

impl CITest for PearsonEquivalence {
    fn run_test(
        &self,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        z: Array2<f64>,
    ) -> anyhow::Result<TestResult> {
        let n = x_values.len() as f64;
        let s = z.axis_iter(Axis(1)).len() as f64;

        let pearsonr = PearsonCorrelation{boolean: false, significance_level: self.significance_level}.run_test(x_values, y_values, z);
        let statistic = match pearsonr {
            Ok(TestResult::PValue(_, statistic)) => statistic,
            Ok(_) => 0.0,
            Err(e) => return Err(e)
        };
        let rho = if statistic <= -1.0 { 
            -0.9999999 
        } else {
            if statistic >= 1.0 {
                0.99999999 
            } 
            else {statistic}
        };

        let coefficient = atanh(rho);
        let z_delta = atanh(self.delta_threshold);

        let std_error_factor = sqrt(n - s - 3.);

        let z_score_lower = std_error_factor * (coefficient + z_delta);
        let p_value_lower = 1.0 - Normal::new(0.0, 1.0).unwrap().cdf(z_score_lower);

        let z_score_upper = std_error_factor * (coefficient - z_delta);
        let p_value_upper = Normal::new(0.0, 1.0).unwrap().cdf(z_score_upper);

        let p_value = if p_value_lower > p_value_upper { 
            p_value_lower 
        } else {
            p_value_upper
        };
        
        Ok(wrap_result(self.boolean, p_value, coefficient, self.significance_level))
    }

    fn data_types(&self) -> &'static [CITestDataType] {
        &[CITestDataType::Continuous]
    }
}

#[cfg(test)]
mod tests {
    use ndarray::{stack, array, Array1, Array2, Axis};
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};

    use super::*;

    #[test]
    fn basic_test(){
        let x_vals = array![1.0, 2.0, 3.0, 4.0];
        let y_vals = array![1.0, 1.0, 2.0, 2.0];
        let z_vals = array![[1.0], [2.0], [3.0], [4.0]];
        let empty_z = array![[]];

        let test = PearsonEquivalence {boolean: false, significance_level: 0.05, delta_threshold: 0.1};
        let result = test.run_test(x_vals, y_vals, empty_z);

        let actual = match result {
            Ok(TestResult::PValue(a, b)) => (a,b),
            _ => (0.0, 0.0)
        };

        println!("{:?}", actual);
    }

    const SIGNIFICANCE_LEVEL: f64 = 0.05;

    const DELTA_THRESHOLD: f64 = 0.1;

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

    fn pearson(boolean: bool) -> PearsonEquivalence {
        PearsonEquivalence {boolean: boolean, significance_level: SIGNIFICANCE_LEVEL, delta_threshold: DELTA_THRESHOLD}
    }

    // --- 1. Empty array + independent X, Y + boolean=false ---
    // X and Y are independently generated, no conditioning variables.
    // Expected: high p_value (> 0.05), low |coefficient| (< 0.1)
    #[test]
    fn unconditional_independent_data_is_not_rejected() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);

        let result = pearson(false)
            .run_test(x, y, empty_array())
            .unwrap();
        match result {
            TestResult::PValue(p_value, coefficient) => {
                assert!(
                    p_value > SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be > 0.05 for independent data"
                );
                assert!(
                    coefficient.abs() < 0.1,
                    "coefficient {coefficient} should be near 0 for independent data"
                );
            }
            _ => panic!("Expected TestResult::PValue"),
        }
    }

    // --- 2. Empty array + independent X, Y + boolean=true ---
    // Expected: true (variables are independent)
    #[test]
    fn unconditional_boolean_accepts_independent() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);

        let result = pearson(true)
            .run_test(x, y, empty_array())
            .unwrap();
        match result {
            TestResult::Boolean(independent) => {
                assert!(independent, "Independent data should return true");
            }
            _ => panic!("Expected TestResult::Boolean"),
        }
    }

    // --- 3. Empty array + correlated X, Y + boolean=false ---
    // Y = 3*X + small noise, so they are strongly correlated.
    // Expected: low p_value (< 0.05), high |coefficient| (> 0.9)
    #[test]
    fn unconditional_dependent_data_is_rejected() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let y = &x * 3.0 + &noise;

        let result = pearson(false)
            .run_test(x, y, empty_array())
            .unwrap();
        match result {
            TestResult::PValue(p_value, coefficient) => {
                assert!(
                    p_value < SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be < 0.05 for correlated data"
                );
                assert!(
                    coefficient.abs() > 0.9,
                    "coefficient {coefficient} should be high for correlated data"
                );
            }
            _ => panic!("Expected TestResult::PValue"),
        }
    }

    // --- 4. Empty array + correlated X, Y + boolean=true ---
    // Expected: false (variables are NOT independent)
    #[test]
    fn unconditional_boolean_rejects_dependent() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let y = &x * 3.0 + &noise;

        let result = pearson(true)
            .run_test(x, y, empty_array())
            .unwrap();
        match result {
            TestResult::Boolean(independent) => {
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
    fn conditional_independent_data_is_not_rejected() {
        let mut rng = seeded_rng();
        let z = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise_x = gen_normal(N, 0.0, 0.1, &mut rng);
        let noise_y = gen_normal(N, 0.0, 0.1, &mut rng);
        let x = &z * 3.0 + &noise_x;
        let y = &z * 2.0 + &noise_y;
        let array = z.insert_axis(Axis(1));

        let result = pearson(false)
            .run_test(x, y, array)
            .unwrap();
        match result {
            TestResult::PValue(p_value, coefficient) => {
                assert!(
                    p_value > SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be > 0.05 after conditioning"
                );
                assert!(
                    coefficient.abs() < 0.1,
                    "coefficient {coefficient} should be near 0 after conditioning"
                );
            }
            _ => panic!("Expected TestResult::PValue"),
        }
    }

    // --- 6. Non-empty array + conditionally independent + boolean=true ---
    // Expected: true (conditionally independent given Z)
    #[test]
    fn conditional_boolean_accepts_independent() {
        let mut rng = seeded_rng();
        let z = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise_x = gen_normal(N, 0.0, 0.1, &mut rng);
        let noise_y = gen_normal(N, 0.0, 0.1, &mut rng);
        let x = &z * 3.0 + &noise_x;
        let y = &z * 2.0 + &noise_y;
        let array = z.insert_axis(Axis(1));

        let result = pearson(true)
            .run_test(x, y, array)
            .unwrap();
        match result {
            TestResult::Boolean(independent) => {
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
    fn conditional_dependent_data_is_rejected() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let z = &x * 2.0 + &y * 2.0 + &noise;
        let array = z.insert_axis(Axis(1));

        let result = pearson(false)
            .run_test(x, y, array)
            .unwrap();
        match result {
            TestResult::PValue(p_value, coefficient) => {
                assert!(
                    p_value < SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be < 0.05 for v-structure"
                );
                assert!(
                    coefficient.abs() > 0.9,
                    "coefficient {coefficient} should be high for v-structure"
                );
            }
            _ => panic!("Expected TestResult::PValue"),
        }
    }

    // --- 8. Non-empty array + conditionally dependent (v-structure) + boolean=true ---
    // Expected: false (NOT independent after conditioning on collider)
    #[test]
    fn conditional_boolean_rejects_dependent() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise = gen_normal(N, 0.0, 0.1, &mut rng);
        let z = &x * 2.0 + &y * 2.0 + &noise;
        let array = z.insert_axis(Axis(1));

        let result = pearson(true)
            .run_test(x, y, array)
            .unwrap();
        match result {
            TestResult::Boolean(independent) => {
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
    fn conditional_multiple_vars_independent_is_not_rejected() {
        let mut rng = seeded_rng();
        let z_1 = gen_normal(N, 0.0, 1.0, &mut rng);
        let z_2 = gen_normal(N, 0.0, 1.0, &mut rng);
        let z_3 = gen_normal(N, 0.0, 1.0, &mut rng);
        let noise_x = gen_normal(N, 0.0, 0.1, &mut rng);
        let noise_y = gen_normal(N, 0.0, 0.1, &mut rng);
        let x = 0.5 * &z_1 + 0.5 * &z_2 + 0.5 * &z_3 + &noise_x;
        let y = 0.5 * &z_1 + 0.5 * &z_2 + 0.5 * &z_3 + &noise_y;

        let array = stack(Axis(1), &[z_1.view(), z_2.view(), z_3.view()]).unwrap();

        let result = pearson(false)
            .run_test(x, y, array)
            .unwrap();
        match result {
            TestResult::PValue(p_value, coefficient) => {
                assert!(
                    p_value >= SIGNIFICANCE_LEVEL,
                    "p_value {p_value} should be >= 0.05 after conditioning on all confounders"
                );
                assert!(
                    coefficient.abs() <= 0.1,
                    "coefficient {coefficient} should be near 0 after conditioning on all confounders"
                );
            }
            _ => panic!("Expected TestResult::PValue"),
        }
    }

    // #[test]
    // fn pearsonr_errors_on_empty_input() {
    //     let x: Array1<f64> = Array1::zeros(0);
    //     let y: Array1<f64> = Array1::zeros(0);
    //     assert!(pearsonr(&x.view(), &y.view()).is_err());
    // }

    // #[test]
    // fn pearsonr_errors_on_too_few_elements() {
    //     let x = Array1::from_vec(vec![1.0, 2.0]);
    //     let y = Array1::from_vec(vec![3.0, 4.0]);
    //     assert!(pearsonr(&x.view(), &y.view()).is_err());
    // }

    // #[test]
    // fn pearsonr_errors_on_mismatched_lengths() {
    //     let x = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    //     let y = Array1::from_vec(vec![1.0, 2.0]);
    //     assert!(pearsonr(&x.view(), &y.view()).is_err());
    // }

    // #[test]
    // fn pearsonr_succeeds_with_minimum_input() {
    //     let x = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    //     let y = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    //     let (coefficient, p_value) = pearsonr(&x.view(), &y.view()).unwrap();
    //     assert!(
    //         (coefficient - 1.0).abs() < 1e-10,
    //         "perfect positive correlation"
    //     );
    //     assert!(p_value < 0.05, "should be significant");
    // }
}
