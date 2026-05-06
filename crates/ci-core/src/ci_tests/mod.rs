pub mod chi_squared;
pub mod cressie_read;
pub mod freeman_tukey;
pub mod log_likelihood;
pub mod modified_likelihood;
pub mod pearson_correlation;
pub mod pearson_equivalence;

use chi_squared::ChiSquared;
use cressie_read::CressieRead;
use freeman_tukey::FreemanTukey;
use log_likelihood::LogLikelihood;
use modified_likelihood::ModifiedLikelihood;
use pearson_correlation::PearsonCorrelation;
use pearson_equivalence::PearsonEquivalence;

use crate::registry::Registry;

/// # Panics
///
/// Panics if any test name is already registered (indicates a duplicate registration bug).
pub fn register_all_tests(registry: &mut Registry) {
    registry
        .add_to_registry("chi_square", ChiSquared::new(true, 0.05))
        .expect("Failed to register Chi Square test!");

    registry
        .add_to_registry("log_likelihood", LogLikelihood::new(true, 0.05))
        .expect("Failed to register Log Likelihood test!");

    registry
        .add_to_registry("modified_likelihood", ModifiedLikelihood::new(true, 0.05))
        .expect("Failed to register Modified Likelihood Test!");

    registry
        .add_to_registry("pearson_correlation", PearsonCorrelation::new(false, 0.05))
        .expect("Failed to register Pearson Correlation test!");

    registry
        .add_to_registry("pearson_equivalence", PearsonEquivalence::new(false, 0.05, 0.1))
        .expect("Failed to register Pearson Equivalence test!");

    registry
        .add_to_registry("cressie_read", CressieRead::new(true, 0.05))
        .expect("Failed to register Cressie Read test!");

    registry
        .add_to_registry("freeman_tukey", FreemanTukey::new(true, 0.05))
        .expect("Failed to register Freeman Tukey test!");
}
