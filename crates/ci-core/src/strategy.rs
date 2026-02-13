pub trait CITest {
    fn name(&self) -> &'static str;
    fn data_types(&self) -> &'static [&'static str];    
    fn run_test(&self);
}
