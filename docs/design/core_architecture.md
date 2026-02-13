## System Architecture FFI
```mermaid
graph TD
    A[Registry] --> B[Hashmap]
    C[CITest Trait] --> D
    B --> D[CITests]
    D --> E[Pearson test]
    D --> F[Chi square test]
    D --> G[...]
```