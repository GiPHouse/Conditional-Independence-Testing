use extendr_api::prelude::*;
use ci_core::strategy::{CITest, TestResult};
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
                    Ok(test_result_to_robj(result))
            }
        }
    };
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
    impl RChiSquared;
    impl RLogLikelihood;
    impl RCressieRead;
    impl RPearsonCorrelation;
    impl RFreemanTukey;
    impl RModifiedLikelihood;
    impl RPearsonEquivalence;
}

fn test_result_to_robj(r: TestResult) -> Robj {
    match r {
        TestResult::PValue(p, coef) => list!(
            kind = "pvalue",
            p_value = p,
            coefficient = coef,
        ).into(),
        TestResult::Statistic(stat, p, df) => list!(
            kind = "statistic",
            statistic = stat,
            p_value = p,
            df = df as i32,
        ).into(),
        TestResult::Boolean(b) => list!(
            kind = "boolean",
            independent = b,
        ).into(),
    }
}