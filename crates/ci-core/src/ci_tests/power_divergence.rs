use crate::strategy::{CITest,TestResult};
use polars::prelude::{DataFrame, PolarsError, Series, NamedFrom, df};
use scirs2_core::ndarray::{Array, Array1, Array2, Axis};
use scirs2_stats::contingency::chi2_contingency;
use std::collections::HashMap;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use ordered_float::OrderedFloat;

const SIGNIFICANCE_LEVEL: f64 = 0.05;

pub struct PowerDivergence {
    // Object traits
}

fn build_unique_value_map(arr: &Vec<OrderedFloat<f64>>) -> (HashMap<OrderedFloat<f64>, usize>, usize) {
    // create resulting map and length
    let mut result_map: HashMap<OrderedFloat<f64>, usize> = HashMap::new();
    let mut unique_values: usize = 0;

    // add all unique values to the map, and map them to the order in which they appear
    for i in arr {
        if let std::collections::hash_map::Entry::Vacant(e) = result_map.entry(*i) {
            e.insert(unique_values);
            unique_values += 1;
        }
    }
    (result_map, unique_values)
}

//Computation of contingency_table
fn contingency_table(
    col1: &Array1<f64>,
    col2: &Array1<f64>,
) -> Result<Array2<f64>, PolarsError> {
    // sort the arrays (into vectors) to make the contingency table independent of order
    let mut sorted_col1: Vec<OrderedFloat<f64>> = col1.mapv(|i| OrderedFloat(i)).to_vec();
    sorted_col1.sort();
    let mut sorted_col2: Vec<OrderedFloat<f64>> = col2.mapv(|i| OrderedFloat(i)).to_vec();
    sorted_col2.sort();

    // create unique value map and compute number of unique values for column1 and column2. 
    let (col1_data_map, col1_size) = build_unique_value_map(&sorted_col1);
    let (col2_data_map, col2_size) = build_unique_value_map(&sorted_col2);

    //allocate contingency table
    let mut result = Array::zeros((col1_size, col2_size));

    //compute contingency table
    for i in 0..col1.len() {
        if let Some(result_row) = col1_data_map.get(&OrderedFloat(col1[i])) {
            if let Some(result_col) = col2_data_map.get(&OrderedFloat(col2[i])) {
                result[[*result_row, *result_col]] += 1.;
            }
        }
    }
    Ok(result)
}

fn result(boolean: bool, p_value: f64, chi2: f64, dof: usize) -> TestResult {
    if boolean {
        return TestResult::Boolean(Ok(p_value >= SIGNIFICANCE_LEVEL));
    }
    return TestResult::Correlated(Ok((p_value, chi2, dof)));
}

impl CITest for PowerDivergence {
    fn run_test(
        &self,
        data: &DataFrame,
        col_x: &str,
        col_y: &str,
        cols_z: Array1<String>,
        boolean: bool,
    ) -> anyhow::Result<TestResult> {

        // Step 1: Check if the arguments are valid
        if cols_z.iter().any(|x| x == col_x || x == col_y) {
            anyhow::bail!("X and/or Y cannot be found in Z.");
        }

        // Step 2: Do a simple contingency test if there are no conditional variables.
        if cols_z.is_empty() {
            let table = contingency_table(data, col_x, col_y)?;
            let table_f64 = table.mapv(|x| x as f64);
            let (chi2, p_value, dof, _expected) =
                chi2_contingency(&table_f64.view(), false, None).expect("Operation failed");
            Ok(result(boolean, p_value, chi2, dof))
        }
        // Step 3: If there are conditionals variables, iterate over unique states
        else {
            let mut chi: f64 = 0.0;
            let mut dof: usize = 0;
            let partitions = data.partition_by(cols_z, true)?; //shows duplicates as well
            for df in partitions {
                let contingency = contingency_table(&df, col_x, col_y)?;
                let contingency_f64 = contingency.mapv(|x| x as f64);

                /* Hypothesis: The code never enters this if statement, even if it is present in the actual pgmpy package */
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
                let (c, _p_value, d, _expected) =
                    chi2_contingency(&contingency_f64.view(), false, None)
                        .expect("Operation failed");
                chi += c;
                dof += d;
            }
            // TODO: find chi^2 distribution
            let chi2 = ChiSquared::new(dof as f64)?;
            let p_value = 1.0 - chi2.cdf(chi);
            Ok(result(boolean, p_value, chi, dof))
        }
    }
    //Other necessary stuff
}

impl PowerDivergence{
    fn test_wrapper(
        &self,
        array: Array2<f64>,
        x_values: Array1<f64>,
        y_values: Array1<f64>,
        boolean: bool,
    ) -> anyhow::Result<TestResult> {   

        //Code starts with conversion into dataframe, allowing application of polars partition by. 
        //Performance increase is possible by custom partition by implementation for ndarrays.  
        let mut data = df!{
            "x" => x_values.to_vec(),
            "y" => y_values.to_vec(),
        }.unwrap();
        let num_cols = array.len_of(Axis(1));
        let mut col_names: Vec<String> = vec![];
        for col in 0..num_cols {
            let col_name = format!("Z_{}", col);
            col_names.push(col_name);
            data.with_column(Series::new(format!("Z_{}", col).into(), array.index_axis(Axis(1), col).to_vec())).unwrap();
        }
        self.run_test(&data, "x", "y", Array::from(col_names), boolean)
    }
}
