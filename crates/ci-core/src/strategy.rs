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
pub trait CITest {
    //fn name(&self) -> &'static str;
    //fn data_types(&self) -> &'static [&'static str];    
    fn run_test(&self);
}
