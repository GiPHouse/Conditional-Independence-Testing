use ndarray::Array2;

use crate::strategy::CITest;

pub struct PowerDivergence {
    // Object traits
}

fn bincount(v: Vec<i32>, minlength: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; minlength];
    for i in v {
        if result.len() < (i as usize) + 1 {
            result.resize((i as usize) + 1, 0);
        }
        result[i as usize] += 1;
    }
    result
}

impl CITest for PowerDivergence {
    fn run_test(&self) {}
    //Other necessary stuff
}
