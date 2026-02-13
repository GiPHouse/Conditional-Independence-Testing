use crate::strategy::CITest;
use std::collections::HashMap;

pub struct Registry {
    pub tests: HashMap<String, Box<dyn CITest>>,
}

impl Registry {
    pub fn register_tests() {} // Maybe this can run internally automatically?
    pub fn get_test() {}
    //Other necessary stuff
}
