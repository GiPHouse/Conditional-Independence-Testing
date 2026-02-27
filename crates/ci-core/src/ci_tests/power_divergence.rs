use ndarray::Array2;

use crate::strategy::CITest;

pub struct PowerDivergence {
    // Object traits
}

fn contingency_table<Q>(data: Array2<Q>, col1: usize, col2: usize) -> Array2<usize> where Q: Eq + Hash {
    let mut col1_data_map: HashMap<&Q, usize> = HashMap::new(); 
    let mut col1_size: usize = 0;
    for i in data.slice(s![..,col1]) {
        if !col1_data_map.contains_key(i) {
            col1_data_map.insert(i, col1_size);
            col1_size += 1;
        }
    }
    let mut col2_data_map: HashMap<&Q, usize> = HashMap::new();
    let mut col2_size: usize = 0;
    for i in data.slice(s![..,col2]) {
        if !col2_data_map.contains_key(i) {
            col2_data_map.insert(i, col2_size);
            col2_size += 1;
        }
    }
    let mut result = Array::zeros((col1_size,col2_size));
    let rows = data.len_of(Axis(0));
    for row in 0..rows {
        let i = &data[[row,col1]];
        let j = &data[[row,col2]];
        if let Some(result_row) = col1_data_map.get(i) {
            if let Some(result_col) = col2_data_map.get(j) {
                result[[*result_row, *result_col]] += 1;
            }
        }
    }
    result
}

impl CITest for PowerDivergence {
    fn run_test(&self) {}
    //Other necessary stuff
}
