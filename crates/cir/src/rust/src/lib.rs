//! R bindings for the conditional independence testing library.
//!
//! Exposes CI test functions and registry queries to R via the `extendr` framework.
//! Each CI test accepts paired observation vectors and a conditioning matrix, returning
//! a named R list whose shape depends on whether the test runs in boolean or numeric mode.

use extendr_api::prelude::*;
use ci_core::registry::Registry;
use ci_core::strategy::{CITest, CITestDataType};
use ci_core::ci_tests::{
    chi_squared::ChiSquared,
    cressie_read::CressieRead,
    freeman_tukey::FreemanTukey,
    log_likelihood::LogLikelihood,
    modified_likelihood::ModifiedLikelihood,
    pearson_correlation::PearsonCorrelation,
    pearson_equivalence::PearsonEquivalence,
};
use ndarray::{ArrayView1, ArrayView2};
mod util;

/// Generates an R-callable wrapper for a [`CITest`] implementation.
///
/// The generated function signature is:
/// ```text
/// fn $fn_name(x_values, y_values, z, boolean, significance_level) -> Robj
/// ```
/// - `x_values` / `y_values`: paired observation vectors.
/// - `z`: conditioning matrix; pass a 0-column matrix for unconditional tests.
/// - `boolean`: when `true`, returns only an independence verdict at `significance_level`
///   instead of the raw test statistic and p-value.
/// - `significance_level`: threshold used only when `boolean` is `true`.
macro_rules! r_ci_test {
    ($fn_name:ident, $inner:ty) => {
        #[extendr]
        fn $fn_name(
            x_values: ArrayView1<f64>,
            y_values: ArrayView1<f64>,
            z: ArrayView2<f64>,
            boolean: bool,
            significance_level: f64,
        ) -> anyhow::Result<Robj> {
            let citest = <$inner>::new(boolean, significance_level);
            let result = citest.run_test(
                x_values.to_owned(),
                y_values.to_owned(),
                z.to_owned(),
            )?;
            Ok(util::test_result_to_robj(result))
        }
    };
}

/// Returns a sorted vector of all registered CI test names.
#[extendr]
fn list_ci_tests() -> anyhow::Result<Vec<String>> {
    let registry = Registry::new();
    let mut tests: Vec<String> = registry.all_tests()?.map(String::from).collect();
    tests.sort();
    Ok(tests)
}

/// Returns a sorted vector of CI test names compatible with the given data type.
///
/// `data_type` must be one of `"discrete"`, `"continuous"`, or `"mixed"` (case-insensitive).
/// Returns an error for any other value.
#[extendr]
fn list_ci_tests_for(data_type: &str) -> anyhow::Result<Vec<String>> {
    let dt = match data_type.to_lowercase().as_str() {
        "discrete" => CITestDataType::Discrete,
        "continuous" => CITestDataType::Continuous,
        "mixed" => CITestDataType::Mixed,
        _ => anyhow::bail!("Unknown data type: '{data_type}'. Use 'discrete', 'continuous', or 'mixed'."),
    };
    let registry = Registry::new();
    let mut tests: Vec<String> = registry.tests_with_data_type(&dt)?.map(String::from).collect();
    tests.sort();
    Ok(tests)
}

r_ci_test!(chi_squared_test, ChiSquared);
r_ci_test!(log_likelihood_test, LogLikelihood);
r_ci_test!(cressie_read_test, CressieRead);
r_ci_test!(pearson_correlation_test, PearsonCorrelation);
r_ci_test!(freeman_tukey_test, FreemanTukey);
r_ci_test!(modified_likelihood_test, ModifiedLikelihood);
r_ci_test!(pearson_equivalence_test, PearsonEquivalence);


extendr_module! {
    mod cir;
    fn list_ci_tests;
    fn list_ci_tests_for;
    fn chi_squared_test;
    fn log_likelihood_test;
    fn cressie_read_test;
    fn pearson_correlation_test;
    fn freeman_tukey_test;
    fn modified_likelihood_test;
    fn pearson_equivalence_test;
}