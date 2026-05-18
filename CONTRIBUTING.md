# Contributing to bitcraft

First off, thank you for considering contributing to `bitcraft`! It's people like you that make the open-source community such a great place to learn, inspire, and create.

## 🧠 Philosophy

`bitcraft` is designed with strict performance and minimal footprint in mind. Before contributing, please understand our core principles:

1. **Zero-Cost Abstractions**: The macros must compile down to register-optimized bitwise operations (`u32`, `u64`, `u128`).
2. **`no_std` Compatibility**: The core crate must remain `#![no_std]` compatible. Any features requiring dynamic memory allocation (like `bytevec!`) must be explicitly gated behind the `alloc` feature.
3. **Opt-in Concurrency**: Concurrency primitives (like `atomic_bitstruct!`) rely on `portable-atomic` and must be strictly gated behind the `atomics` feature.

## 🛠️ Local Development

### Prerequisites

- Install the latest stable Rust toolchain via [rustup](https://rustup.rs/).

### Running Tests

Due to the feature-gated nature of `bitcraft`, it is critical to run tests with all features enabled to ensure full coverage.

```bash
# Run all tests with atomics and alloc features enabled
cargo test --all-features

# Verify no_std compatibility (core functionality only)
cargo build --no-default-features
```

### Expanding Macros

If you are modifying the macro definitions (`src/macros/*.rs`), it is highly recommended to use `cargo-expand` to verify the generated code.

```bash
cargo install cargo-expand
cargo expand --test your_test_file
```

## 📝 Pull Request Process

1. **Create an Issue**: Before undertaking a major rewrite or adding a new macro, please open an issue to discuss the design.
2. **Fork and Branch**: Fork the repository and create a new branch from `main`.
3. **Write Tests**: Any new functionality MUST be accompanied by comprehensive tests in the `tests/` directory. If it involves atomics, ensure lock-free invariants are tested under concurrency.
4. **Documentation**: Update the relevant markdown files (`README.md`, `implementation.md`, etc.) and ensure all public traits/macros have rustdoc strings.
5. **Run Formatters and Linters**:

   ```bash
   cargo fmt --all
   cargo clippy --all-features -- -D warnings
   ```

6. **Submit**: Open a Pull Request and detail the changes.

## 🐛 Reporting Bugs

If you find a bug, please create an issue containing:

- The `bitcraft` version you are using.
- The Rust compiler version (`rustc --version`).
- A minimal reproducible example (ideally a short macro invocation that triggers the bug).

## 🛡️ Security Vulnerabilities

If you discover a security vulnerability (e.g., bounds-checking bypass allowing out-of-bounds memory access in `byteslice!`), please do NOT open a public issue. Instead, reach out to the maintainers privately.
