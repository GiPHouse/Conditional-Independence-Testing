mod chi_squared;
mod cressie_read;
mod freeman_tukey;
mod log_likelihood;
mod modified_likelihood;
mod pearson_correlation;
mod pearson_equivalence;

pub use chi_squared::ChiSquared;
pub use cressie_read::CressieRead;
pub use freeman_tukey::FreemanTukey;
pub use log_likelihood::LogLikelihood;
pub use modified_likelihood::ModifiedLikelihood;
pub use pearson_correlation::PearsonCorrelation;
pub use pearson_equivalence::PearsonEquivalence;

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
