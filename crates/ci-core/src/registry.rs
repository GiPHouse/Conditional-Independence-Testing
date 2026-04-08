use crate::strategy::{CITest, CITestDataType};
use std::collections::HashMap;

/// Central registry for managing available conditional independence test implementations.
///
/// The registry maintains a collection of test implementations that can be retrieved
/// by name.
pub struct Registry {
    tests: HashMap<String, Box<dyn CITest>>,
}

impl Registry {
    /// Creates a new Registry with all `CITests` as elements.
    #[must_use = "creating a Registry without using it has no effect"]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut registry = Self {
            tests: HashMap::new(),
        };

        crate::ci_tests::register_all_tests(&mut registry);

        registry
    }
    /// Retrieves a test implementation by name.
    ///
    /// # Parameters
    /// - `test_name`: Name of the test to retrieve (case-insensitive)
    ///
    /// # Returns
    /// A reference to the test implementation.
    ///
    /// # Errors
    /// Returns an error if the test name is not found in the registry.
    pub fn get_test(&self, test_name: &str) -> anyhow::Result<&dyn CITest> {
        let test_name = test_name.to_lowercase();
        self.tests
            .get(&test_name)
            .map(std::convert::AsRef::as_ref)
            .ok_or_else(|| anyhow::anyhow!("Test '{test_name}' not found!"))
    }

    /// Returns a list of all registered test names.
    ///
    /// # Returns
    /// A vector containing references to all test names in the registry.
    ///
    /// # Errors
    /// Returns an error if the registry is empty.
    pub fn all_tests(&self) -> anyhow::Result<impl Iterator<Item = &str>> {
        if self.tests.is_empty() {
            anyhow::bail!("No tests found!");
        }

        Ok(self.tests.keys().map(std::string::String::as_str))
    }

    /// Returns a list of the registered test names that support the given `data_type`.
    ///
    /// # Returns
    /// A vector containing references to the test names in the registry.
    ///
    /// # Errors
    /// Returns an error if the registry is empty.
    pub fn tests_with_data_type<'a: 'b, 'b>(
        &'a self,
        data_type: &'b CITestDataType,
    ) -> anyhow::Result<impl Iterator<Item = &'a str> + 'b> {
        if self.tests.is_empty() {
            anyhow::bail!("No tests found!");
        }

        Ok(self
            .tests
            .iter()
            .filter(|(_, v)| v.data_types().contains(data_type))
            .map(|(k, _)| k.as_str()))
    }

    /// Adds a new test implementation to the registry.
    ///
    /// # Parameters
    /// - `test_name`: Unique identifier for the test (case-insensitive)
    /// - `test`: Implementation of the `CITest` trait
    ///
    /// # Errors
    /// Returns an error if a test with the same name already exists.
    pub fn add_to_registry(
        &mut self,
        test_name: &str,
        test: impl CITest + 'static,
    ) -> anyhow::Result<()> {
        let test_name = test_name.to_lowercase();
        if self.tests.contains_key(&test_name) {
            anyhow::bail!("Test already exists in registry!");
        }
        self.tests.insert(test_name, Box::new(test));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::CITestDataType::{Continuous, Discrete, Mixed};
    use super::*;

    #[test]
    /// Test to check registry creation.
    fn test_registry_new() {
        let registry = Registry::new();
        assert_ne!(registry.tests.len(), 0);
    }

    #[test]
    /// Test to check getting tests.
    fn test_get_test() {
        let registry = Registry::new();
        assert!(registry.get_test("chi_square").is_ok());
        assert!(registry.get_test("dummy").is_err());
    }

    #[test]
    /// Test to check listing all tests.
    fn test_all_tests() -> anyhow::Result<()> {
        let registry = Registry::new();
        assert!(!registry.all_tests()?.collect::<Vec<&str>>().is_empty());
        Ok(())
    }

    #[test]
    /// Test to check listing all tests supporting continuous data.
    ///
    /// We check if *at least* all current CI tests supporting continuous data are returned so that
    /// the addition of future tests doesn't necessitate updating the tests of the registry.
    fn test_tests_with_data_type_continuous() -> anyhow::Result<()> {
        let registry = Registry::new();
        for test in [
            "independence_match",
            "pearson_correlation",
            "pearson_equivalence",
        ] {
            assert!(registry
                .tests_with_data_type(&Continuous)?
                .any(|t| t == test));
        }
        Ok(())
    }

    #[test]
    /// Test to check listing all tests supporting discrete data.
    ///
    /// We check if *at least* all current CI tests supporting discrete data are returned so that
    /// the addition of future tests doesn't necessitate updating the tests of the registry.
    fn test_tests_with_data_type_discrete() -> anyhow::Result<()> {
        let registry = Registry::new();
        for test in [
            "chi_square",
            "g_test",
            "independence_match",
            "likelihood_ratio",
            "modified_likelihood",
            "power_divergence",
        ] {
            assert!(registry.tests_with_data_type(&Discrete)?.any(|t| t == test));
        }
        Ok(())
    }

    #[test]
    /// Test to check listing all tests supporting mixed data.
    ///
    /// We check if *at least* all current CI tests supporting mixed data are returned so that
    /// the addition of future tests doesn't necessitate updating the tests of the registry.
    fn test_tests_with_data_type_mixed() -> anyhow::Result<()> {
        let registry = Registry::new();
        assert!(registry
            .tests_with_data_type(&Mixed)?
            .any(|t| t == "independence_match"));
        Ok(())
    }
}
