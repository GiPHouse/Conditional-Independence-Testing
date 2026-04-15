from collections.abc import Callable

import numpy as np

class Registry:
    def __init__(self) -> None: ...
    def get_test(
        self, test_name: str
    ) -> Callable[
        [
            np.ndarray[tuple[int], np.dtype[np.float64]],
            np.ndarray[tuple[int], np.dtype[np.float64]],
            np.ndarray[tuple[int, int], np.dtype[np.float64]],
            bool,
        ],
        bool | tuple[float, float] | tuple[float, float, float],
    ]: ...
    def list_all(self) -> list[str]: ...
