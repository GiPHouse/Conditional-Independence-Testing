use polars::frame::DataFrame;
use scirs2_core::Array1;

pub trait CITest {
    fn run_test(
        &self,
        data: &DataFrame,
        col_x: &str,
        col_y: &str,
        cols_z: Array1<&str>,
    ) -> anyhow::Result<()>;
}
