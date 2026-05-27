#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
//otherwise we run into problems with clippy, but actual problems would only occur after 9 quadrillion

use ci_core::ci_tests::PearsonCorrelation;
use ci_core::strategy::CITest;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use ndarray::{Array1, Array2};

// Generate deterministic continuous data using sine/cosine to mock real variables
fn generate_continuous_vector(len: usize, offset: usize) -> Array1<f64> {
    Array1::from_shape_fn(len, |i| ((i + offset) as f64).sin())
}

fn generate_conditioning_matrix(rows: usize, cols: usize) -> Array2<f64> {
    Array2::from_shape_fn((rows, cols), |(i, j)| ((i + j) as f64).cos())
}

fn bench_pearson(
    c: &mut Criterion,
    name: &str,
    x_values: &Array1<f64>,
    y_values: &Array1<f64>,
    z: &Array2<f64>,
) {
    let test = PearsonCorrelation::new(false, 0.05);

    c.bench_function(name, |b| {
        // Because PearsonCorrelation::run_test consumes the arrays (takes them by value),
        // we use iter_batched instead of iter_batched_ref so Criterion clones them
        // outside of the timed loop.
        b.iter_batched(
            || (x_values.clone(), y_values.clone(), z.clone()),
            |(x, y, z_data)| {
                let result = test.run_test(black_box(x), black_box(y), black_box(z_data));

                black_box(result)
            },
            BatchSize::SmallInput,
        );
    });
}

fn benchmark_pearson(c: &mut Criterion) {
    let n = 100;

    let x_values = generate_continuous_vector(n, 0);
    let y_values = generate_continuous_vector(n, 1);

    // Unconditional case (empty matrix)
    let z_empty = Array2::zeros((0, 0));
    // Conditional case with 2 conditioning variables
    let z_cond = generate_conditioning_matrix(n, 2);

    let cases = [
        ("pearson_unconditional", &z_empty),
        ("pearson_conditional", &z_cond),
    ];

    for (name, z) in cases {
        bench_pearson(c, name, &x_values, &y_values, z);
    }
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::from_secs(10))
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = benchmark_pearson
}

criterion_main!(benches);
