use ndarray::{Array2, Axis};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

/// Group the rows of `data` by their value. Returns one list of row
/// indices per distinct row, so each list points to the rows that share
/// the same combination of values.
#[must_use]
pub fn partition_by(data: &Array2<f64>) -> Vec<Vec<usize>> {
    let mut groups: HashMap<Vec<OrderedFloat<f64>>, Vec<usize>> = HashMap::new();
    for (i, row) in data.axis_iter(Axis(0)).enumerate() {
        let key: Vec<OrderedFloat<f64>> = row.iter().map(|&v| OrderedFloat(v)).collect();
        groups.entry(key).or_default().push(i);
    }
    groups.into_values().collect()
}
