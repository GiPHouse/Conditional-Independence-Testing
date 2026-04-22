from pgmpy.ci_tests import PearsonrEquivalence
from ci_python import PyRegistry
import pandas as pd
import numpy as np

# registry = PyRegistry()
# test = registry.get_test("pearson_equivalence")

# df = pd.DataFrame({'X': [1,2,3,4], 'Y': [1,2,1,2], 'Z': [1,1,2,2]})
# pe = PearsonrEquivalence(df)
# statistic, p_value = pe.run_test('X', 'Y', ['Z'])
# print("pgmpy; statistic: ", statistic, ", p_value: ", p_value)
# x = df['X'].to_numpy()
# y = df['Y'].to_numpy()
# z = np.zeros((0, 0))
# print(type(x), type(x).__module__, type(y), type(y).__module__, type(z), type(y).__module__)
# print(z)
# print(f"Rust: {test(z, x, y, boolean = False)}")

registry = PyRegistry()
test = registry.get_test("pearson_equivalence")

rng = np.random.default_rng(seed=42)
size = 1000

df_ind = pd.DataFrame(rng.standard_normal((size, 3)), columns=["X", "Y", "Z"])
array_empty = np.zeros((0, 0))
x_ind = df_ind["X"].to_numpy()
y_ind = df_ind["Y"].to_numpy()

print(f"  Rust  empty Z: {test(array_empty, x_ind, y_ind, boolean=False)}")

pe = PearsonrEquivalence(df_ind)
statistic, p_value = pe.run_test('X', 'Y', [])
print("pgmpy; statistic: ", statistic, ", p_value: ", p_value)

