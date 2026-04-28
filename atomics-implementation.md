# Atomics Implementation (`atomic_bitstruct!`)

The `atomic_bitstruct!` macro provides **zero-cost, lock-free concurrent mutation** of individual bitfields packed inside standard Rust atomic integers.

This architecture allows developers to define memory-efficient flags, counters, and states that can be safely updated concurrently by multiple threads without the need for traditional synchronization primitives like `Mutex` or `RwLock`.

---

## 1. Supported Base & Field Types

The macro natively maps to `core::sync::atomic` types, giving you full control over the underlying memory footprint:

- **Unsigned Atomics**: `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`
- **Signed Atomics**: `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`
*(Note: `AtomicU128` and `AtomicI128` are not supported by the Rust standard library at the time of writing).*

Fields defined inside an `atomic_bitstruct!` inherit all the robust capabilities of the core `bitstruct!` engine. This includes:

1. `bool`: Single-bit boolean flags.
2. **Unsigned Integers** (`u8` to `u128`): Extracted using logical bitshifts.
3. **Signed Integers** (`i8` to `i128`): Safely extracted using automatic arithmetic shift sign-extension. Negative bounds testing and 2's complement handling are built-in.
4. **Custom Enums**: Derived via the `bitenum!` macro for type-safe state representations.

---

## 2. Generated API Surface

When you use the `atomic_bitstruct!` macro, it generates an outer **Atomic Struct** and an inner **Snapshot Value Struct**. Here is a complete overview of the methods available to you.

```rust
use bitcraft::atomic_bitstruct;
use core::sync::atomic::{AtomicU32, Ordering};

atomic_bitstruct! {
    /// A lock-free task scheduler state.
    pub struct AtomicTaskState(AtomicU32) {
        pub is_running: bool = 1,
        pub retries: u8 = 7,
        pub task_id: u16 = 16,
    }
}
```

### Outer Atomic Methods (`AtomicTaskState`)

These methods interact directly with the shared memory location. Every method requires an explicit `core::sync::atomic::Ordering`.

- **Field Getters**: `state.is_running(Ordering::Acquire) -> bool`
- **Field Setters**: `state.set_retries(5, Ordering::Release)` (Executes a targeted CAS loop)
- **Snapshot `get()`**: `state.get(Ordering::Acquire) -> AtomicTaskStateValue`
- **Total `set()`**: `state.set(new_value, Ordering::Release)` (Direct `store()`, overwrites everything)
- **Batch `update()`**: `state.update(set_order, fetch_order, |v| { ... })`
- **Conditional `update_or_abort()`**: `state.update_or_abort(set_order, fetch_order, |v| { ... })`

### Inner Snapshot Methods (`AtomicTaskStateValue`)

The snapshot value (`v` in closures, or returned by `get()`) is a standard, non-atomic `bitstruct!`. It operates entirely in local CPU registers.

- **Field Getters**: `v.retries() -> u8`
- **Field Setters**: `v.set_retries(5)` (Instant bitwise mutation locally)
- **Safe Setters**: `v.try_set_retries(300)` (Returns `Err` if value overflows bit-width)
- **Builders**: `v.with_is_running(true)` (Chainable initialization)
- **Raw Interop**: `v.to_bits() -> u32` / `AtomicTaskStateValue::from_bits(raw)`

---

## 3. Architecture & Concurrency Model

### The Challenge of Atomic Bitfields

Updating an entire atomic integer is trivial. However, updating a specific 4-bit chunk inside a 32-bit integer is a fundamentally different challenge. Standard hardware atomic instructions like `fetch_add` or `fetch_or` lack the ability to clear specific bit ranges while simultaneously setting others.

### CAS Loop Synchronization (`fetch_update`)

To achieve field-level atomicity, `atomic_bitstruct!` relies entirely on **Compare-And-Swap (CAS)** loops via Rust's `fetch_update` primitive.

When you call a generated setter method (e.g., `set_status`), the macro automatically generates the optimal bitmask and executes the following underlying logic:

```rust
// Internally generated inside the macro:
self.0.fetch_update(set_order, core::sync::atomic::Ordering::Relaxed, |raw| {
    // 1. Mask out the old field value (clearing the bits)
    // 2. Shift and apply the new value
    Some((raw & !MASK) | (val_masked << OFFSET))
}).unwrap();
```

**Why this is safe:** If another thread mutates the atomic integer in the exact microsecond between the "read" and the "write" phases, the hardware CAS instruction fails. `fetch_update` will automatically spin, fetch the *newest* value, and retry the mask-and-shift operation. This ensures absolute correctness and zero data-loss under high contention.

---

## 4. Struct-Level Batch Updates (The `Value` Pattern)

Performing back-to-back single-field updates on an atomic struct is highly inefficient, as each individual setter triggers its own independent CAS loop. Furthermore, it breaks transactionality: another thread might observe the struct in a partially updated state.

To solve this cleanly, whenever you define an `atomic_bitstruct!`, the macro automatically generates a shadow, non-atomic type named `[StructName]Value`. This shadow type is a standard `bitstruct!` representing a disconnected, point-in-time snapshot of the bits.

The macro exposes three distinct methods for struct-level interactions using this `Value` type: `.get()`, `.set()`, and `.update()`.

### `get()`: Non-Atomic Snapshot

If you want to read all fields exactly as they exist at a specific point in time, you can fetch a `Value` snapshot directly.

```rust
// Fetches the entire atomic state in a single CPU instruction
let current_snapshot = f.get(Ordering::Acquire);

// You can now read multiple fields locally without further atomic overhead
if current_snapshot.is_ready() {
    println!("Limit is: {}", current_snapshot.limit());
}
```

### `set()`: Total Overwrite

If you have constructed a `Value` struct locally and wish to overwrite the *entire* atomic state at once, you can use `.set()`.

```rust
let mut new_state = ConcurrentFlagsValue::default();
new_state.set_is_ready(true);
new_state.set_limit(10);

// Instantly overwrites the atomic integer using a direct memory store.
// WARNING: Any concurrent changes made by other threads will be completely wiped out!
f.set(new_state, Ordering::Release);
```

`.set()` bypasses the CAS loop entirely, utilizing a direct atomic `store()`. It is extremely fast but offers no protection against overwriting concurrent modifications.

### `update()`: Transactional CAS Loop

If you want to modify a few fields simultaneously while **preserving** concurrent modifications made to *other* fields by different threads, you must use `.update()`.

The `.update()` method yields a mutable `Value` snapshot into a closure. Once the closure finishes, the new bits are pushed back in a *single* CAS transaction.

```rust
// We update multiple fields atomically in ONE single transaction!
let final_state = f.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
    // 'v' is a mutable ConcurrentFlagsValue snapshot.
    // These calls are normal, non-atomic operations. They are extremely fast.
    v.set_is_ready(true);
    v.set_retries(-500);

    let current_limit = v.limit();
    v.set_limit(current_limit + 1);
});

// The atomic struct is now fully updated, guaranteed.
// Any concurrent changes to fields you didn't touch (e.g., if there was a fourth field)
// are safely preserved by the CAS retry mechanism!
assert_eq!(final_state.retries(), -500);
```

This ensures absolute atomicity across multi-field changes without intermediate states leaking to other threads, while strictly preserving concurrency!

### `update_or_abort()`: Lock-Free Business Logic & Partial Updates

While `.update()` blindly forces the CAS loop to eventually commit, many concurrent systems need to **conditionally abort** a transaction based on the concurrent state.

For example, you might only want to increment a connection counter if the pool is not full. The `.update_or_abort()` method takes a closure that returns an `Option<()>`. If you return `None`, the CAS loop safely aborts, no memory is written (saving CPU cache line invalidations), and it returns an `Err(original_snapshot)`.

#### The Power of "Partial Updates"

It is crucial to understand that the `v` parameter passed into your closure is a **complete snapshot** of the atomic state at that exact microsecond. Because it holds *all* the bits, you **only need to call setters for the fields you want to change**.

Any field you *don't* modify simply retains its value from the initial load. You do not need to read a field and write it back just to preserve it. The CAS loop inherently guarantees that the unmodified fields will be perfectly preserved during the atomic swap.

```rust
let result = pool_flags.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
    // `v` contains the full state. Let's look at `active_connections`
    let prev_active = v.active_connections();

    // 1. Evaluate concurrent state against business logic
    if prev_active >= 100 {
        return None; // ABORT! The pool is full.
    }

    // 2. State is valid, mutate our local snapshot.
    // Notice we ONLY modify `active_connections`.
    // If this struct also has an `is_active` boolean or a `status` enum,
    // they are AUTOMATICALLY preserved because we didn't touch them!
    v.set_active_connections(prev_active + 1);

    // 3. Commit the transaction to the CAS loop
    Some(())
});

match result {
    Ok(final_state) => {
        println!("Success! New connection count: {}", final_state.active_connections());
    }
    Err(stalled_state) => {
        println!("Failed to connect: Pool is at max capacity ({})!", stalled_state.active_connections());
    }
}
```

#### Why this is better than Mutex/RwLock

1. **Zero Thread Blocking**: If multiple threads try to grab a connection at the exact same microsecond, they don't go to sleep waiting for a Mutex lock. The CAS loop simply spins, fetching the newly-updated value and re-running your closure instantly.
2. **Safe Back-Offs**: By returning `None`, your thread safely backs off without triggering expensive memory-bus locks or cache invalidations across the CPU cores.
3. **No Deadlocks**: Because there are no locks, there are no deadlocks.

This completely replaces the need for heavy synchronization primitives, allowing you to build highly-contended, transactionally-safe architectures where threads can cleanly back off or fail based on localized business rules.

---

## 5. Bounds Checking & Safety in Atomic Contexts

Because `atomic_bitstruct!` is built directly on top of the `bitstruct!` engine, all field-level protections are strictly enforced even within concurrent CAS loops.

### Individual `try_set()`

When setting individual fields via standard setters (e.g., `set_limit(val, order)`), if the provided `val` exceeds the defined bit limits of the field, the macro will safely drop the overflowing bits to prevent adjacent field corruption.

If you need runtime validation, every field also generates a `try_set_[field_name]` method.

```rust
// Safely attempts to execute the CAS loop.
// If the value is too large, the CAS loop is aborted BEFORE execution
// and it returns a BitstructError.
f.try_set_limit(255, Ordering::SeqCst)?;
```

### Bounds Safety Inside `.update()`

When you use the `.update()` closure, the `Value` snapshot is a standard `bitstruct!`. All bounds checking is enforced locally inside the closure *before* the CAS loop attempts to commit the final state to memory.

```rust
f.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
    // If you use try_set inside the batch updater, you can catch bounds errors
    // locally before the atomic commit happens!
    if v.try_set_limit(999).is_err() {
        // Handle error...
        return None; // Abort CAS loop!
    }
    Some(())
});
```

---

## 6. Mechanical Sympathy & Memory Orderings

To provide "Mechanical Sympathy" with the underlying hardware, `atomic_bitstruct!` never hides synchronization costs. Every getter and setter explicitly requires a `core::sync::atomic::Ordering`.

This enforces that developers actively dictate the CPU-level memory barriers emitted by the compiler:

- **Getters** (`f.is_ready(Ordering::Acquire)`): Map directly to atomic `.load(order)`.
- **Setters** (`f.set_ready(true, Ordering::Release)`): The provided `order` acts as the `set_order` (the successful write path) for the CAS loop. The underlying failure/retry load path is hardcoded to `Ordering::Relaxed` internally, ensuring optimal performance by not emitting unnecessary CPU synchronization barriers when a loop iteration is aborted.
- **Batch Updates** (`f.update(set_order, fetch_order, closure)`): Allows full explicit configuration of both the success and the retry ordering constraints.

By passing `Ordering` directly into the generated methods, `atomic_bitstruct!` adheres strictly to Rust's concurrency paradigms, giving you high-level abstractions without sacrificing low-level performance tuning.
