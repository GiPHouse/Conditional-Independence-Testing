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
use ci_core::strategy::CITest;
use ci_core::ci_tests::chi_squared::ChiSquared;
use ci_core::ci_tests::cressie_read::CressieRead;
use ci_core::ci_tests::freeman_tukey::FreemanTukey;
use ci_core::ci_tests::log_likelihood::LogLikelihood;
use ci_core::ci_tests::pearson_correlation::PearsonCorrelation;
use ci_core::ci_tests::pearson_equivalence::PearsonEquivalence;
use ndarray::{Array1, Array2};

//pub async fn run_test(Path(test_name):Path<String>, Json(req):Json<TestRequest>) -> Response{}