from pgmpy.estimators.CITests import (
   pearsonr
) 
import numpy as np
from ci_python import PyRegistry
import time
import pandas as pd


registry = PyRegistry()
test = registry.get_test("pearson_correlation")

# array = np.zeros((0, 0))  # no conditioning variables
# x = np.random.randn(200)
# y = np.random.randn(200)
rng = np.random.default_rng(seed=42)

df_ind = pd.DataFrame(np.random.randn(200, 3), columns=["X", "Y", "Z"])

array = np.zeros((0, 0))

x = df_ind["X"].to_numpy()

y = df_ind["Y"].to_numpy()

time_start = time.perf_counter()
result = test(array, x, y, boolean=True)   # returns bool
print(f"This should return true: \n{result}")
result = test(array, x, y, boolean=False)  # returns (p_value, coefficient)
print(f"This should return (p_value, coefficient) \n{result}")
time_end = time.perf_counter()

duration = time_end-time_start

print(f"Total duration Rust implementation: {duration}s")

time_start2 = time.perf_counter()
result2 = pearsonr(X="X", Y="Y", Z=[], data = df_ind, significance_level=0.05)
print(f"This should return true: \n{result2}")
coef2, p_value2 = pearsonr(X="X", Y="Y", Z=[], data = df_ind, boolean=False)
print(f"This should return (p_value, coefficient) \n({p_value2}, {coef2})")
time_end2 = time.perf_counter()

duration2 = time_end2-time_start2

print(f"Total duration Pythone implementation: {duration2}s")

