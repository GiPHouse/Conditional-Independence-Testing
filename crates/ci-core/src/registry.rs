use crate::strategy::CITest;
use std::collections::HashMap;


/// Finds a memory type that satisfies both hardware requirements and desired properties.
/// # Parameters
/// - `memory_properties`: The GPU's available memory types and their properties
/// - `allowed_memory_types`: Bitmask of which memory types the buffer supports
/// - `desired_properties`: The properties we need
///
/// # Returns
/// The index of the first suitable memory type found.
///
/// # Errors
/// Returns an error if no memory type satisfies both requirements.
pub struct Registry {
    pub tests: HashMap<String, Box<dyn CITest>>,
}

/// Finds a memory type that satisfies both hardware requirements and desired properties.
/// # Parameters
/// - `memory_properties`: The GPU's available memory types and their properties
/// - `allowed_memory_types`: Bitmask of which memory types the buffer supports
/// - `desired_properties`: The properties we need
///
/// # Returns
/// The index of the first suitable memory type found.
///
/// # Errors
/// Returns an error if no memory type satisfies both requirements.
impl Registry {
    pub fn get_test(&self, test_name: String) -> anyhow::Result<Option<&Box<dyn CITest>>>  {
        let test_name = test_name.to_lowercase();
        if !self.tests.contains_key(&test_name) {
            anyhow::bail!("Test not found");
        }
        Ok(self.tests.get(&test_name))
    }

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

    pub fn add_to_registry(&mut self, test_name: String, test: impl CITest+'static) -> anyhow::Result<()> {
        let test_name = test_name.to_lowercase();
        if self.tests.contains_key(&test_name) {
            anyhow::bail!("Test already exists in registry!");
        }
        let ci_test = Box::new(test);
        self.tests.insert(test_name, ci_test);
        Ok(())
    }

    
}

//let register = Registry::new()
//let chi_square = register.get_test("Chi_square")