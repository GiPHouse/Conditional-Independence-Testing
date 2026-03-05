
import numpy as np
from ci_python import PyRegistry

registry = PyRegistry()
test = registry.get_test("pearson_correlation")

array = np.zeros((0, 0))  # no conditioning variables
x = np.random.randn(200)
y = np.random.randn(200)

result = test(array, x, y, boolean=True)   # returns bool
print(f"This should return true: \n{result}")
result = test(array, x, y, boolean=False)  # returns (p_value, coefficient)
print(f"This should return (p_value, coefficient) \n{result}")
