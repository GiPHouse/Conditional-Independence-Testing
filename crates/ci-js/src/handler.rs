#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::conversion::{TestRequest, TestResponse};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ci_core::strategy::{CITest,TestResult};
use ci_core::ci_tests::chi_squared::ChiSquared;
use ci_core::ci_tests::cressie_read::CressieRead;
use ci_core::ci_tests::freeman_tukey::FreemanTukey;
use ci_core::ci_tests::log_likelihood::LogLikelihood;
use ci_core::ci_tests::pearson_correlation::PearsonCorrelation;
use ci_core::ci_tests::pearson_equivalence::PearsonEquivalence;
use ndarray::{Array1, Array2};

pub async fn run_test(Path(test_name):Path<String>, Json(req):Json<TestRequest>) -> Response{
    let test = PearsonCorrelation::new(req.boolean, req.significance_level);


    let z_row = req.z.len();
    let z_col = req.z.first().map_or(0, |r|r.len());

    let z = if req.z.is_empty(){
        Array2::zeros((req.x.len(),0))
    }
    else{
        Array2::from_shape_vec((z_row, z_col), req.z.into_iter().flatten().collect()).unwrap()
    };

    let x = Array1::from_vec(req.x);
    let y = Array1::from_vec(req.y);

    match test.run_test(x, y, z) {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        Ok(result) => return(Json(map_test_result(result)). into_response())
    }
}

fn map_test_result(result:TestResult) -> TestResponse{
    match result {
    TestResult::PValue(pval, coef) => TestResponse::PValue { p_value: pval, coefficient: coef},
    TestResult::Boolean(bool) => TestResponse::Boolean { boolean: bool },
    TestResult::Statistic(pval,coef , d) => TestResponse::Statistics { p_value: pval, coefficient: coef, dof: d},
}

}
