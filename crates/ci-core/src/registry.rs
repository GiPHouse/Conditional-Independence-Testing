use std::collections::HashMap;
use crate::strategy::CITest;

struct Registry {
  tests: HashMap<String, Box<dyn CITest>>
}

impl Registry {
  fn register_tests() {} // Maybe this can run internally automatically?
  fn get_test() {}
  //Other necessary stuff
} 