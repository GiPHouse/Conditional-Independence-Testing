from collections.abc import Callable, Hashable  # noqa: D100

import pandas as pd

from . import _RustRegistry  # type: ignore[reportAttributeAccessIssue]


class CITestRegistry:
    """Registry to manage Conditional Independence (CI) Test Strategies.

    Allows looking up tests by name or inferring suitable tests based on data type.
    """

    _rust_registry: _RustRegistry

    def __init__(self) -> None:
        """Initialise `CITestRegistry`."""
        self._rust_registry = _RustRegistry()

    def list_all(self, data_type: list[str] | None = None) -> list[str]:
        """List all registered CI test strategies.

        Parameters
        ----------
        data_type : str, optional
            If provided, filters tests that support the given data type.

        Returns:
        -------
        list of str
            Names of all registered CI tests.

        """
        if data_type is not None:
            raise NotImplementedError

        raise NotImplementedError

    def get_test(
        self, test: str | None | Callable, data: pd.DataFrame | None = None
    ) -> Callable[
        [
            int | str | Hashable,
            int | str | Hashable,
            list[int | str | Hashable],
            pd.DataFrame,
            bool,
            float,
        ],
        bool | tuple[float, float] | tuple[float, float, float],
    ]:
        """Retrieve a CI test strategy.

        Parameters
        ----------
        test : str, callable or None
            The name of the test, a callable function, or None.

        data : pandas.DataFrame, optional
            The dataframe used to infer the test type if `test` is None.

        Returns:
        -------
        callable
            The CI test function.

        Raises:
        ------
        ValueError
            If `test` is None and `data` is None, or if the test name is not found.

        """
        # Case 1: Test is already a function/strategy
        if callable(test):
            return test

        # Case 2: Test is None, infer from data
        if test is None:
            if data is None:
                raise ValueError("Cannot determine a suitable CI test as data is None. Please specify CI test to use.")
            raise NotImplementedError

        # Case 3: Test is a string name
        if isinstance(test, str):
            rust_test = self._rust_registry.get_test(test)

            def wrapper(
                X: int | str | Hashable,
                Y: int | str | Hashable,
                Z: list[int | str | Hashable],
                data: pd.DataFrame,
                boolean: bool,
                significance_level: float,  # TODO: How to handle this? In Rust or Python?
            ) -> bool | tuple[float, float] | tuple[float, float, float]:
                data_x = data[X].to_numpy()
                data_y = data[Y].to_numpy()
                data_z = data.loc[:, Z].to_numpy()
                return rust_test(data_z, data_x, data_y, boolean)

            return wrapper


ci_registry = CITestRegistry()
