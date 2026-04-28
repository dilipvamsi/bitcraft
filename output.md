# Bitcraft Macro Showcase & Expansion ⚙️

This document explains the functionality of the `bitcraft` engine and provides the **struct equivalent** code that the Rust compiler actually expands during compilation, reflecting the latest performance optimizations.

---

## 1. `bitenum!` (Type-Safe Enumerations)

Generates a zero-cost, memory-safe enumeration that resolves to the narrowest possible CPU primitive. It now includes pre-computed range constants.

### **Usage**

```rust
bitenum! {
    pub enum ConnectionState(2) {
        DISCONNECTED = 0,
        CONNECTING = 1,
        CONNECTED = 2,
    }
}

bitenum! {
    pub enum SignedState(i 3) {
        MIN = -4,
        ERROR = -1,
        OK = 0,
        MAX = 3,
    }
}
```

### **Generated "Struct Equivalent"**

```rust
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ConnectionState(pub u8);

impl ConnectionState {
    pub const DISCONNECTED: Self = Self(0);
    pub const CONNECTING: Self = Self(1);
    pub const CONNECTED: Self = Self(2);

    #[allow(dead_code)]
    /// The number of bits allocated for this enumeration in memory.
    pub const BITS: usize = 2;

    #[allow(dead_code)]
    /// The maximum value allowed for this enumeration variant.
    pub const MASK: u8 = 0b11;

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_defined(self) -> bool {
        match self.0 {
            0 | 1 | 2 => true,
            _ => false,
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u8 { self.0 }

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(val: u8) -> Self {
        debug_assert!(val <= Self::MASK);
        Self(val)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn try_from_bits(val: u8) -> Result<Self, BitstructError> {
        let s = Self(val);
        if s.is_defined() { Ok(s) } else { Err(BitstructError::InvalidVariant { value: val as u128, enum_name: "ConnectionState" }) }
    }
}

// And for SignedState:
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SignedState(pub i8);

impl SignedState {
    pub const MIN: Self = Self(-4);
    pub const ERROR: Self = Self(-1);
    pub const OK: Self = Self(0);
    pub const MAX: Self = Self(3);

    pub const BITS: usize = 3;

    #[inline(always)]
    pub const fn is_defined(self) -> bool {
        match self.0 {
            -4 | -1 | 0 | 3 => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub const fn to_bits(self) -> i8 { self.0 }

    #[inline(always)]
    pub const fn from_bits(mut val: i8) -> Self {
        // Dynamic zero-cost sign extension via shift
        val = (val << 5) >> 5;
        debug_assert!(val >= Self::MIN.0 && val <= Self::MAX.0);
        Self(val)
    }

    #[inline(always)]
    pub const fn try_from_bits(mut val: i8) -> Result<Self, BitstructError> {
        val = (val << 5) >> 5;
        let s = Self(val);
        if s.is_defined() { Ok(s) } else { Err(BitstructError::InvalidVariant { value: (val as i128) as u128, enum_name: "SignedState" }) }
    }
}
```

---

## 2. `bitstruct!` (Word-Aligned Packing)

Packs multiple logical fields into a single standard CPU register (`u8`-`u128`, `i8`-`i128`). Now utilizes named constants for offsets and masks, and strictly respects signed boundaries.

### **Usage**

```rust
bitstruct! {
    pub struct Status(u8) {
        pub ready: bool = 1,
        pub mode: u8 = 3,
        pub offset: i8 = 4,
    }
}
```

### **Generated "Struct Equivalent"**

```rust
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Status(pub u8);

impl Status {
    #[allow(dead_code)]
    pub const BITS: usize = 8;

    #[allow(dead_code)]
    const READY_OFFSET: usize = 0;
    #[allow(dead_code)]
    const READY_BITS: usize = 1;
    #[allow(dead_code)]
    const READY_MASK: u8 = 0x01;

    #[inline]
    #[allow(dead_code)]
    pub const fn ready(self) -> bool {
        ((self.0 >> Self::READY_OFFSET) & Self::READY_MASK) != 0
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_ready(&mut self, val: bool) {
        let val_masked = val as u8;
        self.0 = (self.0 & !(Self::READY_MASK << Self::READY_OFFSET))
               | (val_masked << Self::READY_OFFSET);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn try_set_ready(&mut self, val: bool) -> Result<(), BitstructError> {
        self.set_ready(val); Ok(())
    }

    #[allow(dead_code)]
    const MODE_OFFSET: usize = 1;
    #[allow(dead_code)]
    const MODE_BITS: usize = 3;
    #[allow(dead_code)]
    const MODE_MASK: u8 = 0x07;

    #[inline]
    #[allow(dead_code)]
    pub const fn mode(self) -> u8 {
        ((self.0 >> Self::MODE_OFFSET) & Self::MODE_MASK) as u8
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_mode(&mut self, val: u8) {
        debug_assert!(val <= Self::MODE_MASK);
        let val_masked = val & Self::MODE_MASK;
        self.0 = (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
               | (val_masked << Self::MODE_OFFSET);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn try_set_mode(&mut self, val: u8) -> Result<(), BitstructError> {
        if val > Self::MODE_MASK { return Err(BitstructError::Overflow { ... }); }
        self.set_mode(val); Ok(())
    }

    #[allow(dead_code)]
    pub const OFFSET_OFFSET: usize = 4;
    #[allow(dead_code)]
    pub const OFFSET_BITS: usize = 4;
    #[allow(dead_code)]
    const OFFSET_MASK: u8 = 0x0F;

    #[doc(hidden)]
    pub const OFFSET_MIN: i8 = -8;
    #[doc(hidden)]
    pub const OFFSET_MAX: i8 = 7;
    #[doc(hidden)]
    const OFFSET_SHIFT_UP: usize = 8 - Self::OFFSET_BITS;

    #[inline]
    #[allow(dead_code)]
    pub const fn offset(self) -> i8 {
        let raw = ((self.0 >> Self::OFFSET_OFFSET) & Self::OFFSET_MASK) as i8;
        (raw << Self::OFFSET_SHIFT_UP) >> Self::OFFSET_SHIFT_UP
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_offset(&mut self, val: i8) {
        debug_assert!(val >= Self::OFFSET_MIN && val <= Self::OFFSET_MAX);
        let val_masked = (val as u8) & Self::OFFSET_MASK;
        self.0 = (self.0 & !(Self::OFFSET_MASK << Self::OFFSET_OFFSET))
               | (val_masked << Self::OFFSET_OFFSET);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn try_set_offset(&mut self, val: i8) -> Result<(), BitstructError> {
        if val < Self::OFFSET_MIN || val > Self::OFFSET_MAX { return Err(BitstructError::Overflow { ... }); }
        self.set_offset(val); Ok(())
    }
}
```

---

## 3. `bytestruct!` (Array-Backed Dense Packing)

Native support for arbitrary-length dense buffers (`[u8; N]`) using **Const-Generic Helper Functions** and alignment-aware fast paths.

### **Usage**

```rust
bytestruct! {
    pub struct Node(2) {
        pub a: u8 = 4,
        pub b: u16 = 12,
    }
}
```

### **Generated "Struct Equivalent"**

The macro generates a **"Literal Guard"** pattern inside optimized helper functions:

```rust
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Node(pub [u8; 2]);

impl Node {
    #[allow(dead_code)]
    pub const BITS: usize = 16;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = 8;

    #[allow(dead_code)]
    pub const B_OFFSET: usize = 4;
    #[allow(dead_code)]
    pub const B_BITS: usize = 12;
    #[allow(dead_code)]
    const B_MASK: u32 = 0x0FFF;

    #[inline]
    #[allow(dead_code)]
    pub const fn b(self) -> u16 {
        // The unrolling engine generates a direct bitwise expression:
        (((self.0[0] as u32 | (self.0[1] as u32 << 8)) >> 4) & Self::B_MASK) as u16
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_b(&mut self, val: u16) {
        debug_assert!((val as u32) <= Self::B_MASK);
        let mut full = (self.0[0] as u32 | (self.0[1] as u32 << 8));
        full = (full & !(Self::B_MASK << 4)) | ((val as u32 & Self::B_MASK) << 4);
        self.0[0] = full as u8;
        self.0[1] = (full >> 8) as u8;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn try_set_b(&mut self, val: u16) -> Result<(), BitstructError> {
        if (val as u32) > Self::B_MASK { return Err(...); }
        self.set_b(val); Ok(())
    }
}
```

---

## 4. `byteval!` (Packed ID NewTypes)

Shorthand for "Odd-Width" integers like 24-bit or 40-bit IDs. Utilizes the same optimized helpers as `bytestruct!`.

### **Usage**

```rust
byteval! { pub struct Id24(3); }
byteval! { pub struct SignedId24(i 3); }
```

### **Generated "Struct Equivalent"**

```rust
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Id24(pub [u8; 3]);

impl Id24 {
    #[allow(dead_code)]
    pub const BITS: usize = 24;

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_u32(self) -> u32 {
        // Fully unrolled bitwise OR chain
        (self.0[0] as u32) | (self.0[1] as u32 << 8) | (self.0[2] as u32 << 16)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_u32(val: u32) -> Self {
        // Efficient array constructor initialization (Fresh Write path)
        Self([val as u8, (val >> 8) as u8, (val >> 16) as u8])
    }

    #[inline]
    #[allow(dead_code)]
    pub fn try_set_value(&mut self, val: u32) -> Result<(), BitstructError> {
        if val > 0xFF_FFFF { return Err(...); }
        self.0 = Self::from_u32(val).0; Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub const fn value(self) -> u32 { self.to_u32() }
}

// And for SignedId24:
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct SignedId24(pub [u8; 3]);

impl SignedId24 {
    #[allow(dead_code)]
    pub const BITS: usize = 24;

    #[allow(dead_code)]
    const VALUE_SHIFT_UP: usize = 32 - 24;

    #[inline(always)]
    #[allow(dead_code)]
    pub const fn value(self) -> i32 {
        let val = (self.0[0] as u32) | (self.0[1] as u32 << 8) | (self.0[2] as u32 << 16);
        let mut signed_val = val as i32;
        signed_val = (signed_val << Self::VALUE_SHIFT_UP) >> Self::VALUE_SHIFT_UP;
        signed_val
    }

    #[inline]
    #[allow(dead_code)]
    pub fn try_set_value(&mut self, val: i32) -> Result<(), BitstructError> {
        if val < -8388608 || val > 8388607 { return Err(...); }
        // Mask and pack into array
        self.0[0] = (val & 0xFF) as u8;
        // ...
        Ok(())
    }
}
```

---

## 5. `atomic_bitstruct!` (Lock-Free Concurrency)

Generates a thread-safe bitstruct backed by a `core::sync::atomic` type, providing lock-free getters, setters, and transaction-safe CAS loop closures.

### **Usage**

```rust
atomic_bitstruct! {
    pub struct AtomicPoolTracker(AtomicU32) {
        pub is_active: bool = 1,
        pub active_connections: u16 = 15,
    }
}
```

### **Generated "Struct Equivalent"**

The macro generates two things: the Atomic outer struct and its non-atomic `Value` companion snapshot struct.

```rust
// The Companion Non-Atomic Snapshot Struct (Generated via standard bitstruct!)
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct AtomicPoolTrackerValue(pub u32);

impl AtomicPoolTrackerValue {
    // ... standard bitstruct! methods (set_is_active, active_connections, etc.) ...
}

// The Outer Atomic Struct
#[repr(transparent)]
pub struct AtomicPoolTracker(pub core::sync::atomic::AtomicU32);

impl AtomicPoolTracker {
    #[inline(always)]
    pub const fn new(val: u32) -> Self {
        Self(core::sync::atomic::AtomicU32::new(val))
    }

    #[inline(always)]
    pub fn is_active(&self, order: core::sync::atomic::Ordering) -> bool {
        AtomicPoolTrackerValue::from_bits(self.0.load(order)).is_active()
    }

    #[inline(always)]
    pub fn set_is_active(&self, val: bool, order: core::sync::atomic::Ordering) {
        let mut raw = self.0.load(core::sync::atomic::Ordering::Relaxed);
        loop {
            let mut snap = AtomicPoolTrackerValue::from_bits(raw);
            snap.set_is_active(val);
            match self.0.compare_exchange_weak(raw, snap.to_bits(), order, core::sync::atomic::Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => raw = x,
            }
        }
    }

    #[inline]
    pub fn get(&self, order: core::sync::atomic::Ordering) -> AtomicPoolTrackerValue {
        AtomicPoolTrackerValue::from_bits(self.0.load(order))
    }

    #[inline]
    pub fn set(&self, val: AtomicPoolTrackerValue, order: core::sync::atomic::Ordering) {
        self.0.store(val.to_bits(), order);
    }

    #[inline]
    pub fn update_or_abort<F>(&self, set_order: core::sync::atomic::Ordering, fetch_order: core::sync::atomic::Ordering, mut f: F) -> Result<AtomicPoolTrackerValue, AtomicPoolTrackerValue>
    where
        F: FnMut(&mut AtomicPoolTrackerValue) -> Option<()>
    {
        self.0.fetch_update(set_order, fetch_order, |raw| {
            let mut snap = AtomicPoolTrackerValue::from_bits(raw);
            f(&mut snap).map(|_| snap.to_bits())
        }).map(|raw| AtomicPoolTrackerValue::from_bits(raw))
        .map_err(|raw| AtomicPoolTrackerValue::from_bits(raw))
    }
}
```

---

## 🔍 Visibility and Inspection

To see the real, machine-generated expansion for any macro in your project, install the `cargo-expand` tool:

```bash
cargo install cargo-expand
cargo expand --example your_example
```

Our build system provides a filtered expansion specifically for the engine's internals via `make expand-sample`, which generates the clean `sample/expanded_sample.rs` file you see in the repository.
