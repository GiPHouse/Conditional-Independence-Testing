from pgmpy.estimators.CITests import pearsonr, power_divergence
import numpy as np
from ci_python import PyRegistry
import time
import pandas as pd


registry = PyRegistry()
test = registry.get_test("pearson_correlation")
cressie_test = registry.get_test("cressie_read")

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

    # Discrete data for cressie_read (binary to avoid sparse strata).
    dZ1 = rng.integers(0, 2, size)
    dZ2 = rng.integers(0, 2, size)
    dX = (dZ1 ^ rng.integers(0, 2, size)) & 1
    dY = (dZ2 ^ rng.integers(0, 2, size)) & 1
    df_disc = pd.DataFrame({"X": dX, "Y": dY, "Z1": dZ1, "Z2": dZ2})
    array_z_disc = np.column_stack([dZ1, dZ2]).astype(float)
    x_disc = df_disc["X"].to_numpy(dtype=float)
    y_disc = df_disc["Y"].to_numpy(dtype=float)

    return (
        df_ind,
        df_cind,
        array_empty,
        array_z,
        x_ind,
        y_ind,
        x_cond,
        y_cond,
        df_disc,
        array_z_disc,
        x_disc,
        y_disc,
    )


def bench(size):
    rng = np.random.default_rng(seed=42)
    (
        df_ind,
        df_cind,
        array_empty,
        array_z,
        x_ind,
        y_ind,
        x_cond,
        y_cond,
        df_disc,
        array_z_disc,
        x_disc,
        y_disc,
    ) = make_data(size, rng)

    # Warmup
    test(array_empty, x_ind, y_ind, boolean=False)
    test(array_z, x_cond, y_cond, boolean=False)
    pearsonr(X="X", Y="Y", Z=[], data=df_ind, boolean=False)
    pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data=df_cind, boolean=False)

    # Correctness check (single run)
    print(f"  Rust  empty Z: {test(array_empty, x_ind, y_ind, boolean=False)}")
    print(f"  Rust  with Z:  {test(array_z, x_cond, y_cond, boolean=False)}")
    coef, pval = pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data=df_cind, boolean=False)
    print(f"  pgmpy with Z:  ({pval}, {coef})")

    # Cressie-Read (discrete data)
    print(
        f"  Rust  cressie empty Z: {cressie_test(array_empty, x_disc, y_disc, boolean=False)}"
    )
    print(
        f"  Rust  cressie with Z:  {cressie_test(array_z_disc, x_disc, y_disc, boolean=False)}"
    )
    print(
        f"  pgmpy cressie empty Z: {power_divergence(X='X', Y='Y', Z=[], data=df_disc, boolean=False, lambda_='cressie-read')}"
    )
    print(
        f"  pgmpy cressie with Z:  {power_divergence(X='X', Y='Y', Z=['Z1', 'Z2'], data=df_disc, boolean=False, lambda_='cressie-read')}"
    )

    # Benchmark
    t0 = time.perf_counter()
    for _ in range(N_ITER):
        test(array_empty, x_ind, y_ind, boolean=False)
    rust_empty = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearsonr(X="X", Y="Y", Z=[], data=df_ind, boolean=False)
    pgmpy_empty = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        test(array_z, x_cond, y_cond, boolean=False)
    rust_z = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data=df_cind, boolean=False)
    pgmpy_z = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        cressie_test(array_z_disc, x_disc, y_disc, boolean=False)
    rust_cressie = (time.perf_counter() - t0) / N_ITER

    t0 = time.perf_counter()
    for _ in range(N_ITER):
        power_divergence(
            X="X", Y="Y", Z=["Z1", "Z2"], data=df_disc, boolean=False, lambda_="cressie-read"
        )
    pgmpy_cressie = (time.perf_counter() - t0) / N_ITER

    print(
        f"  Empty Z:  Rust={rust_empty*1000:.4f}ms  pgmpy={pgmpy_empty*1000:.4f}ms  speedup={pgmpy_empty/rust_empty:.2f}x"
    )
    print(
        f"  With  Z:  Rust={rust_z*1000:.4f}ms  pgmpy={pgmpy_z*1000:.4f}ms  speedup={pgmpy_z/rust_z:.2f}x"
    )
    print(
        f"  Cressie:  Rust={rust_cressie*1000:.4f}ms  pgmpy={pgmpy_cressie*1000:.4f}ms  speedup={pgmpy_cressie/rust_cressie:.2f}x"
    )


for size in [1_000, 10_000]:
    print(f"\n{'='*60}")
    print(f"N={size:,}  ({N_ITER} iterations)")
    print(f"{'='*60}")
    bench(size)
