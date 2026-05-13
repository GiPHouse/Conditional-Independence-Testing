from pgmpy.estimators.CITests import pearsonr, power_divergence
from pgmpy.ci_tests import PearsonrEquivalence
import numpy as np
from ci_python import PyRegistry
import time
import pandas as pd


registry = PyRegistry()

N_ITER = 50


def make_data(size, rng):
    df_ind = pd.DataFrame(rng.standard_normal((size, 3)), columns=["X", "Y", "Z"])
    Z1 = rng.normal(0, 1, size)
    Z2 = rng.normal(0, 1, size)
    X = 3 * Z1 + 2 * Z2 + rng.normal(0, 0.1, size)
    Y = 2 * Z1 + 3 * Z2 + rng.normal(0, 0.1, size)
    df_cind = pd.DataFrame({"X": X, "Y": Y, "Z1": Z1, "Z2": Z2})

    array_empty = np.zeros((0, 0))
    array_z = np.column_stack([Z1, Z2])
    x_ind = df_ind["X"].to_numpy()
    y_ind = df_ind["Y"].to_numpy()
    x_cond = df_cind["X"].to_numpy()
    y_cond = df_cind["Y"].to_numpy()

    return df_ind, df_cind, array_empty, array_z, x_ind, y_ind, x_cond, y_cond

# ---------------------------------------------------------------------------
# Pearson correlation bench
# ---------------------------------------------------------------------------

pearsonr_test = registry.get_test("pearson_correlation")

def bench_pearsonr(size):
    rng = np.random.default_rng(seed=42)
    df_ind, df_cind, array_empty, array_z, x_ind, y_ind, x_cond, y_cond = make_data(
        size, rng
    )

    # Warmup
    pearsonr_test(array_empty, x_ind, y_ind)
    pearsonr_test(array_z, x_cond, y_cond)
    pearsonr(X="X", Y="Y", Z=[], data=df_ind, boolean=False)
    pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data=df_cind, boolean=False)

    # Correctness check (single run)
    print(f"  Rust  empty Z: {pearsonr_test(array_empty, x_ind, y_ind)}")
    print(f"  Rust  with Z:  {pearsonr_test(array_z, x_cond, y_cond)}")
    coef, pval = pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data=df_cind, boolean=False)
    print(f"  pgmpy with Z:  ({pval}, {coef})")

    # Benchmark
    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearsonr_test(array_empty, x_ind, y_ind)
    rust_empty = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearsonr(X="X", Y="Y", Z=[], data=df_ind, boolean=False)
    pgmpy_empty = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearsonr_test(array_z, x_cond, y_cond)
    rust_z = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data=df_cind, boolean=False)
    pgmpy_z = (time.perf_counter() - t0) / N_ITER

    print(
        f"  Empty Z:  Rust={rust_empty*1000:.4f}ms  pgmpy={pgmpy_empty*1000:.4f}ms  speedup={pgmpy_empty/rust_empty:.2f}x"
    )
    print(
        f"  With  Z:  Rust={rust_z*1000:.4f}ms  pgmpy={pgmpy_z*1000:.4f}ms  speedup={pgmpy_z/rust_z:.2f}x"
    )


for size in [1_000, 10_000]:
    print(f"\n{'='*60}")
    print(f"N={size:,}  ({N_ITER} iterations)")
    print(f"{'='*60}")
    bench_pearsonr(size)

# ---------------------------------------------------------------------------
# Pearson equivalence bench
# ---------------------------------------------------------------------------

pearson_equivalence_test = registry.get_test("pearson_equivalence")

def bench_pearson_equivalence(size):
    rng = np.random.default_rng(seed=42)
    df_ind, df_cind, array_empty, array_z, x_ind, y_ind, x_cond, y_cond = make_data(
        size, rng
    )

    # Warmup
    pearson_equivalence_test(array_empty, x_ind, y_ind)
    pearson_equivalence_test(array_z, x_cond, y_cond)

    pearson_eqvuivalence_ind = PearsonrEquivalence(df_ind)
    pearson_eqvuivalence_cind = PearsonrEquivalence(df_cind)
    pearson_eqvuivalence_ind.run_test(X="X", Y="Y", Z=[])
    pearson_eqvuivalence_cind.run_test(X="X", Y="Y", Z=["Z1", "Z2"])

    # Correctness check (single run)
    print(f" Rust pearson equivalence empty Z: {pearson_equivalence_test(array_empty, x_ind, y_ind)}")
    print(f" Rust pearson equivalence with Z:  {pearson_equivalence_test(array_z, x_cond, y_cond)}")
    coef, pval = pearson_eqvuivalence_cind.run_test(X="X", Y="Y", Z=["Z1", "Z2"])
    print(f"  pgmpy pearson equivalence with Z:  ({pval}, {coef})")

    # Benchmark
    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearson_equivalence_test(array_empty, x_ind, y_ind)
    rust_empty = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearson_eqvuivalence_ind.run_test(X="X", Y="Y", Z=[])
    pgmpy_empty = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearson_equivalence_test(array_z, x_cond, y_cond)
    rust_z = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearson_eqvuivalence_cind.run_test(X="X", Y="Y", Z=["Z1", "Z2"])
    pgmpy_z = (time.perf_counter() - t0) / N_ITER

    print(
        f" Pearson equivalence empty Z:  Rust={rust_empty*1000:.4f}ms  pgmpy={pgmpy_empty*1000:.4f}ms  speedup={pgmpy_empty/rust_empty:.2f}x"
    )
    print(
        f" Pearson equivalence with  Z:  Rust={rust_z*1000:.4f}ms  pgmpy={pgmpy_z*1000:.4f}ms  speedup={pgmpy_z/rust_z:.2f}x"
    )


for size in [1_000, 10_000]:
    print(f"\n{'='*60}")
    print(f"N={size:,}  ({N_ITER} iterations)")
    print(f"{'='*60}")
    bench_pearson_equivalence(size)

# ---------------------------------------------------------------------------
# Scaling benchmark: Cressie-Read with increasing discrete conditioning vars
# ---------------------------------------------------------------------------
cressie_test = registry.get_test("cressie_read")


def bench_scaling(size, n_z_vars, n_iter=20):
    rng = np.random.default_rng(seed=42)
    z_cols = {f"Z{i}": rng.integers(0, 2, size) for i in range(n_z_vars)}
    X = (sum(z_cols.values()) + rng.integers(0, 2, size)) % 3
    Y = (sum(z_cols.values()) + rng.integers(0, 2, size)) % 3
    df = pd.DataFrame({"X": X, "Y": Y, **z_cols})
    array_z = np.column_stack(list(z_cols.values())).astype(float)
    x = df["X"].to_numpy(dtype=float)
    y = df["Y"].to_numpy(dtype=float)
    z_names = list(z_cols.keys())

    # Warmup
    cressie_test(array_z, x, y)
    power_divergence(
        X="X", Y="Y", Z=z_names, data=df, boolean=False, lambda_="cressie-read"
    )

    t0 = time.perf_counter()
    for _ in range(n_iter):
        cressie_test(array_z, x, y)
    rust_t = (time.perf_counter() - t0) / n_iter

    t0 = time.perf_counter()
    for _ in range(n_iter):
        power_divergence(
            X="X", Y="Y", Z=z_names, data=df, boolean=False, lambda_="cressie-read"
        )
    pgmpy_t = (time.perf_counter() - t0) / n_iter

    print(
        f"  |Z|={n_z_vars:2d}  Rust={rust_t*1000:8.4f}ms  pgmpy={pgmpy_t*1000:8.4f}ms  speedup={pgmpy_t/rust_t:6.2f}x"
    )


print(f"\n{'='*60}")
print("Cressie-Read scaling benchmark: N=10,000, increasing |Z|")
print(f"{'='*60}")
for n_z in [2, 4, 6, 8, 10, 15]:
    bench_scaling(10_000, n_z)
