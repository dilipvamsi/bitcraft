# Changelog

All notable changes to the `bitcraft` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-05-15

### Added

- **Atomic Operations:** Introduced lock-free, concurrent data structures (`atomic_bitstruct!`, `atomic_bitarray!`, `atomic_bitenum!`). These use `portable-atomic` for CAS operations and guarantee atomic updates.
- **Dynamic Heap Types:** Added heap-allocated dynamic types `bytevec!` (growable bit array) and `bytebox!` (fixed-size heap-allocated bits).
- **Zero-Copy Slicing:** Introduced `byteslice!` for zero-copy slicing over existing byte arrays, perfect for FFI and performance-critical code paths.
- **Feature Gating:**
  - `atomics`: Enables concurrent structures. Pulls in `portable-atomic`. (Disabled by default).
  - `alloc`: Enables dynamic types requiring heap allocation. (Disabled by default).
- **Extensive Documentation:**
  - Comprehensive `README.md` with usage examples.
  - Added `usecases.md` mapping internals to real-world software applications.
  - Added `comparisons.md` explicitly benchmarking against `bitvec` and `modular-bitfield`.
  - Added `implementation.md` detailing architectural design and "Acting Primitives".
- **Standardized Iterators:** Introduced unified `Iter<'a>` traits to traverse both static and dynamic bit structures with zero overhead.

### Changed

- Refactored core macro engine to strictly map to register-optimized primitives (`u32` to `u128`).
- Required integration tests to be run with `--all-features` to ensure `atomics` dependencies compile correctly.

### Fixed

- Ensured strict `#![no_std]` compatibility across the core library.

## [0.9.x] and earlier

### Added

- Initial core `bitcraft` macros (`bitstruct!`, `bitarray!`, `bitenum!`).
- Fundamental bit-shifting and masking logic.
- Type-safe bounds checking via standard traits.
