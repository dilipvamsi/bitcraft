# ⚖️ `bitstruct` vs. The Ecosystem

Choosing a bit manipulation library in Rust often involves balancing **Ergonomics**, **Compile-time Safety**, and **Runtime Performance**. This document compares `bitstruct` with other common solutions to help you decide which is right for your project.

## 📊 Feature Comparison Matrix

| Feature | standard Rust | `bitfield` | `modular-bitfield` | `packed_struct` | `bitstruct` (this crate) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Macro Type** | N/A | Declarative | Procedural | Procedural | **Declarative** |
| **Bit-Level Density** | ❌ (Byte-min) | ✅ | ✅ | ✅ | **✅ (Bit-min)** |
| **Memory Alignment** | Compiler-Chosen | Explicit | Explicit | Explicit | **Hardware-Aligned** |
| **Safety** | High (UB risk) | Medium | High | High | **Strict (Total Types)** |
| **`no_std` Support** | ✅ | ✅ | ✅ | ✅ | **✅ (Core-only)** |
| **Compile Speed** | Instant | Fast | Slow | Slow | **Blazing Fast** |
| **Acting Primitives** | ❌ | ❌ | ❌ | ❌ | **✅ (Direct Register Routing)** |
| **Literal Guarding** | ❌ | ❌ | ❌ | ❌ | **✅ (Branchless unrolling)** |
| **Byte-Array Support** | ❌ | ⚠️ (Manual) | ✅ (Proc-macro) | ✅ (Proc-macro) | **✅ (Instant/Declarative)** |
| **Odd-Int IDs (24-bit)** | ❌ | ❌ | ❌ | ❌ | **✅ (`byteval!` Built-in)** |
| **Signed Bitfields** | ✅ | ✅ | ✅ | ✅ | **❌ (Strictly Checked)** |
| **C FFI / ABI** | ✅ | ⚠️ | ✅ | ✅ | **✅ (Transparent)** |

> [!NOTE]
> **Roadmap**: `bitstruct` base storage will remain unsigned for maximum register efficiency and deterministic bit-packing. Support for interpreting fields as **signed integers** (two's complement) within these unsigned containers is currently on the roadmap.

---

## 🔬 In-Depth Assessment

### 1. `bitstruct` vs. `modular-bitfield`
`modular-bitfield` is the current ecosystem standard for procedural-macro-based bitfields.

*   **Philosophical Difference**: `modular-bitfield` focuses on providing a "Rust-like" struct feel with `#[bitfield]` attributes. `bitstruct` focuses on **Mechanical Sympathy**—optimizing specifically for how hardware interacts with memory registers.
*   **Compile Times**: `bitstruct` uses declarative `macro_rules!`, which compile significantly faster than procedural macros and don't require external crate dependencies like `syn` or `quote`.
*   **Registers**: While `modular-bitfield` handles bit-ranges well, `bitstruct!` specializes in **Acting Primitives**. It ensures that even if you are manipulating 5 bytes of data, the CPU uses a 64-bit register for atomic-like updates rather than byte-by-byte loads.

### 2. `bitstruct` vs. `packed_struct`
`packed_struct` is a powerful tool for complex, nested serialization.

*   **Complexity vs. Speed**: `packed_struct` is feature-rich (endianness, custom bit-numbering) but can be "particular" about type signatures and has a steeper learning curve. `bitstruct` favors a **Flat & Fast** approach.
*   **Internal Optimization**: `bitstruct` implements the **Literal Guard Pattern**. Instead of loops or fragmented memory loads, it generates flat sequences of bitwise operations that LLVM can perfectly constant-fold and vectorize.

### 3. `bitstruct` vs. Standard Rust `enum`
Standard Rust enums are **Algebraic Data Types**, which is dangerous when parsing untrusted data (like network packets).

*   **The UB Gap**: Reading an invalid bit pattern into a standard `repr(u8)` enum is **Undefined Behavior (UB)**. You must manually validate the byte before transmute.
*   **Total Types**: `bitstruct`'s `bitenum!` treats variants as "Active states" of a total underlying type. Using `try_from_bits()` ensures you never encounter UB, even if the raw input is corrupted or malicious.

### 4. `bitstruct` vs. C-Style Bitfields (FFI)
If you are interfacing with C firmware or legacy network protocols, layout predictability is mandatory.

*   **ABI Stability**: Every `bitstruct!`, `bytestruct!`, and `bitenum!` is marked `#[repr(transparent)]`. This means they have the **exact same memory representation** as their underlying Rust primitive (u8-u128) or byte-array.
*   **Natural LSB-First**: Unlike many procedural macros that can be ambiguous about bit-ordering, `bitstruct` enforces a strict LSB-first mapping. This matches the standard layout of C bitfields on Little-Endian architectures (x86_64/ARM64), allowing you to safely cast raw C-originated pointers directly into Rust types using `bytemuck`.

---

## ⚡ The `bitstruct` Edge: Unique Innovations

What makes `bitstruct` different is not just that it packs bits, but *how* it handles the instructions generated for those bits.

### 🛡️ The Literal Guard Pattern
Most libraries use runtime checks or generic loops to handle variable-width fields. `bitstruct` uses a recursive macro to generate **Literal Guards** (e.g., `if len > 4 { ... }`). Because the field widths are constants at compile-time, the optimizer deletes the branches entirely.
**Result**: Your code performs exactly like hand-optimized assembly.

### 🏎️ `bytestruct!` & `byteval!`: The Competitive Edge
Most bitfield libraries in the Rust ecosystem (`modular-bitfield`, `packed_struct`) either:
1.  **Restrict you to primitives** (`u8`–`u128`).
2.  **Rely on Procedural Macros** which drastically increase compile-time and complexity for small array-backed types.

`bitstruct` is the **only library** to offer instant, declarative bitfields for arbitrary `[u8; N]` arrays. 
- **Wait, why does this matter?** If you need a 3-byte integer (24-bit ID) that is packed 1,000,000 times in an array, standard Rust forces you to use a 4-byte `u32` (wasting 25% of your memory footprint) or write massive boilerplate. `byteval! { struct Id(3); }` solves this in one line with zero runtime cost.
- **Register Routing**: While other array-backed solutions use slow byte-by-byte loops, `bytestruct!` uses **Acting Primitives** (e.g., loading an 8-byte slice of a 13-byte array into a `u64` register). This is a level of specialized hardware optimization not found in broader general-purpose serializing libraries.

### 🧩 LSB-First Consistency
Many libraries struggle with bit-ordering consistency across different platforms. `bitstruct` enforces a strict **Little-Endian / LSB-First** convention. This ensures that the layout you see in your source code perfectly matches the physical bits on a little-endian CPU (x86_64, ARM64), eliminating cognitive load during debugging.

---

## 🏁 Conclusion: When should you use `bitstruct`?

*   **Use `bitstruct` if**: You are writing a vector engine, a high-frequency trading system, a network driver, or any code where **Cache Locality** and **Instruction Latency** are your primary bottlenecks.
*   **Use something else if**: You need complex nested structures with varying endianness within the same struct, or you prefer attribute-heavy procedural syntax over declarative macro blocks.
