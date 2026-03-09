use polars::frame::DataFrame;
use scirs2_core::Array1;

pub enum TestResult {
    Correlated(anyhow::Result<(f64, f64, usize)>),
    Boolean(anyhow::Result<bool>),
}

pub trait CITest {
    fn run_test(
        &self,
        data: &DataFrame,
        col_x: &str,
        col_y: &str,
        cols_z: Array1<String>,
        boolean: bool,
    ) -> anyhow::Result<TestResult>;
}
