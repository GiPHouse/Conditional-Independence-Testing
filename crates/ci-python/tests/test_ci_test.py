import numpy as np
import pytest
import ci_python


@pytest.fixture
def registry():
    return ci_python.PyRegistry()


@pytest.fixture
def ci_test(registry):
    # IMPORTANT: PyCITest is constructed here, not configured here
    return registry.get_test("cressie_read")


# ----------------------------
# 1. Smoke test: callable FFI boundary
# ----------------------------
def test_pycitest_is_callable(ci_test):
    assert callable(ci_test)


# ----------------------------
# 2. Statistic return path (3-tuple)
# ----------------------------
def test_pycitest_statistic_output_shape(ci_test):
    x = np.array([1., 1., 2., 2., 1., 1., 2., 2.])
    y = np.array([1., 2., 1., 2., 1., 2., 1., 2.])
    z = np.zeros((8, 2), dtype=np.float64)

    result = ci_test(z, x, y)
    print(result)
    assert isinstance(result, tuple)
    assert len(result) == 3

    p_value, statistic, dof = result

    assert isinstance(p_value, float)
    assert isinstance(statistic, float)
    assert isinstance(dof, int)