mod chi_squared;
mod g_test;
mod independence_match;
mod likelihood_ratio;
mod modified_likelihood;
mod pearson_correlation;
mod pearson_equivalence;
mod power_divergence;

use chi_squared::ChiSquared;
use g_test::GTest;
use independence_match::IndependenceMatch;
use likelihood_ratio::LikelihoodRatio;
use modified_likelihood::ModifiedLikelihood;
use pearson_correlation::PearsonCorrelation;
use pearson_equivalence::PearsonEquivalence;
use power_divergence::PowerDivergence;

use crate::registry::Registry;

pub fn register_all_tests(registry: &mut Registry) {
    registry
        .add_to_registry("chi_square", ChiSquared::new(1.0))
        .expect("Failed to register Chi Square test!");

    registry
        .add_to_registry("g_test", GTest::new(0.0))
        .expect("Failed to register GTest!");

    registry
        .add_to_registry("independence_match", IndependenceMatch {})
        .expect("Failed to register Independence Match test!");

    registry
        .add_to_registry("likelihood_ratio", LikelihoodRatio::new(0.0))
        .expect("Failed to register Likelihood Ratio test!");

    registry
        .add_to_registry("modified_likelihood", ModifiedLikelihood::new(-1.0))
        .expect("Failed to register Modified Likelihood tTest!");

    registry
        .add_to_registry("pearson_correlation", PearsonCorrelation {})
        .expect("Failed to register Pearson Correlation test!");

    registry
        .add_to_registry("pearson_equivalence", PearsonEquivalence::new(0.1))
        .expect("Failed to register Pearson Equivalence test!");

    registry
        .add_to_registry("power_divergence", PowerDivergence::new(-(2.0 / 3.0)))
        .expect("Failed to register Power Divergence test!");
}
