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
        if s.is_defined() { Ok(s) } else { Err(BitstructError::InvalidVariant) }
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
```

---

## 🔍 Visibility and Inspection

To see the real, machine-generated expansion for any macro in your project, install the `cargo-expand` tool:

```bash
cargo install cargo-expand
cargo expand --example your_example
```

Our build system provides a filtered expansion specifically for the engine's internals via `make expand-sample`, which generates the clean `sample/expanded_sample.rs` file you see in the repository.
