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
Z1 = rng.normal(loc=0.0, scale=1.0, size=200)
Z2 = rng.normal(loc=0.0, scale=1.0, size=200)
X = 3 * Z1 + 2 * Z2 + rng.normal(loc=0, scale=0.1, size=200)
Y = 2 * Z1 + 3 * Z2 + rng.normal(loc=0, scale=0.1, size=200)
df_cind_mul = pd.DataFrame({"X": X, "Y": Y, "Z1": Z1, "Z2": Z2})

array = np.zeros((0, 0))

x = df_ind["X"].to_numpy()

y = df_ind["Y"].to_numpy()

array2 = np.column_stack([Z1, Z2])

x_2 = df_cind_mul["X"].to_numpy()

y_2 = df_cind_mul["Y"].to_numpy()

time_start = time.perf_counter()
result = test(array, x, y, boolean=True)   # returns bool
print(f"This should return true: \n{result}")
result2 = test(array, x, y, boolean=False)  # returns (p_value, coefficient)
print(f"This should return (p_value, coefficient) \n{result2}")
time_end = time.perf_counter()

duration = time_end-time_start

print(f"Total duration Rust1 implementation: {duration}s")

time_start2 = time.perf_counter()
result3 = pearsonr(X="X", Y="Y", Z=[], data = df_ind, significance_level=0.05)
print(f"This should return true: \n{result3}")
coef2, p_value2 = pearsonr(X="X", Y="Y", Z=[], data = df_ind, boolean=False)
print(f"This should return (p_value, coefficient) \n({p_value2}, {coef2})")
time_end2 = time.perf_counter()

duration2 = time_end2-time_start2

print(f"Total duration Python1 implementation: {duration2}s")


time_start3 = time.perf_counter()
result4 = test(array2, x_2, y_2, boolean=True)   # returns bool
print(f"This should return true: \n{result4}")
result5 = test(array2, x_2, y_2, boolean=False)  # returns (p_value, coefficient)
print(f"This should return (p_value, coefficient) \n{result5}")
time_end3 = time.perf_counter()

duration3 = time_end3-time_start3

print(f"Total duration Rust implementation: {duration3}s")

time_start4 = time.perf_counter()
result6 = pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data = df_cind_mul, significance_level=0.05)
print(f"This should return true: \n{result6}")
coef3, p_value3 = pearsonr(X="X", Y="Y", Z=["Z1", "Z2"], data = df_cind_mul, boolean=False)
print(f"This should return (p_value, coefficient) \n({p_value3}, {coef3})")
time_end4 = time.perf_counter()

duration4 = time_end4-time_start4

print(f"Total duration Pythone implementation: {duration4}s")

# print("Running test")
   
# all_tests = registry.list_all_tests()
# print(all_tests)