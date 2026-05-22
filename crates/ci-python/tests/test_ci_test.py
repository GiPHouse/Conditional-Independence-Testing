import numpy as np
import pytest
import ci_python


@pytest.fixture
def independent_data():
    rng = np.random.default_rng(67)

    x = rng.integers(0, 2, size=200).astype(float)
    y = rng.integers(0, 2, size=200).astype(float)

    # unconditional test -> 0-column matrix
    z = np.empty((200, 0), dtype=float)

    return x, y, z


@pytest.fixture
def correlated_data():
    rng = np.random.default_rng(67)

    x = rng.normal(size=200)
    y = x + rng.normal(scale=0.01, size=200)

    z = np.empty((200, 0), dtype=float)

    return x.astype(float), y.astype(float), z


DISCRETE_TESTS = [
    "chi_squared", 
    "log_likelihood", 
    "cressie_read", 
    "pearson_correlation", 
    "freeman_tukey", 
    "modified_likelihood"

]


@pytest.mark.parametrize("test_fn", DISCRETE_TESTS)
def test_numeric_mode_returns_statistic(test_fn, independent_data):
    test = ci_python.CITest(test_fn)
    x,y,z=independent_data
    result = test(x,y,z)
    print(result)
    assert isinstance(result, tuple)
    assert isinstance(result[0], float)


