# Bitstruct Macro Showcase & Expansion ⚙️

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

    /// The number of bits allocated for this enumeration in memory.
    pub const BITS: usize = 2;

    /// The maximum value allowed for this enumeration variant.
    pub const MASK: u8 = 0b11;

    #[inline(always)]
    pub const fn is_defined(self) -> bool {
        match self.0 {
            0 | 1 | 2 => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub const fn to_bits(self) -> u8 { self.0 }

    #[inline(always)]
    pub const fn from_bits(val: u8) -> Self {
        debug_assert!(val <= Self::MASK);
        Self(val)
    }

    #[inline(always)]
    pub const fn try_from_bits(val: u8) -> Result<Self, BitstructError> {
        let s = Self(val);
        if s.is_defined() { Ok(s) } else { Err(...) }
    }
}
```

---

## 2. `bitstruct!` (Word-Aligned Packing)

Packs multiple logical fields into a single standard CPU register (`u8`-`u128`). Now utilizes named constants for offsets and masks.

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
    const READY_OFFSET: usize = 0;
    const READY_MASK: u8 = 0x01;

    #[inline]
    pub const fn ready(self) -> bool {
        ((self.0 >> Self::READY_OFFSET) & Self::READY_MASK) != 0
    }

    #[inline]
    pub fn set_ready(&mut self, val: bool) {
        let val_masked = val as u8;
        self.0 = (self.0 & !(Self::READY_MASK << Self::READY_OFFSET))
               | (val_masked << Self::READY_OFFSET);
    }

    const MODE_OFFSET: usize = 1;
    const MODE_MASK: u8 = 0x07;

    #[inline]
    pub const fn mode(self) -> u8 {
        ((self.0 >> Self::MODE_OFFSET) & Self::MODE_MASK) as u8
    }

    #[inline]
    pub fn set_mode(&mut self, val: u8) {
        debug_assert!(val <= Self::MODE_MASK);
        let val_masked = val & Self::MODE_MASK;
        self.0 = (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
               | (val_masked << Self::MODE_OFFSET);
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
pub struct Node(pub [u8; 2]);

impl Node {
    const B_OFFSET: usize = 4;
    const B_MASK: u128 = 0x0FFF;

    #[inline]
    pub const fn b(self) -> u16 {
        // Optimized helper: for this case, it performs a 16-bit LE-load 
        // since the span is byte-aligned (after internal resolution).
        let val = ::bitcraft::read_le_bits::<{ Self::B_OFFSET }, 12, _>(&self.0);
        val as u16
    }

    #[inline]
    pub fn set_b(&mut self, val: u16) {
        debug_assert!((val as u128) <= Self::B_MASK);
        ::bitcraft::write_le_bits::<{ Self::B_OFFSET }, 12, _>(&mut self.0, val as u128);
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
pub struct Id24(pub [u8; 3]);

impl Id24 {
    /// Native conversion to the widest capable primitive.
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        // Unrolled bitwise ORs: (arr[0]) | (arr[1] << 8) | (arr[2] << 16)
        // Highly optimized by LLVM into a single 24-bit aligned load.
        let mut val = 0u32;
        let mut i = 0;
        while i < 3 { val |= (self.0[i] as u32) << (i << 3); i += 1; }
        val
    }

    #[inline(always)]
    pub const fn to_bits(self) -> u32 { self.to_u32() }

    const VALUE_OFFSET: usize = 0;
    const VALUE_MASK: u128 = 0x00FFFFFF;

    #[inline]
    pub const fn value(self) -> u32 {
        let val = ::bitcraft::read_le_bits::<{ Self::VALUE_OFFSET }, 24, _>(&self.0);
        val as u32
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
