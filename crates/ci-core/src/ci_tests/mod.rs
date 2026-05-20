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

use crate::registry::Registry;

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
        .add_to_registry("pearson_correlation", PearsonCorrelation::new(true, 0.05))
        .expect("Failed to register Pearson Correlation test!");

    registry
        .add_to_registry("pearson_equivalence", PearsonEquivalence::new(true, 0.05))
        .expect("Failed to register Pearson Equivalence test!");

    registry
        .add_to_registry("cressie_read", CressieRead::new(true, 0.05))
        .expect("Failed to register Cressie Read test!");

    registry
        .add_to_registry("freeman_tukey", FreemanTukey::new(true, 0.05))
        .expect("Failed to register Freeman Tukey test!");
}
