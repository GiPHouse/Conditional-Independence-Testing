import numpy as np
import pytest
import ci_python


# Helper function to evaluate contents of numeric tuple returned by various functions
def check_numeric_results(result, expected_stat_approx_zero=False, expected_dof=None, significance_level=0.05):
    assert isinstance(result, tuple)
    assert len(result) == 3

    p_value, statistic, dof = result

    assert isinstance(p_value, float)
    assert isinstance(statistic, float)
    assert isinstance(dof, (int, float))  # Accept float if PyO3 casted usize dynamically

    if expected_dof is not None:
        assert int(dof) == expected_dof
    if expected_stat_approx_zero:
        assert abs(statistic) < 1e-9, f"expected statistic ~0, got {statistic}"
        assert p_value > 0.99, f"expected p ~1, got {p_value}"
    else:
        assert statistic > 5.0, f"expected large statistic, got {statistic}"
        assert p_value < significance_level, f"expected p < {significance_level}, got {p_value}"


def test_unconditional_independent_data_is_not_rejected():
    x = np.array([1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0], dtype=np.float64)
    y = np.array([1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0], dtype=np.float64)
    empty_z = np.zeros((0, 0), dtype=np.float64)

    # Numeric Mode
    test_numeric = ci_python.CITest("cressie_read", boolean=False, significance_level=0.05)
    result_numeric = test_numeric(x, y, empty_z)
    check_numeric_results(result_numeric, expected_stat_approx_zero=True, expected_dof=1)

    # Boolean Mode
    test_boolean = ci_python.CITest("cressie_read", boolean=True, significance_level=0.05)
    result_boolean = test_boolean(x, y, empty_z)
    assert isinstance(result_boolean, bool)
    assert result_boolean is True, "expected fail-to-reject (independent=True)"


def test_unconditional_dependent_data_is_rejected():
    x = np.array([1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0], dtype=np.float64)
    y = np.array([1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0], dtype=np.float64)
    empty_z = np.zeros((0, 0), dtype=np.float64)

    # Numeric Mode
    test_numeric = ci_python.CITest("cressie_read", boolean=False, significance_level=0.05)
    result_numeric = test_numeric(x, y, empty_z)
    check_numeric_results(result_numeric, expected_stat_approx_zero=False)

    # Boolean Mode
    test_boolean = ci_python.CITest("cressie_read", boolean=True, significance_level=0.05)
    result_boolean = test_boolean(x, y, empty_z)
    assert isinstance(result_boolean, bool)
    assert result_boolean is False, "expected reject (independent=False)"


def test_conditional_independent_per_group():
    x = np.array([1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0], dtype=np.float64)
    y = np.array([1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0], dtype=np.float64)
    z = np.array([[0.0], [0.0], [0.0], [0.0], [1.0], [1.0], [1.0], [1.0]], dtype=np.float64)

    # Numeric Mode
    test_numeric = ci_python.CITest("cressie_read", boolean=False, significance_level=0.05)
    result_numeric = test_numeric(x, y, z)
    check_numeric_results(result_numeric, expected_stat_approx_zero=True, expected_dof=2)

    # Boolean Mode
    test_boolean = ci_python.CITest("cressie_read", boolean=True, significance_level=0.05)
    result_boolean = test_boolean(x, y, z)
    assert isinstance(result_boolean, bool)
    assert result_boolean is True, "expected conditional independence to hold"


def test_conditional_dependent_per_group():
    x = np.array([1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0], dtype=np.float64)
    y = np.array([1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0], dtype=np.float64)
    z = np.array([[0.0], [0.0], [0.0], [0.0], [1.0], [1.0], [1.0], [1.0]], dtype=np.float64)

    # Numeric Mode
    test_numeric = ci_python.CITest("cressie_read", boolean=False, significance_level=0.05)
    result_numeric = test_numeric(x, y, z)
    check_numeric_results(result_numeric, expected_stat_approx_zero=False)


def test_error_handling_zero_observations():
    x = np.array([1.0, 2.0], dtype=np.float64)
    y = np.array([1.0, 2.0], dtype=np.float64)
    z = np.array([[]], dtype=np.float64)

    test_numeric = ci_python.CITest("modified_likelihood", boolean=False, significance_level=0.05)

    with pytest.raises(RuntimeError):
        test_numeric(x, y, z)
