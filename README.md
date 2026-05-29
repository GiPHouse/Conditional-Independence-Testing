# Conditional Independence Testing

[![CI](https://img.shields.io/github/actions/workflow/status/GiPHouse/Conditional-Independence-Testing/ci.yml?branch=main&logo=github&label=CI)](https://github.com/GiPHouse/Conditional-Independence-Testing/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/GiPHouse/Conditional-Independence-Testing/blob/main/LICENSE)

A fast, multi-language library for statistical conditional independence testing. The Rust core implements several well-known tests from the power-divergence family (for discrete data) and Pearson-correlation-based tests (for continuous data), with bindings for Python and R.

## Available Tests

| Test | Data type | Output |
|---|---|---|
| `chi_squared` | Discrete | `(p_value, statistic, dof)` |
| `cressie_read` | Discrete | `(p_value, statistic, dof)` |
| `freeman_tukey` | Discrete | `(p_value, statistic, dof)` |
| `log_likelihood` | Discrete | `(p_value, statistic, dof)` |
| `modified_likelihood` | Discrete | `(p_value, statistic, dof)` |
| `pearson_correlation` | Continuous | `(p_value, coefficient)` |
| `pearson_equivalence` | Continuous | `(p_value, coefficient)` |

All tests accept an optional conditioning matrix Z. Pass an empty matrix for unconditional tests. Every test can also run in **boolean mode**, returning only an independence verdict at a given significance level instead of the raw statistic.

## Crate Structure

```
ci-core      Core test implementations and CITest trait (Rust library)
ci-python    PyO3 bindings — importable as ci_python
cir          extendr bindings — importable as an R package
ci-js        wasm-pack bindings
```

Downstream crates depend only on `ci-core`. The bindings crates are thin wrappers that handle type conversion.

## Getting Started

### Rust

Add `ci_core` to your `Cargo.toml`:

```toml
[dependencies]
ci_core = { git = "https://github.com/GiPHouse/Conditional-Independence-Testing" }
```

Run a test:

```rust
use ci_core::ci_tests::chi_squared::ChiSquared;
use ci_core::strategy::{CITest, TestResult};
use ndarray::{Array1, Array2};

let test = ChiSquared::new(/*boolean=*/false, /*significance_level=*/0.05);

let x = Array1::from_vec(vec![0.0, 1.0, 0.0, 1.0, 0.0]);
let y = Array1::from_vec(vec![1.0, 0.0, 1.0, 0.0, 1.0]);
let z = Array2::zeros((0, 0)); // empty conditioning set

match test.run_test(x, y, z)? {
    TestResult::Statistic(p_value, statistic, dof) => {
        println!("p={p_value:.4}, χ²={statistic:.4}, df={dof}");
    }
    _ => unreachable!(),
}
```

For conditional tests, pass a matrix where each column is one conditioning variable:

```rust
use ndarray::{Array2, Axis, stack};

// Condition on two variables z1 and z2
let z = stack(Axis(1), &[z1.view(), z2.view()])?;
test.run_test(x, y, z)?;
```

### Python

Build and install the Python package from the repository root:

```bash
pip install maturin
maturin develop -m crates/ci-python/Cargo.toml
```
Created a Box structure for existing CI tests:
```
let inner: Box<dyn CITestTrait> = match name {
            "chi_squared" => Box::new(ChiSquared::new(boolean, significance_level)),
            "log_likelihood" => Box::new(LogLikelihood::new(boolean, significance_level)),
            "cressie_read" => Box::new(CressieRead::new(boolean, significance_level)),
            "pearson_correlation" => Box::new(PearsonCorrelation::new(boolean, significance_level)),
            "freeman_tukey" => Box::new(FreemanTukey::new(boolean, significance_level)),
            "modified_likelihood" => Box::new(ModifiedLikelihood::new(boolean, significance_level)),
            "pearson_equivalence" => Box::new(PearsonEquivalence::new(
                boolean,
                significance_level,
                delta_threshold,
            )),
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown test: '{name}'"
                )))
            }
        };
```
Results can be tested as:

```python
use ci_core::strategy::CITest as CITestTrait;
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

pub fn __call__(
        &self,
        py: Python<'_>,
        x: PyReadonlyArray1<'_, f64>,
        y: PyReadonlyArray1<'_, f64>,
        z: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<Py<PyAny>> {
        let result = self
            .inner
            .run_test(
                x.as_array().to_owned(),
                y.as_array().to_owned(),
                z.as_array().to_owned(),
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        test_result_to_pyobj(&result, py)
    }
```

### R

Install the R package from the repository root:

```r
# From within R, using devtools
devtools::install("crates/cir")
```

```r
library(cir)

x <- c(0, 1, 0, 1, 0)
y <- c(1, 0, 1, 0, 1)
z <- matrix(nrow = length(x), ncol = 0)  # unconditional

result <- chi_squared_test(x, y, z, boolean = FALSE, significance_level = 0.05)
cat("p =", result$p_value, " statistic =", result$statistic, "\n")
```

Boolean mode returns only an independence verdict:

```r
result <- pearson_correlation_test(x, y, z, boolean = TRUE, significance_level = 0.05)
cat("independent:", result$independent, "\n")
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions, coding standards, and how to add a new CI test.

Quick check before opening a PR:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## License

Licensed under the [MIT license](LICENSE).
