mod chi_squared;
mod log_likelihood;
mod independence_match;
mod modified_likelihood;
mod pearson_correlation;
mod pearson_equivalence;
mod cressie_read;
mod freeman_tukey;

use chi_squared::ChiSquared;
use log_likelihood::LogLikelihood;
use independence_match::IndependenceMatch;
use modified_likelihood::ModifiedLikelihood;
use pearson_correlation::PearsonCorrelation;
use pearson_equivalence::PearsonEquivalence;
use cressie_read::CressieRead;
use freeman_tukey::FreemanTukey;

use crate::registry::Registry;

pub fn register_all_tests(registry: &mut Registry) {
    registry
        .add_to_registry("chi_square", ChiSquared {})
        .expect("Failed to register Chi Square test!");

    registry
        .add_to_registry("log_likelihood", LogLikelihood {})
        .expect("Failed to register Log Likehood test!");

    registry
        .add_to_registry("independence_match", IndependenceMatch {})
        .expect("Failed to register Independence Match test!");

    registry
        .add_to_registry("modified_likelihood", ModifiedLikelihood {})
        .expect("Failed to register Modified Likelihood tTest!");

    registry
        .add_to_registry("pearson_correlation", PearsonCorrelation {})
        .expect("Failed to register Pearson Correlation test!");

    registry
        .add_to_registry("pearson_equivalence", PearsonEquivalence {})
        .expect("Failed to register Pearson Equivalence test!");

    registry
        .add_to_registry("cressie_read", CressieRead {})
        .expect("Failed to register Cressie Read test!");

    registry
        .add_to_registry("freeman_tukey", FreemanTukey {})
        .expect("Failed to register Freeman Tukey test!");
}
