use polars::prelude::*;
use scirs2_core::prelude::*;
use std::{collections::HashMap, usize};

use crate::strategy::CITest;

pub struct PowerDivergence {
    // Object traits
}

fn contingency_table(data: &DataFrame, col1: &String, col2: &String) -> Array2<usize> {
    let column1 = data.column(col1).unwrap().as_materialized_series();
    let column2 = data.column(col2).unwrap().as_materialized_series();
    let mut col1_data_map = HashMap::new(); 
    let mut col1_size: usize = 0;
    for i in column1.iter() {
        if !col1_data_map.contains_key(&i) {
            col1_data_map.insert(i, col1_size);
            col1_size += 1;
        }
    }
    let mut col2_data_map = HashMap::new();
    let mut col2_size: usize = 0;
    for i in column2.iter() {
        if !col2_data_map.contains_key(&i) {
            col2_data_map.insert(i, col2_size);
            col2_size += 1;
        }
    }
    let mut result = Array::zeros((col1_size,col2_size));
    let mut it1 = column1.iter();
    let mut it2 = column2.iter();
    let mut i = it1.next();
    let mut j = it2.next();
    while i.is_some() && j.is_some()  {
        if let Some(result_row) = col1_data_map.get(&i.unwrap()) {
            if let Some(result_col) = col2_data_map.get(&j.unwrap()) {
                result[[*result_row, *result_col]] += 1;
            }
        }
        i = it1.next();
        j = it2.next();
    }
    result
}

impl CITest for PowerDivergence {
    fn run_test(&self) {}
    //Other necessary stuff
}
