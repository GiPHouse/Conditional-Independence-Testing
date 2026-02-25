use crate::strategy::CITest;
use std::collections::HashMap;

/// Central registry for managing available conditional independence test implementations.
///
/// The registry maintains a collection of test implementations that can be retrieved
/// by name.
pub struct Registry {
    pub tests: HashMap<String, fn() -> Box<dyn CITest>>,
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
    pub fn get_test(&self, test_name: &str) -> anyhow::Result<Box<dyn CITest>> {
        let test_name = test_name.to_lowercase();
        let test = self.tests.get(&test_name);
        match test {
            Some(t) => Ok(t()),
            None => Err(anyhow::anyhow!("Test '{test_name}' not found!")),
        }
    }

    /// Returns a list of all registered test names.
    ///
    /// # Returns
    /// A vector containing references to all test names in the registry.
    ///
    /// # Errors
    /// Returns an error if the registry is empty.
    pub fn list_all_tests(&self) -> anyhow::Result<Vec<&String>> {
        if self.tests.is_empty() {
            anyhow::bail!("No tests found!");
        }

        let array_size = self.tests.len();
        let mut all_tests = Vec::with_capacity(array_size);
        for test in self.tests.keys() {
            all_tests.push(test);
        }
        Ok(all_tests)
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
        test: fn() -> Box<dyn CITest>,
    ) -> anyhow::Result<()> {
        let test_name = test_name.to_lowercase();
        if self.tests.contains_key(&test_name) {
            anyhow::bail!("Test already exists in registry!");
        };
        self.tests.insert(test_name, test);
        Ok(())
    }
}

//let register = Registry::new()
//let chi_square = register.get_test("Chi_square")

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test to check register creation
    fn test_registry_new() {
        let registry = Registry::new();
        assert_ne!(registry.tests.len(), 0);
    }

    #[test]
    // Test to check getting tests
    fn test_get_test() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        let registry = Registry::new();
        assert!(registry.get_test("chi_square").is_ok());
        assert!(registry.get_test("dummy").is_err());
    }

    #[test]
    // Test to check listing all tests
    fn test_list_tests() -> anyhow::Result<()> {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        let registry = Registry::new();
        assert!(!registry.list_all_tests()?.is_empty());
        Ok(())
    }
}
