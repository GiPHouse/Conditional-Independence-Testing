use ci_core::strategy::{TestResult};
use extendr_api::prelude::*;

pub fn test_result_to_robj(r: TestResult) -> Robj {
    match r {
        TestResult::PValue(p, coef) => list!(
            kind = "pvalue",
            p_value = p,
            coefficient = coef,
        ).into(),
        TestResult::Statistic(p, stat, df) => list!(
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