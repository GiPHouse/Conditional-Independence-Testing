from _ci_python import _RustRegistry  # type: ignore[reportAttributeAccessIssue] # noqa: D104

from .CITests import ci_registry  # noqa: D104

__all__ = [
    "_RustRegistry",
    "ci_registry"
]
