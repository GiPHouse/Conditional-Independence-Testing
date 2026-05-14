pub mod chi_squared;
pub mod cressie_read;
pub mod freeman_tukey;
pub mod log_likelihood;
pub mod modified_likelihood;
pub mod pearson_correlation;
pub mod pearson_equivalence;

use crate::strategy::CITestDataType;

/// All available CI tests paired with the data types they support.
pub const ALL_CI_TESTS: &[(&str, &[CITestDataType])] = &[
    ("chi_squared", &[CITestDataType::Discrete]),
    ("cressie_read", &[CITestDataType::Discrete]),
    ("freeman_tukey", &[CITestDataType::Discrete]),
    ("log_likelihood", &[CITestDataType::Discrete]),
    ("modified_likelihood", &[CITestDataType::Discrete]),
    ("pearson_correlation", &[CITestDataType::Continuous]),
];
