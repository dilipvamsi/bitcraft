# 🗺️ `bitcraft` Use Cases Guide

With so many specialized tools in the `bitcraft` suite, it can be tricky to know exactly which macro to reach for. This guide maps each macro to its ideal real-world scenario.

---

## 1. The Structure Macros

### `bitstruct!`

**Best Used For**: "Hot path" data that fits within standard CPU registers (1-128 bits).
**Real-World Example**: Parsing IPv4 / TCP packet headers, interacting with Memory-Mapped I/O (MMIO) hardware registers in an embedded OS, or packing a player's exact X/Y tile coordinate and health into a single 64-bit word for a multiplayer game server.
**When NOT to use**: If the total size of your fields exceeds 128 bits (16 bytes).

### `bytestruct!`

**Best Used For**: Dense structures that exceed standard CPU registers but still require predictable layout, or when the data logically feels like a contiguous buffer.
**Real-World Example**: A 25-byte custom Bluetooth Low Energy (BLE) advertisement packet where bits 0-4 are flags, bits 5-102 are payload, and the rest is a CRC checksum.
**When NOT to use**: If your data fits perfectly into an 8-byte `u64`, use `bitstruct!` instead for slightly less overhead.

### `atomic_bitstruct!`

**Best Used For**: High-throughput concurrent environments where multiple threads mutate disjoint fields in a shared struct.
**Real-World Example**: A web server's connection pool manager. Thread A updates the `active_connections` count, while Thread B flips the `is_draining` boolean flag on the exact same 32-bit integer without using a `Mutex`.
**When NOT to use**: When data is only mutated by a single thread (unnecessary atomic hardware overhead).

---

## 2. The Enumeration Macros

### `bitenum!`

**Best Used For**: Type-safe state validation over the wire or disk.
**Real-World Example**: Decoding a 3-bit `OpCode` from a malicious network packet. If the packet sends an invalid opcode, `try_from_bits` safely catches it without causing Undefined Behavior (UB).
**When NOT to use**: When you don't care about memory size and a standard Rust `enum` will do just fine.

### `atomic_bitenum!`

**Best Used For**: Lock-free State Machines.
**Real-World Example**: A thread pool worker transitioning from `IDLE` -> `RUNNING` -> `FAILED`. The atomic CAS loop (`update_or_abort`) ensures that if two threads try to claim the worker simultaneously, only one succeeds without taking a lock.

---

## 3. The Custom Integer Macro

### `byteval!`

**Best Used For**: Storing billions of "Odd-width" numbers without wasting padding space.
**Real-World Example**: You have a database of 500 million RGB colors (24-bit). Using `byteval!(3)` creates a native 3-byte struct that saves ~500MB of RAM compared to storing them as 4-byte `u32` integers. Alternatively, `byteval!(i 3)` perfectly handles 24-bit signed audio samples natively from an ADC.
**When NOT to use**: If the value is a standard power-of-two (just use `u16`, `u32`, etc.).

---

## 4. The Static Array Macros

### `bitarray!`

**Best Used For**: Small, dense arrays of sub-byte elements that fit into a single CPU register (up to 128 elements of 1 bit, or 32 elements of 4 bits).
**Real-World Example**: A fast path Bloom Filter or a bitset for a 64-player lobby (a single `u64` representing 64 players' "ready" states).
**When NOT to use**: If the total bits exceed 128.

### `bytearray!`

**Best Used For**: Massive, fixed-size bit arrays that live on the stack or inline within another struct.
**Real-World Example**: An inline 4096-bit (512-byte) bitset tracking the allocated memory pages in a custom memory allocator, or a fixed-size voxel chunk mask in a Minecraft clone.
**When NOT to use**: When the size needs to grow at runtime.

### `atomic_bitarray!`

**Best Used For**: Shared, concurrent collections of flags.
**Real-World Example**: A lock-free task scheduler where 64 worker threads flip flags in a shared 256-bit bitset to claim jobs.

---

## 5. The Dynamic Array Macros (Requires `alloc`)

### `bytevec!`

**Best Used For**: Growable collections of packed sub-byte data where memory efficiency is paramount.
**Real-World Example**: Reading an infinite stream of 3-bit DNA sequencing pairs (`A`, `C`, `G`, `T`) from a file into memory. A standard `Vec<u8>` would waste 5 bits per element; `bytevec!(u 3)` packs them seamlessly as the stream grows.
**When NOT to use**: When you know the exact size of your collection at compile time (use `bytearray!` instead to avoid heap allocation overhead).

### `bytebox!`

**Best Used For**: Heap-allocated bit arrays whose size is strictly determined at runtime, but will not change once allocated.
**Real-World Example**: Allocating a bit-mask exactly matched to the dynamic screen resolution of the user's monitor for a software rasterizer.
**When NOT to use**: If the collection needs to frequently `push` or `pop` elements (use `bytevec!`).

### `byteslice!`

**Best Used For**: Zero-copy views into subsets of `bytevec!`, `bytebox!`, or `bytearray!`.
**Real-World Example**: Passing the second half of your massive DNA sequence (`bytevec!`) to a separate thread or validation function without cloning the underlying memory.
