```
glowworm/
│
├── Cargo.toml
├── README.md
├── LICENSE
│
├── src/
│ ├── lib.rs
│ │
│ ├── array/
│ │ ├── mod.rs
│ │ ├── ndarray.rs # Core array type
│ │ ├── shape.rs # Shape & dimension logic
│ │ ├── strides.rs # Stride calculations
│ │ ├── indexing.rs # Slice indexing, boolean masks
│ │ └── iter.rs # Element/n-dimensional iterators
│ │
│ ├── ops/
│ │ ├── mod.rs
│ │ ├── arithmetic.rs # +, -, \*, / broadcasting rules
│ │ ├── algebra.rs # dot, matmul
│ │ └── reduce.rs # sum, mean, min, max, etc.
│ │
│ ├── linalg/
│ │ ├── mod.rs
│ │ └── decompositions.rs # LU, QR, SVD (optional)
│ │
│ ├── utils/
│ │ ├── mod.rs
│ │ ├── errors.rs # Custom error types
│ │ └── macros.rs # Helpful macros (optional)
│ │
│ └── data/
│ ├── mod.rs
│ └── buffer.rs # Memory buffers, contiguous or not
│
├── benches/
│ └── array_bench.rs
│
├── examples/
│ ├── basics.rs
│ └── matmul.rs
│
├── tests/
│ ├── array_tests.rs
│ ├── ops_tests.rs
│ └── linalg_tests.rs
│
└── docs/
├── design.md # Architecture & decisions
├── roadmap.md
└── algorithms.md # Broadcast rules, stride logic, etc.
```
