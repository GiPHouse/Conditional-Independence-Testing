use polars::prelude::*;
use std::{collections::HashMap, usize};
use scirs2_core::ndarray::*;
use scirs2_stats::contingency::*;
use std::hash::Hash;
use anyhow::Error;
use crate::strategy::CITest;

pub struct PowerDivergence {
    // Object traits
}

fn contingency_table(data: &DataFrame, col1: &str, col2: &str) -> Array2<usize> {
    let column1 = data.column(col1).unwrap().as_materialized_series();
    let column2 = data.column(col2).unwrap().as_materialized_series();
    let mut col1_data_map = HashMap::new();
    let mut col1_size: usize = 0;
    for i in column1.iter() {
        if let std::collections::hash_map::Entry::Vacant(e) = col1_data_map.entry(i) {
            e.insert(col1_size);
            col1_size += 1;
        }
    }
    let mut col2_data_map = HashMap::new();
    let mut col2_size: usize = 0;
    for i in column2.iter() {
        if let std::collections::hash_map::Entry::Vacant(e) = col2_data_map.entry(i) {
            e.insert(col2_size);
            col2_size += 1;
        }
    }
    let mut result = Array::zeros((col1_size, col2_size));
    let mut it1 = column1.iter();
    let mut it2 = column2.iter();
    let mut i = it1.next();
    let mut j = it2.next();
    while i.is_some() && j.is_some() {
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

fn main() -> anyhow::Result<()> {
let dataset: DataFrame = df! {
    "X" => [
        // Z = 1
        1,1,1,2,2,2,3,3,3,
        1,2,3,
        // Z = 2
        1,1,2,2,3,3,4,4,4,
        2,3,4
    ],
    "Y" => [
        // Z = 1
        1,2,3,1,2,3,1,2,3,
        1,2,3,
        // Z = 2
        1,2,1,2,2,3,1,2,3,
        3,1,2
    ],
    "Z" => [
        // Z = 1
        1,1,1,1,1,1,1,1,1,
        1,1,1,
        // Z = 2
        2,2,2,2,2,2,2,2,2,
        2,2,2
    ]
}?;
    let X = "X";
    let Y = "Y"; 
    let Z = [];
    let lambda = "cressie-read";

    // Step 1: Check if the arguments are valid
    if Z.contains(&X) || Z.contains(&Y){
        anyhow::bail!("X and/or Y cannot be found in Z.");
    }

    // Step 2: Do a simple contingency test if there are no conditional variables.
    if Z.len() == 0 {
        let table = contingency_table(&dataset, &X.to_string(), &Y.to_string());
        let table_f64 = table.mapv(|x| x as f64);
        let (chi2, p_value, dof, expected) = chi2_contingency(&table_f64.view(), false, None).expect("Operation failed");
        println!("{:?}", chi2);

    }

    // Step 3: If there are conditionals variables, iterate over unique states
    else {        
        let partitions = dataset.partition_by(Z, true)?;  //shows duplicates as well

        for i in 0..partitions.len(){
            let df: &DataFrame = &partitions[i];
            
            let contingency = contingency_table(&df, &X.to_string(), &Y.to_string());
            println!("{:?}", contingency);
            let contingency_f64 = contingency.mapv(|x| x as f64);

            if contingency_f64.sum_axis(Axis(0)).iter().any(|&x| x==0.0) || contingency_f64.sum_axis(Axis(1)).iter().any(|&x| x==0.0){
                let mut z_state = Vec::new();
                for col in Z {
                    let val = df.column(col)?.str()?.get(0);
                    z_state.push(val);
                }
                anyhow::bail!("Skipping the test {:?} _|_ {:?} | {:?}={:?}. Not enough samples",X,Y, Z[0], z_state); //gives error as Z[0] is not accessible as Z.len() == 0, but we are in else? not sure how to fix yet
            }
            else {
                let (chi2, p_value, dof, expected) = chi2_contingency(&contingency_f64.view(), false, None).expect("Operation failed");
                println!("{:?}", chi2);
            }
        }
    }

    Ok(())

}


impl CITest for PowerDivergence {
    fn run_test(&self){
    }
    //Other necessary stuff
}
