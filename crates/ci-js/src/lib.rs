use ci_core::strategy::TestResult;
use js_sys::{Array, Float64Array};
use ndarray::{Array1, Array2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JSCITest {}

#[wasm_bindgen]
impl JSCITest {
    #[wasm_bindgen(js_name = "run_test")]
    pub fn run_test(
        &self,
        z_flat: &Float64Array,
        z_rows: usize,
        z_cols: usize,
        x: &Float64Array,
        y: &Float64Array,
        boolean: bool,
        significance_level: f64,
    ) -> Result<JsValue, JsValue> {
        //Converting Float64Array -> Vec<f64> ->  ndarray
        let z_vec: Vec<f64> = z_flat.to_vec();
        let x_vec: Vec<f64> = x.to_vec();
        let y_vec: Vec<f64> = y.to_vec();

        let z: Array2<f64> = Array2::from_shape_vec((z_rows, z_cols), z_vec)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let x: Array1<f64> = Array1::from_vec(x_vec);
        let y: Array1<f64> = Array1::from_vec(y_vec);

        let test = PearsonCorrelation::new(&boolean, &significance_level);

        let result = test
            .run_test(x, y, z)
            .map_err(|e| JsError::new(&e.to_string()))?;

        match result {
            TestResult::Boolean(b) => Ok(JsValue::from_bool(b)),
            TestResult::PValue(p_value, coefficient) => {
                let array = Array::new();
                array.push(&JsValue::from_f64(p_value));
                array.push(&JsValue::from_f64(coefficient));
                Ok(array.into())
            }

            TestResult::Statistic(p_value, statistic, dof) => {
                let array = Array::new();
                array.push(&JsValue::from_f64(p_value));
                array.push(&JsValue::from_f64(statistic));
                array.push(&JsValue::from_f64(dof as f64));
                Ok(array.into())
            }
        }
    }
}
