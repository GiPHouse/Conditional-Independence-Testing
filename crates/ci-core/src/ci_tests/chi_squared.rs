use polars::frame::DataFrame;
use scirs2_core::Array1;

use crate::strategy::CITest;

pub struct ChiSquared {
    // Object traits
}

impl CITest for ChiSquared {
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
