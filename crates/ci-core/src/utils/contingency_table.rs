use ndarray::{Array, Array1, Array2};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

/// Count how often each pair `(col1[i], col2[i])` occurs and return the
/// counts as a 2D table. Rows and columns are ordered by value, so the
/// result does not depend on the order of the inputs.
pub fn contingency_table(col1: &Array1<f64>, col2: &Array1<f64>) -> Array2<f64> {
    let col1_map = build_global_category_map(col1);
    let col2_map = build_global_category_map(col2);
    contingency_table_with_categories(col1, col2, &col1_map, &col2_map)
}

/// Number every distinct value in `arr` (smallest gets 0, next gets 1,
/// and so on). Pass the same map to several calls of
/// `contingency_table_with_categories` to get tables that share the
/// exact same rows and columns.
pub fn build_global_category_map(arr: &Array1<f64>) -> HashMap<OrderedFloat<f64>, usize> {
    let mut sorted: Vec<OrderedFloat<f64>> = arr.mapv(OrderedFloat).to_vec();
    sorted.sort();
    let mut map = HashMap::new();
    for value in sorted {
        let next_index = map.len();
        map.entry(value).or_insert(next_index);
    }
    map
}

/// Like `contingency_table`, but the rows and columns are fixed by the
/// maps you pass in. Use this when several tables need the same shape,
/// even if some values are missing from a particular table.
pub fn contingency_table_with_categories(
    col1: &Array1<f64>,
    col2: &Array1<f64>,
    col1_map: &HashMap<OrderedFloat<f64>, usize>,
    col2_map: &HashMap<OrderedFloat<f64>, usize>,
) -> Array2<f64> {
    let mut result = Array::zeros((col1_map.len(), col2_map.len()));
    for i in 0..col1.len() {
        if let (Some(&r), Some(&c)) = (
            col1_map.get(&OrderedFloat(col1[i])),
            col2_map.get(&OrderedFloat(col2[i])),
        ) {
            result[[r, c]] += 1.;
        }
    }
    result
}