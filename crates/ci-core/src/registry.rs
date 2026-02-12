use std::collections::HashMap;
use super::strategy::CITest;

struct Registry {
  tests: HashMap<String, Box<dyn CITest>>
}

impl Registry {
  fn register_test() {}
  fn get_test() {}
  //Other necessary stuff
} 