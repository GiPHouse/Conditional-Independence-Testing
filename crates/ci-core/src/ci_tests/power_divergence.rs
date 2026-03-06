use crate::strategy::CITest;
use anyhow::Error;
use polars::prelude::*;
use scirs2_core::ndarray::{Array2, DataOwned, Array, IndexLonger, Array1};
use scirs2_stats::contingency::chi2_contingency;
use std::collections::HashMap;

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

impl CITest for PowerDivergence {
    fn run_test(
        &self,
        data: &DataFrame,
        col_x: &str,
        col_y: &str,
        cols_z: Array1<&str>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // Step 1: Check if the arguments are valid
        if cols_z.iter().any(|&x| x == col_x || x == col_y) {
            anyhow::bail!("X and/or Y cannot be found in Z.");
        }

        // Step 2: Do a simple contingency test if there are no conditional variables.
        if cols_z.is_empty() {
            let table = contingency_table(data, col_x, col_y);
            let table_f64 = table.mapv(|x| x as f64);
            let (chi2, p_value, dof, expected) =
                chi2_contingency(&table_f64.view(), false, None).expect("Operation failed");
            println!("{:?}", chi2);
        }
        // Step 3: If there are conditionals variables, iterate over unique states
        else {
            let partitions = data.partition_by(cols_z, true)?; //shows duplicates as well
            for df in partitions {
                let contingency = contingency_table(&df, col_x, col_y);
                println!("{:?}", contingency);
                let contingency_f64 = contingency.mapv(|x| x as f64);

                /* Hypothesis: this code is never touched */
                // if contingency_f64.sum_axis(Axis(0)).iter().any(|&x| x==0.0) || contingency_f64.sum_axis(Axis(1)).iter().any(|&x| x==0.0){
                //     let mut z_state = Vec::new();
                //     for col in cols_z {
                //         let val = df.column(col)?.str()?.get(0);
                //         z_state.push(val);
                //     }
                //     anyhow::bail!("Skipping the test {:?} _|_ {:?} | {:?}={:?}. Not enough samples",col_x,col_y, cols_z[0], z_state);
                // }
                // else {
                //     let (chi2, p_value, dof, expected) = chi2_contingency(&contingency_f64.view(), false, None).expect("Operation failed");
                //     println!("{:?}", chi2);
                // }
                let (chi2, p_value, dof, expected) =
                    chi2_contingency(&contingency_f64.view(), false, None)
                        .expect("Operation failed");
                println!("{:?}", chi2);
            }
        }
        Ok(())
    }
    //Other necessary stuff
}
