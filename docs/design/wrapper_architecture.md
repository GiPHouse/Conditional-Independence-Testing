## System Architecture Wrapper

```mermaid
graph TD
    A[Python package] -->|PyO3| D[Python bindings]
    B[R package] -->|extendr| E[R bindings]
    C[Javascript package] -->|wasm-pack| F[Java bindings]
    D --> G[Rust]
    E --> G
    F --> G
```
