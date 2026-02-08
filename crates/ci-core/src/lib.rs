Public API surface

Something like:

┌─────────────────────────────────────────────┐
│         Your Library (ci-core)              │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │   PUBLIC API SURFACE                  │ │
│  │   (everything marked `pub`)           │ │
│  │                                       │ │
│  │   • TestRegistry struct               │ │
│  │   • TestRegistry::new()               │ │
│  │   • TestRegistry::register()          │ │
│  │   • CITest trait                      │ │
│  │   • TestResult type                   │ │
│  │                                       │ │
│  └───────────────────────────────────────┘ │
│              ↑                              │
│              │ Users interact through this  │
│              │                              │
│  ┌───────────────────────────────────────┐ │
│  │   PRIVATE IMPLEMENTATION              │ │
│  │   (no `pub`, or `pub(crate)`)         │ │
│  │                                       │ │
│  │   • Internal HashMap                  │ │
│  │   • Helper functions                  │ │
│  │   • Validation logic                  │ │
│  │   • Optimization tricks               │ │
│  │                                       │ │
│  └───────────────────────────────────────┘ │
│                                             │
└─────────────────────────────────────────────┘