use crate::strategy::CITest;
use crate::strategy::TestResult;
use polars::prelude::*;
use scirs2_core::ndarray::{Array, Array1, Array2, Axis};
use scirs2_stats::contingency::chi2_contingency;
use std::collections::HashMap;
use polars::prelude::*;
use scirs2_core::ndarray::*;
use scirs2_stats::contingency::*;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use scirs2_stats::distributions::*;
use rand::distr::Distribution;
use rand_distr::Normal;
use ndarray::array;
use ordered_float::OrderedFloat;


const SIGNIFICANCE_LEVEL: f64 = 0.05;

pub struct PowerDivergence {
    // Object traits
}

fn contingency_table(
    data: &DataFrame,
    col1: &str,
    col2: &str,
) -> Result<Array2<usize>, PolarsError> {
    let column1 = data.column(col1)?.as_materialized_series();
    let column2 = data.column(col2)?.as_materialized_series();
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
    let mut it2 = column2.iter();
    for i in column1.iter() {
        if let Some(j) = it2.next() {
            if let Some(result_row) = col1_data_map.get(&i) {
                if let Some(result_col) = col2_data_map.get(&j) {
                    result[[*result_row, *result_col]] += 1;
                }
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

fn partition_by(
    data: &Array2<f64>,
    by_cols: &[usize],
) -> HashMap<Vec<OrderedFloat<f64>>, Array2<f64>> {
    let mut partition: HashMap<Vec<OrderedFloat<f64>>, Vec<Array1<f64>>> = HashMap::new();

    for row in data.axis_iter(Axis(0)) {
        // Build key from selected columns
        let key: Vec<OrderedFloat<f64>> = by_cols.iter().map(|&i| OrderedFloat(row[i])).collect();

        // Insert row into group
        partition    
            .entry(key)
            .or_insert_with(Vec::new)
            .push(row.to_owned());
    }

    // Convert Vec<Vec<f64>> into Array2<f64>
    partition
        .into_iter()
        .map(|(key, rows)| {
            let n_rows = rows.len();
            let n_cols = rows[0].len();

            let mut array = Array2::<f64>::zeros((n_rows, n_cols));

            for (i, row) in rows.into_iter().enumerate() {
                array.row_mut(i).assign(&row);
            }

            (key, array)
        })
        .collect()
}


fn print_partitions_pretty(partitions: &HashMap<Vec<OrderedFloat<f64>>, Array2<f64>>, custom_headers: Option<Vec<String>>) {
    for (_key, array) in partitions {
        let (rows, cols) = array.dim();
        println!("shape: ({}, {})", rows, cols);

        // Computes column names if necessary
        // Will use X,Y, Z_i as default if empty or None given, if Some(vec!["example1".into(), "example2".into()])
        // is used, it will use those as column names
        let headers: Vec<String> = if let Some(hdrs) = &custom_headers {
                if !hdrs.is_empty() {
                    hdrs.clone()
                } else {
                    (0..cols)
                        .map(|i| match i {
                            0 => "X".to_string(),
                            1 => "Y".to_string(),
                            _ => format!("Z{}", i - 2),
                        })
                        .collect()
                }
                } else {
                (0..cols)
                    .map(|i| match i {
                        0 => "X".to_string(),
                        1 => "Y".to_string(),
                        _ => format!("Z{}", i - 2),
                    })
                    .collect()
                };

        // Computes maximal decimal numbers per partition to keep accuracy
        let max_decimals_per_col: Vec<usize> = (0..cols)
            .map(|c| {
                array.column(c)
                    .iter()
                    .map(|&v| {
                        let s = format!("{:?}", v);
                        if let Some(pos) = s.find('.') {
                            s.len() - pos - 1
                        } else {
                            0
                        }
                    })
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        // Computes column width to create pretty columns
        let mut col_widths = vec![0; cols];
        for c in 0..cols {
            col_widths[c] = headers[c].len();
            for r in 0..rows {
                let val = format!("{:.*}", max_decimals_per_col[c], array[[r, c]]);
                col_widths[c] = col_widths[c].max(val.len());
            }
        }

        // For printing the borders of the tables
        let print_border = |left: &str, mid: &str, right: &str| {
            print!("{}", left);
            for (i, w) in col_widths.iter().enumerate() {
                print!("{}", "─".repeat(*w + 2));
                if i < cols - 1 {
                    print!("{}", mid);
                }
            }
            println!("{}", right);
        };

        // Top border
        print_border("┌", "┬", "┐");

        // Header
        print!("│");
        for (i, name) in headers.iter().enumerate() {
            print!(" {:<width$} ", name, width = col_widths[i]);
            print!("│");
        }
        println!();

        // Seperator between lines
        print_border("├", "┼", "┤");

        // Rows
        for r in 0..rows {
            print!("│");
            for c in 0..cols {
                let val = format!("{:.*}", max_decimals_per_col[c], array[[r, c]]);
                print!(" {:<width$} ", val, width = col_widths[c]);
                print!("│");
            }
            println!();
        }

        // Bottom order
        print_border("└", "┴", "┘");

        println!();
    }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ci_tests::power_divergence::PowerDivergence;
    use scirs2_core::ndarray::{Array1, Array2, Axis};
    use scirs2_core::random::{rngs::SmallRng, Distribution, Normal, SeedableRng};

    const N: usize = 200; // Can't have N greater than or equal to 300 due to scirs2 bug

    fn seeded_rng() -> SmallRng {
        SmallRng::seed_from_u64(42)
    }

    fn gen_normal(n: usize, mean: f64, std_dev: f64, rng: &mut SmallRng) -> Array1<f64> {
        let dist = Normal::new(mean, std_dev).unwrap();
        Array1::fromvec((0..n).map(|| dist.sample(rng)).collect())
    }

    fn gen_nxn_table(n: usize, mean: f64, std_dev: f64, rng: &mut SmallRng) -> Array2<f64> {
        let dist = Normal::new(mean, std_dev).unwrap();
        let arr2 = Array2::from_shapefn((n, n), || {dist.sample(rng)});
        println!("{:?}", arr2);
        return arr2;
    } 


    fn empty_array() -> Array2<f64> {
        Array2::zeros((0, 0))
    }

    fn empty_dataframe() -> DataFrame{
        DataFrame::empty()
    }

    fn power_divergence() -> PowerDivergence {
        PowerDivergence {}
    }

    #[test]
    fn test_empty_array() {
        let mut rng = seeded_rng();
        let x = gen_normal(N, 0.0, 1.0, &mut rng);
        let y = gen_normal(N, 0.0, 1.0, &mut rng);
        let z = gen_nxn_table(N, 0.0, 1.0, &mut rng);

        let result = power_divergence().run_test(&empty_dataframe(), "X", "Y", Array1::from_vec(vec![]), false).unwrap();
        match result {
            TestResult::Correlated(Ok((p_value, chi, dof))) => {
                assert!(
                    p_value > SIGNIFICANCE_LEVEL,
                    "p_value {pvalue} should be > 0.05 for independent data"
                );
                //assert!(
                  //  coefficient.abs() < 0.1,
                    //"coefficient {coefficient} should be near 0 for independent data"
                //);
            }
             => panic!("Expected TestResult::Correlated"),
        }
    }
}
