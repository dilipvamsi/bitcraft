# Bitcraft ⚙️

**The zero-cost, hardware-aligned bitfield and enumeration engine for Rust.**

`bitcraft` is a high-performance declarative macro library designed for systems where every bit counts. Engineered for **Mechanical Sympathy**, it allows developers to define data structures that align perfectly with CPU cache lines and memory bus widths, eliminating the silent performance tax of implicit padding.

> [!NOTE]
> **Type Safety**: `bitcraft` natively supports both **unsigned** (`u8` through `u128`) and **signed** (`i8` through `i128`) base integers for underlying storage. When using a signed base, the macro enforces a strict boundary (e.g. 15 bits for `i16`) to guarantee the sign bit is never compromised. Support for interpreting the *fields themselves* as signed integers (two's complement) is currently on the roadmap.

> [!TIP]
> **New to Bitfields?** See our [Ecosystem Comparison](comparisons.md) to understand how `bitcraft` differs from `modular-bitfield`, `packed_struct`, and standard Rust enums.

> [!TIP]
> **Technical Deep Dive**: Curious about how it works? See our [Internal Implementation Guide](implementation.md) for a breakdown of TT-munching, register specialization, and hardware alignment.

> [!IMPORTANT]
> **Register Safety**: Using signed base types naturally restricts the total sum of allocated bits to avoid overflowing into the sign bit. For example, a `bitstruct` wrapping an `i32` can allocate a maximum of 31 bits. The internal bitwise mask generation safely executes in the unsigned domain to completely prevent unintended sign-extension bugs.

---

## 🚀 The Efficiency Gap

In high-performance domains (vector engines, network stacks, or high-frequency trading), standard Rust structs introduce **implicit padding** to satisfy memory alignment. A 1-bit boolean might consume 8 bits, and a 24-bit ID might occupy 32 bits. At billion-scale, this waste trashes CPU caches and increases memory pressure.

`bitcraft` solves this by giving you:

- **Absolute Bit Control**: Define exactly which bits map to which logical fields.
- **Unique `bytestruct!` Support**: Native support for **flexible 1-16 byte spans** via any unsigned array (`[u8; N]`, `[u16; N]`, `[u32; N]`, etc.). treated as primitive-like registers.
- **Unique `byteval!` IDs**: Instant "Packed IDs" for 24-bit, 40-bit, or 56-bit values that behave like first-class integers.
- **Zero-Multiplication Engine**: High-performance bitwise operations using pre-calculated constants for all types up to 128 bits. The engine unrolls up to 16 bytes into a single contiguous register operation.
- **Dynamic Register Routing**: Automatically selects the optimal CPU register (`u32`, `u64`, `u128`) based on total bit-width to minimize register pressure and maximize instruction throughput.
- **Zero-Cost Abstractions**: Generated code compiles down to the exact bitwise shifts and masks you would write by hand—verified by LLV-MIR inspection.
- **Hardware Alignment**: LSB-first mapping ensures your software layout matches the physical little-endian storage in modern hardware.
- **Boilerplate-Free Ergonomics**: Automatic `Default` (zero-init), `Debug`, and a fluid `with_*` builder pattern come standard.

---

---

## ⚖️ The Deep Dive: `std` vs. `bitcraft`

Chosen between standard Rust types and `bitcraft` depends on whether you are optimizing for **Developer Velocity** (std) or **Mechanical Sympathy** (bitcraft).

### 1. `bitstruct!` vs. Standard Structs: Memory Density

Standard Rust structs satisfy **Alignment Padding** requirements by inserting "dead space." This ensures fields align with CPU word boundaries (8 bytes on 64-bit systems).

**Standard Rust Layout:**

```rust
struct Standard {
    is_active: bool, // 1 byte + 3 bytes padding (to align 4-byte ID)
    id: u32,         // 4 bytes
} // Total: 8 bytes
```

**`bitstruct!` Core:**

```rust
bitstruct! {
    struct Packed(u32) {
        pub is_active: bool = 1, // 1 bit
        pub id: u32 = 31,        // 31 bits
    }
} // Total: 4 bytes (Zero wasted bits)
```

> **Performance Impact**: By cutting memory usage by 50%, you effectively **double your L1/L2 cache capacity** for this data type. In high-frequency loops, this reduces cache misses and memory bus contention.

### 2. `bitenum!` vs. Standard Enums: Safety & "Total Types"

Standard Rust enums are **Algebraic Data Types**. If a memory location contains a bit pattern that doesn't match a valid variant, reading it is **Undefined Behavior (UB)**.

**Standard Rust (`enum`):**

```rust
#[repr(u8)]
enum State { A = 0, B = 1 }
// let s: State = unsafe { std::mem::transmute(3u8) }; // CRASH / UB!
```

**`bitenum!` Core:**

```rust
bitenum! { enum State(2) { A = 0, B = 1 } }
// let s = State::from_bits(3); // Panic in debug mode!
let s = State::try_from_bits(3); // Returns Err(BitstructError::InvalidVariant)
```

- **The Variant Gap**: A `repr(u8)` enum with 2 variants only recognizes values `0` and `1`. Value `2` is illegal and causes UB if transmuted.
- **Safety**: `bitenum!` is **Strictly Validated**. It wraps a primitive but ensures that only defined variants can be instantiated through `try_from_bits`. All bit patterns are contained, but only valid ones are "Active." This is critical for **defensive programming** when parsing untrusted network packets or unstable hardware registers.

### 3. `bytestruct!` vs. Manual Byte Traversal: Instruction Efficiency

Manually manipulating byte arrays usually involves individual byte-level access, which is slow and prevents CPU vectorization.

**Manual Access:**
Requires multiple load instructions and manual byte-by-byte reconstruction.

```rust
let mut arr = [0u8; 5];
let x = u16::from_le_bytes([arr[0], arr[1]]); // Load arr[0], Load arr[1], Shift, Or
let y = u16::from_le_bytes([arr[2], arr[3]]);
arr[4] = 0x01; // Manual index management
```

**`bytestruct!` Acting Primitives:**

```rust
bytestruct! { struct Loc(5) { x: u16 = 16, y: u16 = 16, f: u8 = 8 } }
let mut l = Loc::default();
l.set_x(0x1234); // High-level, zero-cost API
```

The macro chooses the widest possible register (`u32`, `u64`, or `u128`) as an **Acting Primitive**.

- Accessing a field in a 5-byte array becomes: **1 Load (u64) → 1 Shift → 1 Mask**.
- Our **Unrolling Engine** eliminates loops for all structures up to 128 bits, generating literal bitwise expressions that LLVM can perfectly fuse into atomic CPU instructions.
- This reduces the **Instruction Count** and allows the CPU's out-of-order execution engine to retire the result significantly faster.

### 4. `byteval!` vs. NewType Wrappers: Ergonomics & Bandwidth

For "Odd-sized" types like 24-bit or 40-bit IDs, Rust developers often use a `u64` (wasting 24 bits of bandwidth) or a 100-line custom wrapper.

**Standard Wrapper:**

```rust
struct Id24(u32); // Consumes 4 bytes in memory
// OR
struct Id24([u8; 3]); // Hard to use for arithmetic
```

**`byteval!` Core:**

```rust
byteval! { struct Id24(3); }
let id = Id24::from_u32(0xABCDEF); // Behaves like a 3-byte u32
```

- **`byteval!`** provides a zero-cost wrapper that implements all numeric boilerplate automatically.
- it ensures your 24-bit ID actually only consumes 3 bytes on disk/wire while behaving like a first-class `u32` in your code.
- **API Precision**: Conversion methods are Restricted to the nearest Sufficient unsigned integer type (e.g., a 24-bit ID exposes `to_u32`/`from_u32` but not `to_u8` or `to_u128`), ensuring a clean and targeted API.

### 5. Advanced Mechanism: Compile-Time & Runtime Verification

Standard Rust doesn't prevent you from defining a struct that is "too big" for a specific serialization format. `bitstruct` applies several layers of safety:

- **Compile-Time Sum Verification**: `bitstruct` ensures the sum of field bits matches the base type.
- **Debug Bounds Verification**: Builders (`with_xyz`) and setters (`set_xyz`) automatically assert bounds using `debug_assert!` to catch overflow early in development. Release builds use silent truncation masking.
- **Explicit Error Handling**: For untrusted data (like network packets), use the generated `try_set_xyz` and `try_with_xyz` methods, which perform strict bounds validation and return a `Result<_, BitstructError>`.
- **Enum Validation**: `bitenum!` provides `try_from_bits`, which returns `BitstructError::InvalidVariant` if the raw value does not match a defined enumeration variant.

### 6. Summary Feature Matrix

| Feature | Standard Rust (`struct`/`enum`) | `bitstruct` bitcraft Library |
| :--- | :--- | :--- |
| **Granularity** | Byte-level (minimum 8 bits) | **Bit-level** (minimum 1 bit) |
| **Signed Bitfields** | ✅ | **❌ (Strictly Checked)** |
| **Padding** | Implicit (inserted by rustc) | **None** (Explicit control) |
| **Instruction Count** | Multiple loads/stores | **Atomic** (Register-wide) |
| **Alignment** | Compiler-enforced | **Hardware-aligned** (LSB-First) |
| **Safety** | UB-risk on invalid patterns | **UB-Free** (Total Types) & Bounds Checked |
| **FFI / C-ABI** | Manual `#[repr(C)]` | **Transparent** (Automatic) |
| **Const Eval** | Limited in enums | **Full `const fn`** support |

---

## 🚦 When To Use Which Macro?

The `bitcraft` crate provides four specialized tools. Choosing the right one determines your memory density and instruction efficiency:

- **Use `bitstruct!`** `(Base: u8 - u128, i8 - i128)`
  - **When:** You need to pack multiple small fields (booleans, 3-bit ints, 4-bit enums) into a single, standard CPU register (up to 128 bits). For signed bases, the macro restricts usage to size - 1 bits to keep the sign bit safe.
  - **Why:** Fastest execution. The CPU loads the entire struct in a single instruction, manipulates the bits in registers, and writes them back. Perfect for protocol headers or status registers.

- **Use `bytestruct!`** `(Base: [u8/u16/u32...; N])`
  - **When:** Your data structure logically exceeds 16 bytes (128 bits) but must still remain perfectly dense without padding, or when the data is intrinsically an array (like a generic payload buffer with flags at the end).
  - **Why:** Allows dense packing up to 128 bits while still utilizing the widest available CPU registers (like `u64` or `u128`) behind the scenes to modify localized chunks of the array efficiently.

- **Use `byteval!`** `(Base: [u8; N])`
  - **When:** You need a single integer value that has an "awkward" byte width (e.g., a **24-bit** (`[u8; 3]`) audio sample, or a **40-bit** (`[u8; 5]`) network ID).
  - **Why:** It generates a zero-cost NewType wrapper around the byte array but gives you native `to_u32()`, `to_u64()`, and `from_u32()` methods so it behaves like a normal number in code, without wasting the 8 or 24 padding bits a true `u32`/`u64` would consume in an array of thousands.

- **Use `bitenum!`** `(Base: N Bits mapped to u8-u128)`
  - **When:** You need a strongly-typed, memory-safe enumeration to represent a variant parameter inside one of the above structs.
  - **Why:** Pure, safe "Total Types". Writing illegal byte values over a network packet will securely return an error dynamically generated bounds-checking, while guaranteeing 0-bit overhead inside the struct.

---

## 🧩 Showcasing Interoperability

`bitcraft` is engineered for high-performance systems where data must move seamlessly between the CPU, the network, and other languages.

### 1. Network Protocol Buffers (Zero-Copy)

Using the [**`bytemuck`**](https://docs.rs/bytemuck) crate, you can cast raw network buffers directly into typed structures with zero overhead.

```rust
bitstruct! {
    /// A mockup of a TCP-like header segment.
    pub struct TcpHeader(u32) {
        pub source_port: u16 = 16,
        pub dest_port: u16 = 16,
    }
}

fn handle_packet(buffer: &[u8]) {
    // Zero-cost overlay: No memory movement, just a typed view.
    let header: &TcpHeader = bytemuck::from_bytes(&buffer[0..4]);
    println!("Routing from {} to {}", header.source_port(), header.dest_port());
}
```

### 2. Foreign Function Interface (C-ABI)

Every `bitstruct!`, `bytestruct!`, and `bitenum!` is marked `#[repr(transparent)]`. This guarantees binary compatibility with the underlying primitive, making them safe to pass directly to C or C++ interfaces as standard integers or byte arrays.

```rust
bitstruct! {
    pub struct FfiFlags(u8) {
        pub read: bool = 1,
        pub write: bool = 1,
        pub execute: bool = 1,
    }
}

// Transparently passes as 'uint8_t' in C
unsafe extern "C" {
    fn c_process_flags(flags: FfiFlags);
}
```

### 3. Hardware Register Mapping (MMIO)

Because `bitcraft` uses **LSB-first** mapping, your logical definitions perfectly match the physical bit-offsets used in hardware datasheets for little-endian architectures (x86_64, ARM64).

```rust
bitstruct! {
    pub struct ControlRegister(u32) {
        pub enable: bool = 1,
        pub mode: u8 = 2,
        pub interrupt_mask: u8 = 8,
    }
}

// Simulating a memory-mapped register access
let mut reg = ControlRegister::from_bits(0x0000_0000);
reg.set_enable(true);
reg.set_mode(2);
// Resulting value is perfectly aligned for an MMIO write.
```

### 4. Database Storage Density

Store billion-scale metadata with the absolute minimum footprint. Packing a 24-bit ID and a 7-bit status into a single `i32` saves significant storage compared to standard Rust padding while safely ignoring the sign bit.

```rust
bitstruct! {
    pub struct RecordMetadata(i32) {
        pub user_id: u32 = 24,
        pub status_flags: u8 = 7,
    } // 24 + 7 = 31 bits total. Safe for i32.
}
// 1 Billion records = 4.0GB with bitcraft vs 8.0GB+ with standard structs.
```

---

## 🧩 The Macro Suite

`bitcraft` provides four specialized macros, each targeting a specific layer of the storage-performance spectrum.

| Macro | Storage Basis | Range | Primary Use Case |
| :--- | :--- | :--- | :--- |
| [**`bitenum!`**](#1-bitenum) | `u8` .. `u128` | 1 - 128 Bits | Type-safe variants inside packed fields |
| [**`bitstruct!`**](#2-bitstruct) | Primitives | 1 - 128 Bits | Word-aligned "Hot Path" CPU optimization |
| [**`bytestruct!`**](#3-bytestruct) | **`[u8-u128; N]`** | **2 - 16 Bytes** | **Unique**: Array-backed dense buffers with register-speed |
| [**`byteval!`**](#4-byteval) | **`[u8-u128; N]`** | **3 - 16 Bytes** | **Unique**: Packed IDs (24-bit, 40-bit) as first-class numbers |

---

## ⚡ Performance Benchmarks

`bitcraft` is engineered for **Mechanical Sympathy**. While standard Rust types are optimized for simplicity, `bitcraft` allows you to trade a negligible amount of instruction latency for massive gains in memory density (e.g., 2x - 8x).

We evaluated 1,000,000,000 (1B) iterations of complex read/write operations on an optimized release build (`cargo test --release --test performance`):

### 💻 Benchmark Environment

- **OS**: "Manjaro Linux"
- **CPU**: Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz
- **RAM**: 31Gi
- **Rust Version**: 1.93.1

| Metric | Macro Type | Base Storage | Overhead vs. `std` | Physical Density |
| :--- | :--- | :--- | :--- | :--- |
| **Execution Latency** | `bitenum!` | `u8` (3 bits) | **0.95x (Faster!)** | **1.00x** (Safe) |
| **Execution Latency** | `byteval!` | `[u8; 3]` | **0.91x (Faster!)** | **2.67x Higher** |
| **Execution Latency** | `bitstruct!` | `u16` | **0.96x (Faster!)** | **2.00x Higher** |
| **Execution Latency** | `bytestruct!` | `[u8; 2]` | **1.05x (Near Parity!)** | **3.20x Higher** |

### Zero-Copy Casting (`bytemuck`)

All structs generated by `bitstruct!`, `bytestruct!`, and `bitenum!` automatically derive `bytemuck::Pod` and `bytemuck::Zeroable`. This allows for zero-cost casting between raw byte buffers and your typed structs:

```rust
let raw_bytes = [0u8; 16];
let my_struct: MyBytestruct = bytemuck::cast(raw_bytes);
```

### 🛠️ Roadmap & Future Implementation

- [ ] **Signed Field Interpretation**: Support for `i8`, `i16`, etc., via automatic Sign Extension on the N-bit fields.
- [ ] **C-Header Generation**: Integration with `cbindgen` to automatically generate FFI-compatible C headers for C/C++ firmware.
- [ ] **`serde` Integration**: Optional feature to derive `Serialize` and `Deserialize` for all packed types.
- [x] **Property-Based Testing**: Comprehensive fuzzing of bit-packing logic via `proptest`.
- [x] **Safe Mutators**: `try_set` and `try_with` methods for guaranteed boundary safety.

## 🔬 Technical Deep Dive: The Engineering Behind the Speed

`bitstruct` isn't just a set of macros; it's a compiler-aware optimization engine. Below is an analysis of the specific patterns we use to ensure that high abstraction doesn't lead to high overhead.

### 1. The "Literal Guard" Pattern

Standard bit-manipulation libraries often use dynamic loops or `copy_nonoverlapping` to read fragmented fields. In our benchmarks, this was consistently slower than our **Unrolled Engine** approach.

Instead of a loop, `bytestruct!` generates a sequence of literal and unrolled bitwise expressions. Because the field widths and positions are known at compile-time, **LLVM perfectly constant-folds these branches**. This transforms a potential memory-loop into a flat, branchless sequence of bitwise shifts and ORs—the absolute fastest execution path possible.

### 2. Register Specialization (`u64` vs. `u128`)

While `bytestruct!` supports fields up to 16 bytes (128 bits), using `u128` registers for 2-bit flags on 64-bit hardware introduces unnecessary register pressure and instruction complexity.

The engine implements **Dynamic Register Routing**:

- **Fields spanning ≤ 8 bytes**: Operations are performed using native `u64` registers. This allows the CPU to retire instructions immediately without the "software-emulated" overhead often associated with `u128` on modern 64-bit architectures.
- **Fields spanning > 8 bytes**: The macro gracefully promotes the operation to `u128`, ensuring correctness for massive fields while preserving specialized speed for hot-path metadata.

### 3. Instruction Fusion & Stack Traffic

When you manually manipulate byte arrays (e.g., `[u8; 3]`), you often introduce "Stack Traffic." Creating temporary fixed-size arrays to satisfy library signatures (like `u32::from_le_bytes([b0, b1, b2, 0])`) forces the compiler to move data from registers to the stack and back.

`bitstruct` avoids this by generating a single unrolled "Shift-and-OR" expression (e.g., `(b0 as u32) | ((b1 as u32) << 8) | ...`). Modern compilers recognize this pattern and perform **Instruction Fusion**. Instead of multiple individual shifts, the backend generates a single **Unaligned Load** instruction (like `MOV` or `LDR`), effectively loading your "packed" data directly into a high-speed CPU register in one cycle.

### 4. The Latency-Density Paradox

In micro-benchmarks, `bitstruct!` and `bytestruct!` may show a negligible ~1.2x overhead compared to standard structs. This is expected: standard structs use memory offsets, while bitfields use **Shifts, Masks, and Read-Modify-Write cycles**.

However, in real-world applications, this latency is a illusion. The **10x - 100x performance penalty** of a CPU Cache Miss dominates all other metrics. By doubling or quadrupling your physical data density, `bitcraft` ensures your data stays in the **L1/L2 Cache**, providing a massive net gain in system throughput that standard "offset-based" types cannot match.

---

## 📖 Engineering Guide

### 1. `bitenum!`

Define variants that map directly to raw bits. Unlike standard enums, `bitenum` provides a `#[repr(transparent)]` wrapper that consumes zero extra bits and automatically maps to the most efficient CPU primitive (`u8`-`u128`) based exclusively on the number of required bits.

```rust
use bitcraft::bitenum;

bitenum! {
    /// A 2-bit execution state.
    pub enum ExecutionState(2) {
        PENDING = 0,
        RUNNING = 1,
        COMPLETE = 2,
        FAILED = 3,
    }
}
```

**Memory Layout:**

- Automatically maps to the narrowest CPU primitive (`u8`-`u128`) capable of holding the specified bits.
- Example `(2)` bits: Consumes `u8` in physical memory. Natively utilizes bits `[ 1 0 ]`, while bits `[ 7 .. 2 ]` remain zeroed.
- **Strict Validation**: Providing raw values like `4` or `5` to `try_from_bits` will return an error, preventing invalid state propagation.

### 2. `bitstruct!`

The primary tool for packing data into standard CPU words. All getters and builders are `const fn`.

```rust
use bitcraft::{bitstruct, bitenum};

bitstruct! {
    /// A 16-bit packed execution descriptor.
    pub struct Descriptor(u16) {
        pub is_active: bool = 1,      // Bit 0
        pub priority: u8 = 3,         // Bits 1-3
        pub payload_size: u8 = 8,     // Bits 4-11
        pub state: ExecutionState = 2, // Bits 12-13
        // Bits 14-15 are implicit padding
    }
}

// Fluent builder pattern
let desc = Descriptor::from_bits(0)
    .with_is_active(true)
    .with_state(ExecutionState::RUNNING);

assert_eq!(desc.state(), ExecutionState::RUNNING);
```

**Memory Layout (LSB-First):**

```text
MSB                                          LSB
 [ Unused ] [ State ] [ Payload ] [ Prio ] [ Act ]
    2 bits    2 bits    8 bits     3 bits   1 bit
```

- **Field 0 (Act)**: Occupies the lowest possible bit (Index 0).
- **Field N**: Continues immediately after Field N-1.

### 3. `bytestruct!`

For when you need non-standard sizes (e.g., 5, 7, or 13 bytes) that must be array-backed but treated as a single unit.

```rust
use bitcraft::bytestruct;

bytestruct! {
    /// A 5-byte (40-bit) packed coordinate for spatial indexing.
    pub struct NodeLocation(5) {
        pub x: u16 = 16,     // 16 Bits
        pub y: u16 = 16,     // 16 Bits
        pub flags: u8 = 8,   // 8 Bits
    }
}

let loc = NodeLocation::from_u64(0x01_AABB_CCDD);
assert_eq!(loc.x(), 0xCCDD);
```

**Memory Layout (Little-Endian Array):**

```text
Byte:    [  0  ]    [  1  ]    [  2  ]    [  3  ]    [  4  ]
Field:   [  x  ]    [  x  ]    [  y  ]    [  y  ]    [flags]
Content: [Low 8]    [High 8]   [Low 8]    [High 8]   [ u8  ]
```

- **Mapping**: Directly maps bits to a `[u8; 5]` buffer.
- **Acting Primitive**: Operations use a `u64` register for 1-cycle execution.

### 4. `byteval!`

A specialization for "NewType" byte-array wrappers (e.g., 24-bit IDs).

```rust
use bitcraft::byteval;

byteval! {
    /// A 3-byte packed ID.
    pub struct PackedID(3);
}
```

**Memory Layout:**

```text
Byte:    [  0  ]    [  1  ]    [  2  ]
Content: [Low 8]    [Mid 8]    [High 8]
```

- Total Size: 3 bytes (LSB-First value).

---

## 🛡️ Technical Safety & Internals

### Acting Primitive Selection

`bytestruct!` doesn't just manipulate byte arrays. It uses an internal **"Acting Primitive"** routing system. Based on the array size, the macro selects the widest possible CPU register (`u32`, `u64`, or `u128`) to perform operations. This means reading a field from a 13-byte array compiled to a single 128-bit register load and shift—the fastest possible execution path.

### LSB-First Ordering

All macros follow a **Least Significant Bit (LSB) first** convention. The first field defined occupies the lowest bits (starting from bit 0).

> [!CAUTION]
> **Persistence Warning**: Reordering fields in a macro definition **will change the binary layout**. If your data is saved to disk or sent over a network, changing the order breaks backward compatibility.

### Strict Bound Verification

Macros execute compile-time assertions to ensure the sum of all field widths exactly matches or is less than the total storage capacity. This prevents "silent overlaps" and masking bugs.

---

## ⚖️ License

Licensed under the **MIT License**.
