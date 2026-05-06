#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TestRequest {
    pub z: Vec<Vec<f64>>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub boolean: bool,
    pub significance_level: f64,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TestResponse {
    Boolean {
        boolean: bool,
    },
    PValue {
        p_value: f64,
        coefficient: f64,
    },
    Statistics {
        p_value: f64,
        coefficient: f64,
        dof: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_request_succeeds_with_all_fields() {
        let input = json!({
            "x": [1.0, 2.0, 3.0],
            "y": [4.0, 5.0, 6.0],
            "boolean": false,
            "significance_level": 0.05
        });
        let req: TestRequest = from_value(input).unwrap();
        assert_eq!(req.boolean, false);
        assert!((req.significance_level - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_request_deserializes_with_z() {
        let input = json!({
            "z": [[1.0, 2.0], [3.0, 4.0]],
            "x": [1.0, 2.0, 3.0],
            "y": [4.0, 5.0, 6.0],
            "boolean": true,
            "significance_level": 0.05
        });
        let req: TestRequest = from_value(input).unwrap();
        assert_eq!(req.x, vec![1.0, 2.0, 3.0]);
        assert_eq!(req.y, vec![4.0, 5.0, 6.0]);
        assert_eq!(req.z, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(req.boolean, true);
        assert_eq!(req.significance_level, 0.05);
    }

    #[test]
    fn test_request_deserializes_without_z() {
        let input = json!({
            "x": [1.0, 2.0, 3.0],
            "y": [4.0, 5.0, 6.0],
            "boolean": true,
            "significance_level": 0.05
        });
        let req: TestRequest = from_value(input).unwrap();
        assert!(req.z.is_empty());
    }

    #[test]
    fn test_request_deserializes_with_null_z() {
        let input = json!({
            "z": null,
            "x": [1.0, 2.0, 3.0],
            "y": [4.0, 5.0, 6.0],
            "boolean": true,
            "significance_level": 0.05
        });
        let req: TestRequest = from_value(input).unwrap();
        assert!(req.z.is_empty());
    }

    #[test]
    fn test_request_fails_without_x() {
        let input = json!({
            "y": [1.0, 2.0, 3.0]
        });
        assert!(from_value::<TestRequest>(input).is_err());
    }

    #[test]
    fn test_request_fails_without_y() {
        let input = json!({
            "x": [1.0, 2.0, 3.0]
        });
        assert!(from_value::<TestRequest>(input).is_err());
    }

    #[test]
    fn test_request_fails_with_wrong_type() {
        let input = json!({
            "x": "not_an_array",
            "y": [1.0, 2.0, 3.0]
        });
        assert!(from_value::<TestRequest>(input).is_err());
    }

    #[test]
    fn test_request_fails_without_boolean() {
        let input = json!({
            "x": [1.0, 2.0, 3.0],
            "y": [4.0, 5.0, 6.0],
            "significance_level": 0.05
        });
        assert!(from_value::<TestRequest>(input).is_err());
    }

    #[test]
    fn test_request_fails_without_significance_level() {
        let input = json!({
            "x": [1.0, 2.0, 3.0],
            "y": [4.0, 5.0, 6.0],
            "boolean": false
        });
        assert!(from_value::<TestRequest>(input).is_err());
    }

    #[test]
    fn test_result_boolean_serializes() {
        let result = TestResponse::Boolean { boolean: true };
        let json = to_value(&result).unwrap();
        println!("Value Json {}", json);
        assert_eq!(json["boolean"], true);
    }

    #[test]
    fn test_result_pval_serializes() {
        let result = TestResponse::PValue {
            p_value: 0.03,
            coefficient: 0.85,
        };
        let json = to_value(&result).unwrap();
        assert!((json["p_value"].as_f64().unwrap() - 0.03).abs() < 1e-10);
        assert!((json["coefficient"].as_f64().unwrap() - 0.85).abs() < 1e-10);
    }

    #[test]
    fn test_result_statistics_serializes() {
        let result = TestResponse::Statistics {
            p_value: 0.01,
            coefficient: 0.9,
            dof: 5,
        };
        let json = to_value(&result).unwrap();
        assert!((json["p_value"].as_f64().unwrap() - 0.01).abs() < 1e-10);
        assert!((json["coefficient"].as_f64().unwrap() - 0.9).abs() < 1e-10);
        assert_eq!(json["dof"], 5);
    }

    #[test]
    fn test_result_does_not_serialize_nan() {
        let result = TestResponse::PValue {
            p_value: f64::NAN,
            coefficient: 0.5,
        };
        let json = to_value(&result).unwrap();
        // serde_json serializes NaN as null — verify you catch this in your handler
        assert!(json["p_value"].is_null());
    }
}
