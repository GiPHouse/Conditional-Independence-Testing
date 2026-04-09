mod chi_squared;
mod cressie_read;
mod freeman_tukey;
mod independence_match;
mod log_likelihood;
mod modified_likelihood;
mod pearson_correlation;
mod pearson_equivalence;

use chi_squared::ChiSquared;
use cressie_read::CressieRead;
use freeman_tukey::FreemanTukey;
//use independence_match::IndependenceMatch;
use log_likelihood::LogLikelihood;
use modified_likelihood::ModifiedLikelihood;
use pearson_correlation::PearsonCorrelation;
//use pearson_equivalence::PearsonEquivalence;

use crate::registry::Registry;

pub fn register_all_tests(registry: &mut Registry) {
    registry
        .add_to_registry("chi_square", ChiSquared {})
        .expect("Failed to register Chi Square test!");

    registry
        .add_to_registry("log_likelihood", LogLikelihood {})
        .expect("Failed to register Log Likelihood test!");

    //registry
    //.add_to_registry("independence_match", IndependenceMatch {})
    //.expect("Failed to register Independence Match test!");

    registry
        .add_to_registry("modified_likelihood", ModifiedLikelihood {})
        .expect("Failed to register Modified Likelihood Test!");

    registry
        .add_to_registry("pearson_correlation", PearsonCorrelation {})
        .expect("Failed to register Pearson Correlation test!");

    //registry
    //.add_to_registry("pearson_equivalence", PearsonEquivalence {})
    //.expect("Failed to register Pearson Equivalence test!");

    registry
        .add_to_registry("cressie_read", CressieRead {})
        .expect("Failed to register Cressie Read test!");

    registry
        .add_to_registry("freeman_tukey", FreemanTukey {})
        .expect("Failed to register Freeman Tukey test!");
}
