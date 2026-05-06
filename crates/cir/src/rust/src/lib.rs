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

macro_rules! r_ci_test {
    ($r_name:ident, $inner:ty) => {
        #[extendr]
        pub struct $r_name {
            citest: $inner
        }
        
        #[extendr]
        impl $r_name {
            fn new(
                boolean: bool,
                significance_level: f64
                ) -> Self {
            let citest = <$inner>::new(boolean, significance_level);
            $r_name{citest: citest}
            }

            fn run_test(
                &self,
                x_values: ArrayView1<f64>,
                y_values: ArrayView1<f64>,
                z: ArrayView2<f64>,
                ) -> anyhow::Result<Robj> {

                    let result = self.citest.run_test(
                    x_values.to_owned(), 
                    y_values.to_owned(), 
                    z.to_owned())?;
                    Ok(util::test_result_to_robj(result))
            }
        }
    };
}

#[extendr]
fn list_ci_tests() -> anyhow::Result<Vec<String>> {
    let registry = Registry::new();
    let mut tests: Vec<String> = registry.all_tests()?.map(String::from).collect();
    tests.sort();
    Ok(tests)
}

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

r_ci_test!(RChiSquared, ChiSquared);
r_ci_test!(RLogLikelihood, LogLikelihood);
r_ci_test!(RCressieRead, CressieRead);
r_ci_test!(RPearsonCorrelation, PearsonCorrelation);
r_ci_test!(RFreemanTukey, FreemanTukey);
r_ci_test!(RModifiedLikelihood, ModifiedLikelihood);
r_ci_test!(RPearsonEquivalence, PearsonEquivalence);


extendr_module! {
    mod cir;
    fn list_ci_tests;
    fn list_ci_tests_for;
    impl RChiSquared;
    impl RLogLikelihood;
    impl RCressieRead;
    impl RPearsonCorrelation;
    impl RFreemanTukey;
    impl RModifiedLikelihood;
    impl RPearsonEquivalence;
}