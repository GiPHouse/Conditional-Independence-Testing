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

use crate::registry::CITestDataType::{Continuous, Discrete, Mixed};
use crate::registry::Registry;

pub fn register_all_tests(registry: &mut Registry) {
    registry
        .add_to_registry("chi_square", ChiSquared {}, Box::new([Discrete]))
        .expect("Failed to register Chi Square test!");

    registry
        .add_to_registry("g_test", GTest {}, Box::new([Discrete]))
        .expect("Failed to register GTest!");

    registry
        .add_to_registry(
            "independence_match",
            IndependenceMatch {},
            Box::new([Continuous, Discrete, Mixed]),
        )
        .expect("Failed to register Independence Match test!");

    registry
        .add_to_registry("likelihood_ratio", LikelihoodRatio {}, Box::new([Discrete]))
        .expect("Failed to register Likelihood Ratio test!");

    registry
        .add_to_registry(
            "modified_likelihood",
            ModifiedLikelihood {},
            Box::new([Discrete]),
        )
        .expect("Failed to register Modified Likelihood tTest!");

    registry
        .add_to_registry(
            "pearson_correlation",
            PearsonCorrelation {},
            Box::new([Continuous]),
        )
        .expect("Failed to register Pearson Correlation test!");

    registry
        .add_to_registry(
            "pearson_equivalence",
            PearsonEquivalence {},
            Box::new([Continuous]),
        )
        .expect("Failed to register Pearson Equivalence test!");

    registry
        .add_to_registry("power_divergence", PowerDivergence {}, Box::new([Discrete]))
        .expect("Failed to register Power Divergence test!");
}
