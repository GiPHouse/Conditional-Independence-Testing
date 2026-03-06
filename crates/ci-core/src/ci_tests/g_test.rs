use crate::strategy::CITest;
use polars::frame::DataFrame;
use scirs2_core::Array1;

pub struct GTest {
    // Object traits
}

impl CITest for GTest {
    fn run_test(
        &self,
        data: &DataFrame,
        col_x: &str,
        col_y: &str,
        cols_z: Array1<&str>,
    ) -> anyhow::Result<(), anyhow::Error> {
        Ok(())
    }
    //Other necessary stuff
}
