use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::array;
use ci_core::utils::contingency_test::contingency_test;

fn benchmark_contingency_test(c: &mut Criterion) {
    let observed = array![
        [10.0, 20.0, 30.0],
        [6.0,  9.0,  17.0]
    ];

    let lambda_values = vec![0.0, -1.0, 2.0]; // G-test, modified, Cressie-Read

    for &lambda in &lambda_values {
        let bench_id = format!("contingency_test_lambda_{}", lambda);

        c.bench_function(&bench_id, |b| {
            b.iter(|| {
                let result = contingency_test(
                    black_box(&observed),
                    black_box(lambda),
                );
                black_box(result)
            })
        });
    }
}

criterion_group!(benches, benchmark_contingency_test);
criterion_main!(benches);