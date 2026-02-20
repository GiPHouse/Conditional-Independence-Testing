/// Trait defining the interface for conditional independence tests.
///
/// All statistical tests for conditional independence must implement this trait
/// to be compatible with the registry system.
pub trait CITest: Send + Sync {
    //fn name(&self) -> &'static str;
    //fn data_types(&self) -> &'static [&'static str];
    fn run_test(&self);
}
