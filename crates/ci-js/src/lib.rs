use ci_core::ci_tests::{
    chi_squared::ChiSquared, cressie_read::CressieRead, freeman_tukey::FreemanTukey,
    log_likelihood::LogLikelihood, modified_likelihood::ModifiedLikelihood,
    pearson_correlation::PearsonCorrelation, pearson_equivalence::PearsonEquivalence,
};
use ci_core::strategy::{CITest, TestResult};
use js_sys::{Array, Float64Array};
use ndarray::{Array1, Array2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

macro_rules! wasm_ci_test {
    ($fn_name:ident, $inner:ty) => {
        #[wasm_bindgen]
        pub fn $fn_name(
            z_flat: &Float64Array,
            z_rows: usize,
            z_cols: usize,
            x: &Float64Array,
            y: &Float64Array,
            boolean: bool,
            significance_level: f64,
        ) -> Result<JsValue, JsValue> {
            let (x, y, z) = convert_to_ndarray(z_flat, x, y, z_rows, z_cols)?;
            let test = <$inner>::new(boolean, significance_level);
            let result = test
                .run_test(x, y, z)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            convert_to_jsvalue(result)
        }
    };
}

wasm_ci_test!(chi_squared_test, ChiSquared);
wasm_ci_test!(log_likelihood_test, LogLikelihood);
wasm_ci_test!(cressie_read_test, CressieRead);
wasm_ci_test!(pearson_correlation_test, PearsonCorrelation);
wasm_ci_test!(freeman_tukey_test, FreemanTukey);
wasm_ci_test!(modified_likelihood_test, ModifiedLikelihood);

#[wasm_bindgen]
/// Pearson equivalence CI test (TOST): declares independence when the partial correlation
/// lies within `[-delta_threshold, delta_threshold]`.
///
/// Pass a 0-column matrix for `z` to run unconditionally. When `boolean` is `true`,
/// returns an independence verdict instead of the raw p-value and correlation.
pub fn pearson_equivalence_test(
    z_flat: &Float64Array,
    z_rows: usize,
    z_cols: usize,
    x: &Float64Array,
    y: &Float64Array,
    boolean: bool,
    significance_level: f64,
    delta_threshold: f64,
) -> Result<JsValue, JsValue> {
    // Convert JavaScript Float64Array -> Vec<f64> -> ndarray:Array
    let (x, y, z) = convert_to_ndarray(z_flat, x, y, z_rows, z_cols)?;
    let citest = PearsonEquivalence::new(boolean, significance_level, delta_threshold);

    let result = citest
        .run_test(x, y, z)
        .map_err(|e| JsError::new(&e.to_string()))?;

    convert_to_jsvalue(result)
}

fn convert_to_ndarray(
    z_flat: &Float64Array,
    x: &Float64Array,
    y: &Float64Array,
    z_rows: usize,
    z_cols: usize,
) -> Result<(Array1<f64>, Array1<f64>, Array2<f64>), JsValue> {
    let z_vec: Vec<f64> = z_flat.to_vec();
    let x_vec: Vec<f64> = x.to_vec();
    let y_vec: Vec<f64> = y.to_vec();

    let z: Array2<f64> = if z_vec.is_empty() {
        Array2::zeros((x_vec.len(), 0)) // correct empty shape
    } else {
        Array2::from_shape_vec((z_rows, z_cols), z_vec)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
    };
    let x: Array1<f64> = Array1::from_vec(x_vec);
    let y: Array1<f64> = Array1::from_vec(y_vec);

    Ok((x, y, z))
}

fn convert_to_jsvalue(result: TestResult) -> Result<JsValue, JsValue> {
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
