# 🛠️ `bitcraft` Implementation Deep Dive

This document details the engineering principles, memory layouts, and internal mechanisms that allow `bitcraft` to achieve zero-cost bitfield manipulation with strict type safety.

---

## 🚀 1. The Efficiency Gap

In high-performance domains (vector engines, network stacks, or high-frequency trading), standard Rust structs introduce **implicit padding** to satisfy memory alignment. A 1-bit boolean might consume 8 bits, and a 24-bit ID might occupy 32 bits. At billion-scale, this waste trashes CPU caches and increases memory pressure.

`bitcraft` solves this by giving you:

- **Absolute Bit Control**: Define exactly which bits map to which logical fields.
- **Unique `bytestruct!` Support**: Native support for **flexible 1-16 byte spans** via any unsigned array (`[u8; N]`, `[u16; N]`, `[u32; N]`, `[u64; N]`, `[u128; N]`).
- **Unique `byteval!` IDs**: Instant "Packed IDs" for 24-bit, 40-bit, or 56-bit values that behave like first-class integers—solving the "Odd-Width Integer" problem in one line.
- **Zero-Multiplication Engine**: High-performance bitwise operations using pre-calculated constants (`BitLength::BITS_N`) to eliminate manual multiplications in macro expansion.
- **Hardware Alignment**: LSB-first mapping ensures your software layout matches the physical little-endian storage in modern hardware.
- **Boilerplate-Free Ergonomics**: Automatic `Default` (zero-init) and a fluid `with_*` builder pattern come standard.

---

## 📐 2. Memory Layout: `std` vs. `bitcraft`

Standard Rust structs are optimized for **Alignment Padding** to satisfy CPU word boundaries. `bitcraft` is optimized for **Mechanical Sympathy**—aligning data to the exact bit.

### Standard Rust (`struct`)

Rust inserts "dead space" (padding) to ensure fields align with power-of-two boundaries.

```rust
struct Standard {
    active: bool, // 1 byte + 3 bytes padding
    id: u32,      // 4 bytes
} // Total: 8 bytes
```

### `bitstruct!` Layout (LSB-First)

`bitcraft` uses a **Least Significant Bit (LSB) first** mapping. The first field occupies the lowest bits of the underlying primitive.

```rust
bitstruct! {
    struct Packed(u32) {
        active: bool = 1, // Bit 0
        id: u32 = 31,     // Bits 1-31
    }
} // Total: 4 bytes (100% density)
```

**Binary Representation (u32):**

```text
MSB                                          LSB
 [ ID (31 bits)                           ] [A]
 Bit 31 ............................ Bit 1   Bit 0
```

### Big-Endian vs. Little-Endian Naturalism

While generic bitfield libraries often struggle with endianness, `bitcraft` embraces the **Natural Hardware Layout** of modern CPUs (x86_64, ARM64). By using **LSB-first** (Least Significant Bit) mapping, the logical "Bit 0" in your code matches the physical "Bit 0" in a little-endian memory dump. This makes side-by-side debugging with a hex editor intuitive and predictable.

---

## 🏎️ 3. Zero-Copy FFI & `bytemuck` Integration

One of the most powerful features of `bitcraft` is its ability to interpret raw memory buffers without copying. Every generated struct is decorated with `#[repr(transparent)]` and implements the `bytemuck::Pod` and `Zeroable` traits.

### The "No-Alloc" Pattern

In network programming or high-speed data processing, you can "overlay" a bitcraft onto a raw byte slice from a socket or disk:

```rust
// Pseudo-code
let buffer: &[u8] = socket.recv();
// Zero-cost cast: No memory is moved, no allocation occurs.
let header: &PacketHeader = bytemuck::cast_ref(&buffer[0..4]);

if header.is_valid() {
    process(header.payload_id());
}
```

This effectively turns your structs into **Schema-on-Read** overlays, making them ideal for high-performance firmware and protocol parsers.

---

## ⚙️ 4. The "Literal Guard" Pattern

A major performance bottleneck in bitfield libraries is using dynamic loops or `memcpy` to handle fields that span multiple bytes. `bitcraft` avoids this using a **Recursive Unrolling Engine**.

### The "Unrolled Expression" Pattern

When you define a field in `bytestruct!`, the macro doesn't generate a raw loop. Instead, it generates a literal bitwise expression (a "Shift-and-OR" chain) matched to the exact number of array elements spanned by the field.

**1. Literal Unrolling**:
If a field spans 3 bytes, the macro generates:
`arr[idx] | (arr[idx+1] << 8) | (arr[idx+2] << 16)`
Because the element count is known at compile-time, LLVM perfectly optimizes this into native load instructions. Standard loops cannot achieve this level of fusion consistently.

**2. Dynamic Register Routing (Acting Primitives)**:
The engine selects the narrowest possible primitive (`u32`, `u64`, or `u128`) capable of holding the field's total bit span. This ensures that a 2-byte field in a 13-byte array uses efficient 32-bit register operations instead of being forced into slower 128-bit logic.

---

## 🛡️ 5. Checked vs. Unchecked Mutators

`bitcraft` provides two distinct safety layers for modifying data, allowing developers to balance performance and validation.

### `set_*` (Optimistic/High-Performance)

Standard setters are designed for "Hot Paths" where you trust the data source (or have already validated it).

- **Behavior**: Uses `debug_assert!` to check for bit overflows.
- **Production**: In `--release` mode, the check is removed, and values are simply masked. This ensures **zero branch overhead** in tight loops.

### `try_set_*` & `try_with_*` (Validated/Schema-Entry)

Try-setters and their corresponding builder methods are designed for boundaries where data is untrusted (e.g., input from a REST API or a file).

- **Behavior**: Always returns a `Result<(), BitstructError>` (for setters) or `Result<Self, BitstructError>` (for builders).
- **Production**: Performs a runtime bounds check and returns an explicit `BitstructError::Overflow` if the value exceeds the allocated bit-width, preventing data corruption.
- **`byteval!` Specifics**: For "Packed ID" types, these methods are named `try_set_value` and `try_with_value`, providing a safe boundary for odd-width integers.

---

## ⛓️ 6. The "Fluid Builder" Pattern

Generated structs include `with_*` and `try_with_*` methods that enable a clear, functional-style API while preserving register-level efficiency.

### Register Renaming Optimization

Because these methods operate on `self` by value on the stack, modern CPU **Register Renaming** units can often execute multiple "updates" in parallel within the same register pipeline.

```rust
// Functional style is as fast as manual mutation
let config = Config::default()
    .with_enabled(true)
    .with_mode(7);
```

---

## 🛡️ 7. Strict Type Safety Enforcement

The library enforces careful bounds checking and primitive routing to avoid the "Two's Complement Ambiguity" during bit-packing when dealing with signed base storage types.

### Internal Traits

We use **Marker Traits with Associated Constants** to trigger compile-time errors and route primitive behaviors.

#### `IsValidBaseInt`: Restricting and Bounding Base Storage

```rust
pub trait IsValidBaseInt {
    type Unsigned: ValidField + BitLength;
    const MAX_BITS: usize;
}
impl IsValidBaseInt for u32 {
    type Unsigned = u32;
    const MAX_BITS: usize = 32;
}
impl IsValidBaseInt for i32 {
    type Unsigned = u32;
    const MAX_BITS: usize = 31; // Reserves the sign bit!
}
// ... implemented for u8-u128 and i8-i128
```

#### `ValidField`: Restricting Field Types

```rust
pub trait ValidField { const ASSERT_VALID: (); }
impl ValidField for bool { const ASSERT_VALID: () = (); }
impl ValidField for u8 { const ASSERT_VALID: () = (); }
// ... implemented for u8-u128 and bitenum! generated types
```

### Enforcement Mechanism

The macro inserts a check into a `const _: () = { ... };` block:

```rust
const _: () = {
    // This will FAIL TO COMPILE if the sum of fields exceeds the MAX_BITS of the base type.
    // e.g. for an i32 base, if the sum > 31 bits, this triggers an error.
    let _ = [(); 0 - (!(TOTAL_SUM <= <$base_type as $crate::IsValidBaseInt>::MAX_BITS) as usize)];

    // This will FAIL TO COMPILE if $field_type is not a supported type.
    let _ = <$field_type as $crate::ValidField>::ASSERT_VALID;
};
```

---

## 🔌 8. Acting Primitives (Register Routing)

`bytestruct!` supports arrays up to 16 bytes, but it doesn't always use 128-bit operations. It uses **Dynamic Register Routing** to choose the smallest register that can safely hold the field.

| 1-4 Elements (`u8`) | `u32` | 32-bit registers |
| 5-8 Elements (`u8`) | `u64` | 64-bit registers |
| 9-16 Elements (`u8`) | `u128` | 128-bit register |
| Arrays of `u16/u32/u64` | Specialized | Hardware-native |

### Implementation Logic (Pseudo-code)

```rust
macro_rules! route_unroll {
    (span: 1-4) => { unroll_as_u32; }
    (span: 5-8) => { unroll_as_u64; }
    (any) => { loop_fallback; }
}
```

This ensures that "Hot Path" metadata within larger arrays never suffers the overhead of 128-bit register emulation on older hardware.

---

## 💎 9. `bitenum!` and "Total Types"

Standard Rust enums are **Algebraic Data Types**. Reading an invalid bit pattern into a standard `enum` is **Undefined Behavior (UB)**.

`bitenum!` solves this by creating a **Total Type wrapper** around a primitive.

### Standard Enum Danger

```rust
#[repr(u8)]
enum Std { A = 0, B = 1 }
// let x: Std = unsafe { transmute(3u8) }; // UB / CRASH
```

### `bitenum!` Safety

```rust
bitenum! { enum Safe(2) { A = 0, B = 1 } }
// Implementation:
struct Safe(u8);
impl Safe {
    fn is_defined(&self) -> bool {
        match self.0 { 0 | 1 => true, _ => false }
    }
}
```

- **From Bits**: In debug mode, `from_bits` panics if the input is > mask. In release, it truncates.
- **Try From Bits**: Returns `Result<Self, BitstructError::InvalidVariant>`, ensuring your system **never encounters UB** even when parsing malicious network packets.

---

## ❄️ 10. `const` Initialization & Zero-Init Guarantee

`bitcraft` is designed for static and embedded environments where boot-time initialization must be zero-cost.

### Zero-Cost Statics

Every generated accessor and constructor is a `const fn`. This allows you to define global configurations that are baked into the binary's `.data` segment.

```rust
// Baked into the binary at compile-time
static DEFAULT_CFG: Config = Config::from_bits(0x01);
const TEMPLATE: Config = Config::default().with_enabled(true);
```

Because the base storage is always an unsigned integer or primitive array, `bitcraft` types are inherently **Zero-Init compatible**. Running `Default::default()` on a 16-byte bitcraft is equivalent to a single 128-bit clear instruction.

---

## ⚡ 11. Instruction Efficiency: Atomic Register Manipulation

While standard Rust types are optimized for simplicity, `bitcraft` allows you to trade a negligible amount of instruction latency for massive gains in memory density.

### Register Specialization (`u64` vs. `u128`)

While `bytestruct!` supports fields up to 16 bytes (128 bits), using `u128` registers for 2-bit flags on 64-bit hardware introduces unnecessary register pressure and instruction complexity.

The engine implements **Dynamic Register Routing**:

- **Fields spanning ≤ 8 bytes**: Operations are performed using native `u64` registers. This allows the CPU to retire instructions immediately without the "software-emulated" overhead often associated with `u128` on modern 64-bit architectures.
- **Fields spanning > 8 bytes**: The macro gracefully promotes the operation to `u128`, ensuring correctness for massive fields while preserving specialized speed for hot-path metadata.

### Instruction Fusion & Stack Traffic

When you manually manipulate byte arrays (e.g., `[u8; 3]`), you often introduce "Stack Traffic." Creating temporary fixed-size arrays to satisfy library signatures (like `u32::from_le_bytes([b0, b1, b2, 0])`) forces the compiler to move data from registers to the stack and back.

`bitcraft` avoids this by generating a single unrolled "Shift-and-OR" expression (e.g., `(b0 as u32) | ((b1 as u32) << 8) | ...`). Modern compilers recognize this pattern and perform **Instruction Fusion**. Instead of multiple individual shifts, the backend generates a single **Unaligned Load** instruction (like `MOV` or `LDR`), effectively loading your "packed" data directly into a high-speed CPU register in one cycle.

---

## ⚖️ 12. Summary Feature Matrix

| Feature | Standard Rust (`struct`/`enum`) | `bitcraft` Library |
| :--- | :--- | :--- |
| **Granularity** | Byte-level (minimum 8 bits) | **Bit-level** (minimum 1 bit) |
| **Byte-Array Basis** | ❌ | **Unique `bytestruct!` Support** |
| **Odd-Width Ints** | ❌ | **Unique `byteval!` Support** |
| **Padding** | Implicit (inserted by rustc) | **None** (Explicit control) |
| **Instruction Count** | Multiple loads/stores | **Atomic** (Register-wide) |
| **Alignment** | Compiler-enforced | **Hardware-aligned** (LSB-First) |
| **Safety** | UB-risk on invalid patterns | **UB-Free** (Total Types) & Bounds Checked |
| **FFI / C-ABI** | Manual `#[repr(C)]` | **Transparent** (Automatic) |
| **Const Eval** | Limited in enums | **Full `const fn`** support |

---

## ⚖️ 13. Instruction Count Comparison

| Operation | `std` Struct | `bitcraft` (Optimal) |
| :--- | :--- | :--- |
| **Read Field** | `MOV (offset)` | `MOV + SHR + AND` |
| **Write Field** | `MOV (offset)` | `MOV + AND (mask) + OR + MOV` |
| **Bulk Load** | Multiple instructions | **Single 64/128-bit Load** |

While bitfields require slightly more instructions per individual access, they drastically reduce **Stack Traffic** and **Cache Misses**. By doubling your memory density, you effectively double the speed of your CPU cache for that data structure.

---

## 🧩 14. Recursive Macro Architecture (TT Munching)

`bitcraft` uses the **Token-Tree (TT) Muncher** pattern to process field definitions. This is a recursive macro technique that allows for sequential processing of an arbitrary list of inputs.

### 1. Sequential Traversal

Because Rust macros cannot use standard "loops" for code generation, we use recursion. Each macro call processes the *first* field in the list and then calls itself with the *remaining* fields.

### 2. State Accumulation ($shift)

With each recursive call, the macro passes along an accumulated `$shift` value. This ensures that the second field starts exactly where the first one ended, maintaining LSB-first packing without the user having to calculate raw bit offsets manually.

### Recursive Logic (Pseudo-code)

```rust
macro_rules! bitcraft {
    // 1. Entrance Point: Start the recursion at bit offset 0.
    (struct $name:ident ($base:ty) { $($fields)* }) => {
        bitstruct!(@impl_getters_setters $base, 0, $($fields)*);
    };

    // 2. Termination Case: No more fields left to process.
    (@impl_getters_setters $base:ty, $shift:expr, ) => {};

    // 3. Recursive Step: Process the HEAD field, then recurse with the TAIL.
    (@impl_getters_setters $base:ty, $shift:expr, $vis:vis $name:ident $type:tt $bits:tt $($rest:tt)*) => {
        // Generate Getter/Setter for this field using the current $shift.
        impl_accessor!($name, $type, $bits, $shift);

        // Call self again with the NEW shift: (current_shift + current_bits)
        bitstruct!(@impl_getters_setters $base, $shift + $bits, $($rest)*);
    };
}
```

### 3. Type-Based Specialization (The Router)

The recursion also acts as a "Router". By pattern-matching on the field type during the recursive step, `bitcraft` can generate specialized code:

- **`bool`**: Generated getters return native `true`/`false` by checking if the bit is non-zero.
- **`bitenum!`**: Generated getters wrap the raw bits in the custom enum type using its `from_bits` method.
- **`u8 - u128`**: Generic numeric getters perform a standard shift-and-mask.

---

## 🏗️ 15. The "Parsing" Pipeline: TT Munching in Detail

Since `bitcraft` uses declarative `macro_rules!`, it does not have access to a formal AST (Abstract Syntax Tree) in the compiler's sense. Instead, it operates on a stream of **Token Trees**. The "parsing" logic is distributed across three distinct phases.

### Phase 1: The Collective Entry Point

When the macro is invoked, it first performs **batch operations** on the entire list of fields using standard macro repetitions (`$()*`).

- **Memory Density Validation**: It sums the `$bits` of all fields (`0 $(+ $bits)*`) and compares it against the `size_of::<BaseType>()`.
- **Debug Implementation**: It iterates through every `$field_name` to build a clean `fmt::Debug` output.
- **Verification Trigger**: It passes all field types to `@check_fields` to trigger trait-based validation.

### Phase 2: The Recursive "Muncher"

After the initial setup, the macro hands off the stream of tokens to the **TT Muncher** (`@impl_getters_setters`).
A typical muncher arm captures the "Head" (current field) and "Tail" (remaining fields). It calculates the next offset as `$shift + $bits` and passes it to the next recursive call.

### Phase 3: Type Specialization (Pattern Matching)

The muncher doesn't just treat everything as text. It uses **Literal Token Matching** to decide which code to generate.

- **If the type is exactly `bool`**: Specialized logic for boolean conversion.
- **If the type is a primitive (e.g., `u16`)**: Routes to generic integer getters.
- **Fallback (`$type:tt`)**: Assumes a custom `bitenum!` and generates glue code for variant conversion.

This "Pipeline" ensures that even though the logic is defined within a macro, the resulting code is as specialized and typed as if you had written individual implementations for every field by hand.

---

## 🏛️ 16. Architectural Pillars: The Design Philosophy

The `bitcraft` architecture is built on four core pillars that distinguish it from general-purpose serialization libraries.

### 1. Hardware Naturalism (LSB-First)

Unlike libraries that offer "adjustable" bit-ordering, `bitcraft` enforces a strict **Least Significant Bit (LSB) first** convention. This decision is architectural: by matching the physical storage of little-endian CPUs (x86_64, ARM64), we ensure that a bitcraft in memory looks exactly like it does in the source code. This eliminates the "Endianness Tax" and allows for direct MMIO (Memory Mapped I/O) compatibility.

### 2. Register Specialization (Acting Primitives)

The "Acting Primitive" pattern is central to our speed. We don't treat byte-arrays as arrays; we treat them as **Dynamic Registers**. By promoting an 11-byte array to a `u128` register for bit manipulation, we move data from the slow RAM stack into high-speed CPU registers as early as possible. This minimizes "Stack Traffic" and maximizes the instruction retirement rate.

### 3. Declarative-Only Integrity

By intentionally avoiding procedural macros (`syn`/`quote`), `bitcraft` remains **highly portable** and **compiles in milliseconds**. This architecture ensures that the crate has zero heavy dependencies, making it suitable for even the most minimal `no_std` embedded environments where compile-time resources are constrained.

### 4. Zero-Cost Verification (Trait-Based Safety)

We use a **Two-Tier Verification** strategy:

- **Macro-Level**: Sum checks and bit-width assertions happen during expansion.
- **Trait-Level**: We use marker traits (`ValidField`, `IsUnsignedInt`) to inject compile-time errors if a user attempts to use a signed integer or an invalid storage type. This architecture shifts the burden of safety from the **runtime** to the **compiler**, ensuring that a compiled `bitcraft` binary is as lean as manual C code.

---

### 5. Zero-Multiplication Expansion (`BitLength`)

To ensure maximum compile-time efficiency and clean expansion, `bitcraft` avoids manual multiplications like `count * BITS` in macros. Instead, the `BitLength` trait provides pre-calculated constants:

```rust
pub trait BitLength {
    const BITS: usize;
    const BITS_2: usize; // BITS * 2
    // ... up to BITS_16
}
```

Macros now expand to `<$unit as BitLength>::BITS_N`, which the compiler resolves to a literal constant instantly. This ensures that the generated bitwise expressions are as lean as possible.

---

## 🛠️ Roadmap & Future Implementation

- [ ] **Signed Field Interpretation**: Support for `i8`, `i16`, etc., via automatic Sign Extension on the N-bit fields.
- [ ] **C-Header Generation**: Integration with `cbindgen` to automatically generate FFI-compatible C headers for C/C++ firmware.
- [ ] **`serde` Integration**: Optional feature to derive `Serialize` and `Deserialize` for all packed types.
- [x] **Property-Based Testing**: Comprehensive fuzzing of bit-packing logic via `proptest`.
- [x] **Safe Mutators**: `try_set` and `try_with` methods for guaranteed boundary safety.
