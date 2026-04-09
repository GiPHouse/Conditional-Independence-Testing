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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn simple_grouping() {
        let data = array![
            [1.0, 10.0],
            [1.0, 10.0],
            [2.0, 30.0],
        ];

        let result = partition_by(&data);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_empty_array() {
        let data: Array2<f64> = Array2::zeros((0, 2));
        let result = partition_by(&data);

        assert!(result.is_empty());
    }

    #[test]
    fn test_singleton_groups() {
        let data = array![
            [1.0, 10.0],
            [2.0, 20.0],
            [3.0, 30.0],
        ];

        let result = partition_by(&data);

        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|g| g.len() == 1));
    }

    //All values are in a single group
    #[test]
    fn test_single_partition() {
        let data = array![
            [1.0, 10.0],
            [1.0, 10.0],
            [1.0, 10.0],
        ];

        let result = partition_by(&data);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn float_rounding() {
        let data = array![
            [1.0, 10.0],
            [1.0000000001, 10.0],
        ];

        let result = partition_by(&data);

        //To test whether it rounds it up
        assert_eq!(result.len(), 2);
    }

    //Multiple columns
    #[test]
    fn multiple_columns() {
        let data = array![
            [1.0, 2.0, 10.0],
            [1.0, 2.0, 10.0],
            [1.0, 3.0, 30.0],
            [2.0, 2.0, 40.0],
        ];

        let result = partition_by(&data);
        assert_eq!(result.len(), 3);

        // Find the group that has 2 rows (the [1.0, 2.0, 10.0] group)
        assert!(result.iter().any(|g| g.len() == 2));
    }

    //Do we need to write a test case where column order matter?,
    //[Apple, 1] and [1, Apple] do they need to be in separate groups or one?

    //Do we need tests for negative values?
}
