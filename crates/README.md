# Crates

This directory contains all Rust crates in the CI Testing workspace.

## Structure

- **`ci-core/`**: Core Rust implementation of CI tests
  - Contains the Strategy + Registry pattern
  - All statistical test logic lives here
  - Zero dependencies on FFI or language bindings

- **`ci-python/`**: Python bindings via PyO3
  - Thin wrapper around ci-core
  - Exposes registry API to Python

- **`ci-r/`**: R bindings via extendr
  - Thin wrapper around ci-core
  - Follows R package conventions

- **`ci-js/`**: JavaScript/WebAssembly bindings via wasm-pack
  - Compiled to WASM for browser/Node.js use
  - Asynchronous API for non-blocking execution

## Working with the Workspace

All crates share the same `Cargo.lock` and build directory. To build everything:

```bash
cargo build --workspace
```

To test a specific crate:

```bash
cargo test -p ci-core
```

See the root `Cargo.toml` for workspace configuration.
