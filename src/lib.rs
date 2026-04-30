#![no_std]
//! # bitcraft
//!
//! **The zero-cost, hardware-aligned bitfield and enumeration engine for Rust.**
//!
//! `bitcraft` is a high-performance declarative macro library designed for systems where every bit counts.
//! It allows defining types where logical fields map directly to bit-ranges with zero runtime overhead.
//!
//! ## Core Macros
//!
//! - **[`bitstruct!`]**: Define a bit-packed struct over a base integer (`u8` to `u128`).
//! - **[`bytestruct!`]**: Define a byte-aligned struct over a fixed-size array (`[u8; 1-16]`).
//! - **[`byteval!`]**: A shorthand for "NewType" byte-array wrappers (e.g., 24-bit IDs) up to 16 bytes.
//! - **[`bitenum!`]**: Define strongly-typed, zero-cost enumerations for use within bitfields.
//!
//! ## 🛠️ Choosing the Right Macro
//!
//! | Macro | Storage | Max Width | Best For... |
//! |-------|---------|-----------|-------------|
//! | **`bitstruct!`** | `u8-u128` | 128 bits | Hardware registers, CPU-native bitmasks. |
//! | **`bytestruct!`** | `[u8; 1-16]` | 128 bits | Smaller network headers, dense buffers. |
//! | **`byteval!`** | `[u8; 1-16]` | 128 bits | Semantic NewTypes (e.g. `Id24`, `MacAddr`). |
//!
//! ## ⚠️ Error Handling & Validation
//!
//! `bitcraft` provides both "failable" and "panicking" APIs for field updates:
//!
//! - **`set_field(val)`**: Panics in debug mode if `val` overflows the allocated bits.
//! - **`try_set_field(val)`**: Returns `Result<(), BitstructError>` if `val` is too large.
//! - **`try_from_bits(val)`**: Validates that raw bits correspond to a defined `bitenum!` variant.
//!
//!
//! - **No Manual Derives**: All generated types automatically implement `Default` (zero-initialization).
//!
//! ## 🔌 No-Std Support
//!
//! `bitcraft` is `#![no_std]` by default and does not require an allocator. It is perfectly suited for bare-metal kernels, embedded firmware, and high-performance drivers.
//!
//! ## 🛡️ Safety & Memory Layout
//!
//! `bitcraft` enforces strict memory layouts to ensure predictability across FFI boundaries and hardware interfaces:
//!
//! - **Transparent Representation**: All generated structs use `#[repr(transparent)]`, guaranteeing they have the exact same size and alignment as their underlying primitive or array.
//! - **LSB-First Packing**: Bits are filled from the least significant bit (index 0) upward.
//! - **Memory Safety**: While the internal helpers use bit-manipulation, the public API is entirely safe. `Default` initialization always results in zeroed memory.
//!
//! ### Visual Packing Example (u8)
//!
//! ```text
//! MSB (Bit 7)                          LSB (Bit 0)
//!  +---+---+---+---+---+---+---+---+
//!  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  <-- Physical Bits
//!  +---+---+---+---+---+---+---+---+
//!  |  Field B (5)  |   Field A (3) |  <-- Logical Fields
//!  +---+---+---+---+---+---+---+---+
//! ```
//!
//!
//! ## 🚀 Performance & Benchmarks
//!
//! `bitcraft` is built with "Mechanical Sympathy" for Little-Endian systems. All operations are `const fn` and utilize LLVM's constant-folding to eliminate branching for constant-width fields.
//!
//! | Category | Hardware Alignment | Overhead vs Manual Code |
//! |----------|-------------------|-------------------------|
//! | **`bitstruct!`** | Word-Aligned (u8-u128) | **0.92x - 0.98x** (Faster) |
//! | **`bytestruct!`** | Array-Backed ([u8; N]) | **~2.5x** |
//! | **`byteval!`** | Odd-Width IDs (24-bit) | **0.94x - 0.97x** (Faster) |
//! | **`bitenum!`** | Specialized Enum | **0.97x - 0.99x** (Parity) |
//!
//! *Benchmarks performed on a 1B iteration loop. "Faster than manual" is achieved through instruction fusion and aligned register loading.*
//!

//!
//! ## 🧩 Showcasing Interoperability
//!
//! `bitcraft` is designed for high-performance systems where data must move seamlessly between the CPU, the network, and other languages.
//!
//! ### Zero-Copy Network & Buffer Parsing
//!
//! Using the [**`bytemuck`**](https://docs.rs/bytemuck) crate, you can cast raw byte slices directly into typed bitfields with zero memory movement:
//!
//! ```rust
//! # use bitcraft::bitstruct;
//! bitstruct! {
//!     pub struct EthernetHeader(u16) {
//!         pub protocol: u16 = 16,
//!     }
//! }
//!
//! let data = [0x08, 0x00]; // IPv4 EtherType
//! // No allocation, no parsing loop, just a typed overlay.
//! let header: &EthernetHeader = bytemuck::from_bytes(&data);
//! ```
//!
//! ### Foreign Function Interface (C-ABI)
//!
//! Every `bitstruct!` and `bytestruct!` is marked `#[repr(transparent)]`. This guarantees that its memory layout is identical to the underlying primitive, making it safe to pass directly to C/C++ libraries.
//!
//! ```rust
//! # use bitcraft::bitstruct;
//! bitstruct! {
//!     pub struct FfiFlags(u8) {
//!         pub readable: bool = 1,
//!         pub writable: bool = 1,
//!     }
//! }
//!
//! unsafe extern "C" {
//!     // Correctly passes as a 'uint8_t' in C
//!     fn c_handle_flags(flags: FfiFlags);
//! }
//! ```
//!
//! ### Hardware Register Access (MMIO)
//!
//! Because `bitcraft` uses **LSB-first** mapping, your logical field definitions match the physical bit-offsets used in hardware datasheets for Little-Endian systems (x86, ARM).
//!
//! ```rust
//! # use bitcraft::bitstruct;
//! bitstruct! {
//!     pub struct StatusRegister(u32) {
//!         pub ready: bool = 1,
//!         pub error_code: u8 = 4,
//!         pub reserved: u32 = 27,
//!     }
//! }
//!
//! // let reg = StatusRegister::from_bits(mmio_addr.read_volatile());
//! ```
//!
//!
//! ## Usage Example
//!
//! ```rust
//! use bitcraft::{bitstruct, bitenum};
//!
//! bitenum! {
//!     /// Represent the state of an active process.
//!     pub enum ProcessState(2) {
//!         IDLE = 0,
//!         RUNNING = 1,
//!         BLOCKED = 2,
//!     }
//! }
//!
//! bitstruct! {
//!     /// A 16-bit packed struct.
//!     pub struct ProcessDescriptor(u16) {
//!         pub is_privileged: bool = 1,       // Bit 0
//!         pub priority: u8 = 3,              // Bits 1-3
//!         pub memory_pages: u16 = 10,        // Bits 4-13
//!         pub state: ProcessState = 2,       // Bits 14-15
//!     }
//! }
//!
//! fn main() {
//!     let mut desc = ProcessDescriptor::default()
//!         .with_is_privileged(true)
//!         .with_priority(5)
//!         .with_memory_pages(1023)
//!         .with_state(ProcessState::RUNNING);
//!
//!     assert_eq!(desc.priority(), 5);
//!     assert_eq!(desc.state(), ProcessState::RUNNING);
//!
//!     desc.set_state(ProcessState::BLOCKED);
//!     assert_eq!(desc.state(), ProcessState::BLOCKED);
//! }
//! ```

#[doc(hidden)]
pub use bytemuck;
#[doc(hidden)]
pub use paste;

#[doc(hidden)]
pub mod reexport {
    pub use portable_atomic;
}

pub use portable_atomic::Ordering;

/// Error types returned by strict data structures when encountering invalid operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitstructError {
    /// Indicates an attempt to set a value that requires more bits than allocated.
    ///
    /// This is returned by `try_set_*` and `try_with_*` methods when the input value
    /// exceeds the bit-width specified in the macro definition.
    Overflow {
        /// The value that caused the overflow, upcast to u128 for universal representation.
        value: u128,
        /// The maximum number of bits allocated for this specific field.
        allocated_bits: usize,
    },
    /// Indicates an attempt to decode a raw value into an enumeration variant that is not defined.
    ///
    /// This is returned by `bitenum!`'s `try_from_bits` method.
    InvalidVariant {
        /// The raw value that could not be mapped to any variant.
        value: u128,
        /// The name of the enumeration type.
        enum_name: &'static str,
    },
}

impl core::fmt::Display for BitstructError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow {
                value,
                allocated_bits,
            } => {
                write!(
                    f,
                    "BitstructError: Value {} overflows allocated {} bits",
                    value, allocated_bits
                )
            }
            Self::InvalidVariant { value, enum_name } => {
                write!(
                    f,
                    "BitstructError: Value {} is not a valid variant for enum {}",
                    value, enum_name
                )
            }
        }
    }
}

impl core::error::Error for BitstructError {}

/// **Internal Trait**: Used to enforce that only valid integers are used as base types,
/// and to route primitive bounds and masking behaviors.
///
/// This trait prevents "Two's Complement Ambiguity" when dealing with signed base types
/// by enforcing a `MAX_BITS` limit (e.g. 15 bits for `i16`, isolating the sign bit)
/// and by providing an `Unsigned` type to safely compute bitwise masks without sign-extension.
#[doc(hidden)]
pub trait IsValidBaseInt {
    /// A constant that confirms this type is valid at compile-time.
    const ASSERT_VALID: ();
    /// The maximum number of bits this type is allowed to store in bitstruct fields.
    /// For unsigned integers, this equals the type's total bits. For signed integers, this equals
    /// the type's total bits minus 1, explicitly reserving and isolating the sign bit.
    const MAX_BITS: usize;
    /// The corresponding unsigned type (e.g., `u32` for `i32`), used to perform safe bitwise
    /// shifts during mask generation to avoid arithmetic right shifts (sign-extension bugs).
    type Unsigned: BitLength;
}

macro_rules! impl_is_valid_base {
    ($type:ty, $unsigned:ty, $max_bits:expr) => {
        impl IsValidBaseInt for $type {
            const ASSERT_VALID: () = ();
            const MAX_BITS: usize = $max_bits;
            type Unsigned = $unsigned;
        }
    };
}

impl_is_valid_base!(u8, u8, 8);
impl_is_valid_base!(u16, u16, 16);
impl_is_valid_base!(u32, u32, 32);
impl_is_valid_base!(u64, u64, 64);
impl_is_valid_base!(u128, u128, 128);
impl_is_valid_base!(i8, u8, 7);
impl_is_valid_base!(i16, u16, 15);
impl_is_valid_base!(i32, u32, 31);
impl_is_valid_base!(i64, u64, 63);
impl_is_valid_base!(i128, u128, 127);

/// Common bit-width constants for use in manual calculations or macro overrides.
pub const U8_BITS: usize = 8;
pub const U8_BITS_2: usize = 16;
pub const U8_BITS_3: usize = 24;
pub const U8_BITS_4: usize = 32;
pub const U8_BITS_5: usize = 40;
pub const U8_BITS_6: usize = 48;
pub const U8_BITS_7: usize = 56;
pub const U8_BITS_8: usize = 64;
pub const U8_BITS_9: usize = 72;
pub const U8_BITS_10: usize = 80;
pub const U8_BITS_11: usize = 88;
pub const U8_BITS_12: usize = 96;
pub const U8_BITS_13: usize = 104;
pub const U8_BITS_14: usize = 112;
pub const U8_BITS_15: usize = 120;
pub const U8_BITS_16: usize = 128;

pub const U16_BITS: usize = 16;
pub const U16_BITS_2: usize = 32;
pub const U16_BITS_3: usize = 48;
pub const U16_BITS_4: usize = 64;
pub const U16_BITS_5: usize = 80;
pub const U16_BITS_6: usize = 96;
pub const U16_BITS_7: usize = 112;
pub const U16_BITS_8: usize = 128;

pub const U32_BITS: usize = 32;
pub const U32_BITS_2: usize = 64;
pub const U32_BITS_3: usize = 96;
pub const U32_BITS_4: usize = 128;

pub const U64_BITS: usize = 64;
pub const U64_BITS_2: usize = 128;

pub const U128_BITS: usize = 128;

/// **Internal Trait**: Used to retrieve the bit-width of a primitive type or array.
#[doc(hidden)]
pub trait BitLength {
    /// The number of bits in the type.
    const BITS: usize;
    /// Two times the number of bits in the type.
    const BITS_2: usize;
    /// Three times the number of bits in the type.
    const BITS_3: usize;
    /// Four times the number of bits in the type.
    const BITS_4: usize;
    /// Five times the number of bits in the type.
    const BITS_5: usize;
    /// Six times the number of bits in the type.
    const BITS_6: usize;
    /// Seven times the number of bits in the type.
    const BITS_7: usize;
    /// Eight times the number of bits in the type.
    const BITS_8: usize;
    const BITS_9: usize;
    const BITS_10: usize;
    const BITS_11: usize;
    const BITS_12: usize;
    const BITS_13: usize;
    const BITS_14: usize;
    const BITS_15: usize;
    const BITS_16: usize;
}

impl BitLength for u8 {
    const BITS: usize = U8_BITS;
    const BITS_2: usize = U8_BITS_2;
    const BITS_3: usize = U8_BITS_3;
    const BITS_4: usize = U8_BITS_4;
    const BITS_5: usize = U8_BITS_5;
    const BITS_6: usize = U8_BITS_6;
    const BITS_7: usize = U8_BITS_7;
    const BITS_8: usize = U8_BITS_8;
    const BITS_9: usize = U8_BITS_9;
    const BITS_10: usize = U8_BITS_10;
    const BITS_11: usize = U8_BITS_11;
    const BITS_12: usize = U8_BITS_12;
    const BITS_13: usize = U8_BITS_13;
    const BITS_14: usize = U8_BITS_14;
    const BITS_15: usize = U8_BITS_15;
    const BITS_16: usize = U8_BITS_16;
}
impl BitLength for u16 {
    const BITS: usize = U16_BITS;
    const BITS_2: usize = U16_BITS_2;
    const BITS_3: usize = U16_BITS_3;
    const BITS_4: usize = U16_BITS_4;
    const BITS_5: usize = U16_BITS_5;
    const BITS_6: usize = U16_BITS_6;
    const BITS_7: usize = U16_BITS_7;
    const BITS_8: usize = U16_BITS_8;
    const BITS_9: usize = U16_BITS * 9;
    const BITS_10: usize = U16_BITS * 10;
    const BITS_11: usize = U16_BITS * 11;
    const BITS_12: usize = U16_BITS * 12;
    const BITS_13: usize = U16_BITS * 13;
    const BITS_14: usize = U16_BITS * 14;
    const BITS_15: usize = U16_BITS * 15;
    const BITS_16: usize = U16_BITS * 16;
}
impl BitLength for u32 {
    const BITS: usize = U32_BITS;
    const BITS_2: usize = U32_BITS_2;
    const BITS_3: usize = U32_BITS_3;
    const BITS_4: usize = U32_BITS_4;
    const BITS_5: usize = U32_BITS * 5;
    const BITS_6: usize = U32_BITS * 6;
    const BITS_7: usize = U32_BITS * 7;
    const BITS_8: usize = U32_BITS * 8;
    const BITS_9: usize = U32_BITS * 9;
    const BITS_10: usize = U32_BITS * 10;
    const BITS_11: usize = U32_BITS * 11;
    const BITS_12: usize = U32_BITS * 12;
    const BITS_13: usize = U32_BITS * 13;
    const BITS_14: usize = U32_BITS * 14;
    const BITS_15: usize = U32_BITS * 15;
    const BITS_16: usize = U32_BITS * 16;
}
impl BitLength for u64 {
    const BITS: usize = U64_BITS;
    const BITS_2: usize = U64_BITS_2;
    const BITS_3: usize = U64_BITS * 3;
    const BITS_4: usize = U64_BITS * 4;
    const BITS_5: usize = U64_BITS * 5;
    const BITS_6: usize = U64_BITS * 6;
    const BITS_7: usize = U64_BITS * 7;
    const BITS_8: usize = U64_BITS * 8;
    const BITS_9: usize = U64_BITS * 9;
    const BITS_10: usize = U64_BITS * 10;
    const BITS_11: usize = U64_BITS * 11;
    const BITS_12: usize = U64_BITS * 12;
    const BITS_13: usize = U64_BITS * 13;
    const BITS_14: usize = U64_BITS * 14;
    const BITS_15: usize = U64_BITS * 15;
    const BITS_16: usize = U64_BITS * 16;
}
impl BitLength for u128 {
    const BITS: usize = U128_BITS;
    const BITS_2: usize = U128_BITS * 2;
    const BITS_3: usize = U128_BITS * 3;
    const BITS_4: usize = U128_BITS * 4;
    const BITS_5: usize = U128_BITS * 5;
    const BITS_6: usize = U128_BITS * 6;
    const BITS_7: usize = U128_BITS * 7;
    const BITS_8: usize = U128_BITS * 8;
    const BITS_9: usize = U128_BITS * 9;
    const BITS_10: usize = U128_BITS * 10;
    const BITS_11: usize = U128_BITS * 11;
    const BITS_12: usize = U128_BITS * 12;
    const BITS_13: usize = U128_BITS * 13;
    const BITS_14: usize = U128_BITS * 14;
    const BITS_15: usize = U128_BITS * 15;
    const BITS_16: usize = U128_BITS * 16;
}
impl BitLength for i8 {
    const BITS: usize = U8_BITS;
    const BITS_2: usize = U8_BITS_2;
    const BITS_3: usize = U8_BITS_3;
    const BITS_4: usize = U8_BITS_4;
    const BITS_5: usize = U8_BITS_5;
    const BITS_6: usize = U8_BITS_6;
    const BITS_7: usize = U8_BITS_7;
    const BITS_8: usize = U8_BITS_8;
    const BITS_9: usize = U8_BITS_9;
    const BITS_10: usize = U8_BITS_10;
    const BITS_11: usize = U8_BITS_11;
    const BITS_12: usize = U8_BITS_12;
    const BITS_13: usize = U8_BITS_13;
    const BITS_14: usize = U8_BITS_14;
    const BITS_15: usize = U8_BITS_15;
    const BITS_16: usize = U8_BITS_16;
}
impl BitLength for i16 {
    const BITS: usize = U16_BITS;
    const BITS_2: usize = U16_BITS_2;
    const BITS_3: usize = U16_BITS_3;
    const BITS_4: usize = U16_BITS_4;
    const BITS_5: usize = U16_BITS_5;
    const BITS_6: usize = U16_BITS_6;
    const BITS_7: usize = U16_BITS_7;
    const BITS_8: usize = U16_BITS_8;
    const BITS_9: usize = U16_BITS * 9;
    const BITS_10: usize = U16_BITS * 10;
    const BITS_11: usize = U16_BITS * 11;
    const BITS_12: usize = U16_BITS * 12;
    const BITS_13: usize = U16_BITS * 13;
    const BITS_14: usize = U16_BITS * 14;
    const BITS_15: usize = U16_BITS * 15;
    const BITS_16: usize = U16_BITS * 16;
}
impl BitLength for i32 {
    const BITS: usize = U32_BITS;
    const BITS_2: usize = U32_BITS_2;
    const BITS_3: usize = U32_BITS_3;
    const BITS_4: usize = U32_BITS_4;
    const BITS_5: usize = U32_BITS * 5;
    const BITS_6: usize = U32_BITS * 6;
    const BITS_7: usize = U32_BITS * 7;
    const BITS_8: usize = U32_BITS * 8;
    const BITS_9: usize = U32_BITS * 9;
    const BITS_10: usize = U32_BITS * 10;
    const BITS_11: usize = U32_BITS * 11;
    const BITS_12: usize = U32_BITS * 12;
    const BITS_13: usize = U32_BITS * 13;
    const BITS_14: usize = U32_BITS * 14;
    const BITS_15: usize = U32_BITS * 15;
    const BITS_16: usize = U32_BITS * 16;
}
impl BitLength for i64 {
    const BITS: usize = U64_BITS;
    const BITS_2: usize = U64_BITS_2;
    const BITS_3: usize = U64_BITS * 3;
    const BITS_4: usize = U64_BITS * 4;
    const BITS_5: usize = U64_BITS * 5;
    const BITS_6: usize = U64_BITS * 6;
    const BITS_7: usize = U64_BITS * 7;
    const BITS_8: usize = U64_BITS * 8;
    const BITS_9: usize = U64_BITS * 9;
    const BITS_10: usize = U64_BITS * 10;
    const BITS_11: usize = U64_BITS * 11;
    const BITS_12: usize = U64_BITS * 12;
    const BITS_13: usize = U64_BITS * 13;
    const BITS_14: usize = U64_BITS * 14;
    const BITS_15: usize = U64_BITS * 15;
    const BITS_16: usize = U64_BITS * 16;
}
impl BitLength for i128 {
    const BITS: usize = U128_BITS;
    const BITS_2: usize = U128_BITS * 2;
    const BITS_3: usize = U128_BITS * 3;
    const BITS_4: usize = U128_BITS * 4;
    const BITS_5: usize = U128_BITS * 5;
    const BITS_6: usize = U128_BITS * 6;
    const BITS_7: usize = U128_BITS * 7;
    const BITS_8: usize = U128_BITS * 8;
    const BITS_9: usize = U128_BITS * 9;
    const BITS_10: usize = U128_BITS * 10;
    const BITS_11: usize = U128_BITS * 11;
    const BITS_12: usize = U128_BITS * 12;
    const BITS_13: usize = U128_BITS * 13;
    const BITS_14: usize = U128_BITS * 14;
    const BITS_15: usize = U128_BITS * 15;
    const BITS_16: usize = U128_BITS * 16;
}
impl<const N: usize> BitLength for [u8; N] {
    const BITS: usize = N << 3;
    const BITS_2: usize = N << 4;
    const BITS_3: usize = (N * 8) * 3;
    const BITS_4: usize = N << 5;
    const BITS_5: usize = (N * 8) * 5;
    const BITS_6: usize = (N * 8) * 6;
    const BITS_7: usize = (N * 8) * 7;
    const BITS_8: usize = N << 6;
    const BITS_9: usize = (N * 8) * 9;
    const BITS_10: usize = (N * 8) * 10;
    const BITS_11: usize = (N * 8) * 11;
    const BITS_12: usize = (N * 8) * 12;
    const BITS_13: usize = (N * 8) * 13;
    const BITS_14: usize = (N * 8) * 14;
    const BITS_15: usize = (N * 8) * 15;
    const BITS_16: usize = N << 7;
}

/// **Internal Trait**: Used to enforce that only valid types are used as fields.
#[doc(hidden)]
pub trait ValidField {
    const ASSERT_VALID: ();
}

impl ValidField for bool {
    const ASSERT_VALID: () = ();
}
impl ValidField for u8 {
    const ASSERT_VALID: () = ();
}
impl ValidField for u16 {
    const ASSERT_VALID: () = ();
}
impl ValidField for u32 {
    const ASSERT_VALID: () = ();
}
impl ValidField for u64 {
    const ASSERT_VALID: () = ();
}
impl ValidField for u128 {
    const ASSERT_VALID: () = ();
}
// Signed primitives natively handle two's complement evaluation and sign-extension
// safely via mathematical shifting across underlying base boundaries. These primitives
// represent the external API surface mapped securely into the bitfields.
impl ValidField for i8 {
    const ASSERT_VALID: () = ();
}
impl ValidField for i16 {
    const ASSERT_VALID: () = ();
}
impl ValidField for i32 {
    const ASSERT_VALID: () = ();
}
impl ValidField for i64 {
    const ASSERT_VALID: () = ();
}
impl ValidField for i128 {
    const ASSERT_VALID: () = ();
}

/// **Internal Function**: Reads bit-ranges from a byte array. Optimized via const-generics and alignment-aware fast paths.
///
/// # Arguments
/// * `SHIFT` - The bit offset from the start of the array.
/// * `BITS` - The number of bits to read.
/// * `N` - The total size of the byte array.
///
/// This function utilizes LLVM's constant folding to eliminate branches. For byte-aligned fields,
/// it bypasses bit-shifting loops entirely and uses direct LE-byte conversion.
#[doc(hidden)]
#[inline(always)]
const fn read_aligned_le_bits<const BITS: usize, const N: usize>(
    arr: &[u8; N],
    byte_index: usize,
) -> Option<u128> {
    if BITS == 8 {
        return Some(arr[byte_index] as u128);
    }
    if BITS == 16 && byte_index + 2 <= N {
        let value = (arr[byte_index] as u16) | (arr[byte_index + 1] as u16) << 8;
        return Some(value as u128);
    }
    if BITS == 32 && byte_index + 4 <= N {
        let value = (arr[byte_index] as u32)
            | (arr[byte_index + 1] as u32) << 8
            | (arr[byte_index + 2] as u32) << 16
            | (arr[byte_index + 3] as u32) << 24;
        return Some(value as u128);
    }
    if BITS == 64 && byte_index + 8 <= N {
        let value = (arr[byte_index] as u64)
            | (arr[byte_index + 1] as u64) << 8
            | (arr[byte_index + 2] as u64) << 16
            | (arr[byte_index + 3] as u64) << 24
            | (arr[byte_index + 4] as u64) << 32
            | (arr[byte_index + 5] as u64) << 40
            | (arr[byte_index + 6] as u64) << 48
            | (arr[byte_index + 7] as u64) << 56;
        return Some(value as u128);
    }
    if BITS == 128 && byte_index + 16 <= N {
        return Some(
            (arr[byte_index] as u128)
                | (arr[byte_index + 1] as u128) << 8
                | (arr[byte_index + 2] as u128) << 16
                | (arr[byte_index + 3] as u128) << 24
                | (arr[byte_index + 4] as u128) << 32
                | (arr[byte_index + 5] as u128) << 40
                | (arr[byte_index + 6] as u128) << 48
                | (arr[byte_index + 7] as u128) << 56
                | (arr[byte_index + 8] as u128) << 64
                | (arr[byte_index + 9] as u128) << 72
                | (arr[byte_index + 10] as u128) << 80
                | (arr[byte_index + 11] as u128) << 88
                | (arr[byte_index + 12] as u128) << 96
                | (arr[byte_index + 13] as u128) << 104
                | (arr[byte_index + 14] as u128) << 112
                | (arr[byte_index + 15] as u128) << 120,
        );
    }
    None
}

/// **Internal Function**: Reads bit-ranges from a byte array. Optimized via const-generics and alignment-aware fast paths.
///
/// # Arguments
/// * `SHIFT` - The bit offset from the start of the array.
/// * `BITS` - The number of bits to read.
/// * `N` - The total size of the byte array (deduced from `arr`).
///
/// For byte-aligned fields, this function creates a zero-cost mapping to direct byte-array indexing,
/// avoiding expensive multi-byte masks and shifts when possible.
#[doc(hidden)]
#[inline(always)]
pub const fn read_le_bits<
    const SHIFT: usize,
    const BITS: usize,
    const N: usize,
    const BYTE_INDEX: usize,
    const BIT_OFFSET: usize,
    const BYTE_COUNT: usize,
>(
    arr: &[u8; N],
) -> u128 {
    // Fast path: Byte-aligned reads that fit in primitives.
    if BIT_OFFSET == 0 {
        if let Some(val) = read_aligned_le_bits::<BITS, N>(arr, BYTE_INDEX) {
            return val;
        }
    }

    if BYTE_COUNT <= 8 {
        let mut accumulator = 0u64;
        let mut i = 0;
        while i < BYTE_COUNT {
            accumulator |= (arr[BYTE_INDEX + i] as u64) << (i << 3);
            i += 1;
        }
        let bit_mask = if BITS == 64 {
            !0u64
        } else {
            !0u64 >> (64 - BITS)
        };
        ((accumulator >> BIT_OFFSET) & bit_mask) as u128
    } else {
        let mut accumulator = 0u128;
        let mut i = 0;
        while i < BYTE_COUNT {
            accumulator |= (arr[BYTE_INDEX + i] as u128) << (i << 3);
            i += 1;
        }
        let bit_mask = if BITS == 128 {
            !0u128
        } else {
            !0u128 >> (128 - BITS)
        };
        (accumulator >> BIT_OFFSET) & bit_mask
    }
}

#[inline(always)]
const fn write_aligned_le_bits<const BITS: usize, const N: usize>(
    arr: &mut [u8; N],
    byte_index: usize,
    val: u128,
) -> bool {
    if BITS == 8 {
        arr[byte_index] = val as u8;
        return true;
    }
    if BITS == 16 && byte_index + 2 <= N {
        arr[byte_index] = val as u8;
        arr[byte_index + 1] = (val >> 8) as u8;
        return true;
    }
    if BITS == 32 && byte_index + 4 <= N {
        arr[byte_index] = val as u8;
        arr[byte_index + 1] = (val >> 8) as u8;
        arr[byte_index + 2] = (val >> 16) as u8;
        arr[byte_index + 3] = (val >> 24) as u8;
        return true;
    }
    if BITS == 64 && byte_index + 8 <= N {
        arr[byte_index] = val as u8;
        arr[byte_index + 1] = (val >> 8) as u8;
        arr[byte_index + 2] = (val >> 16) as u8;
        arr[byte_index + 3] = (val >> 24) as u8;
        arr[byte_index + 4] = (val >> 32) as u8;
        arr[byte_index + 5] = (val >> 40) as u8;
        arr[byte_index + 6] = (val >> 48) as u8;
        arr[byte_index + 7] = (val >> 56) as u8;
        return true;
    }
    if BITS == 128 && byte_index + 16 <= N {
        arr[byte_index] = val as u8;
        arr[byte_index + 1] = (val >> 8) as u8;
        arr[byte_index + 2] = (val >> 16) as u8;
        arr[byte_index + 3] = (val >> 24) as u8;
        arr[byte_index + 4] = (val >> 32) as u8;
        arr[byte_index + 5] = (val >> 40) as u8;
        arr[byte_index + 6] = (val >> 48) as u8;
        arr[byte_index + 7] = (val >> 56) as u8;
        arr[byte_index + 8] = (val >> 64) as u8;
        arr[byte_index + 9] = (val >> 72) as u8;
        arr[byte_index + 10] = (val >> 80) as u8;
        arr[byte_index + 11] = (val >> 88) as u8;
        arr[byte_index + 12] = (val >> 96) as u8;
        arr[byte_index + 13] = (val >> 104) as u8;
        arr[byte_index + 14] = (val >> 112) as u8;
        arr[byte_index + 15] = (val >> 120) as u8;
        return true;
    }
    false
}

/// **Internal Function**: Writes bit-ranges to a byte array. Optimized via const-generics and alignment-aware fast paths.
///
/// # Arguments
/// * `SHIFT` - The bit offset from the start of the array.
/// * `BITS` - The number of bits to write.
/// * `N` - The total size of the byte array.
///
/// For byte-aligned fields, this function creates a zero-cost mapping to direct byte-array indexing,
/// avoiding expensive multi-byte masks and shifts when possible.
#[doc(hidden)]
#[inline(always)]
pub const fn write_le_bits<
    const SHIFT: usize,
    const BITS: usize,
    const N: usize,
    const BYTE_INDEX: usize,
    const BIT_OFFSET: usize,
    const BYTE_COUNT: usize,
>(
    arr: &mut [u8; N],
    val: u128,
) {
    // Fast path: Byte-aligned writes that fit in primitives.
    if BIT_OFFSET == 0 && write_aligned_le_bits::<BITS, N>(arr, BYTE_INDEX, val) {
        return;
    }

    if BYTE_COUNT <= 8 {
        let mut accumulator = 0u64;
        let mut i = 0;
        while i < BYTE_COUNT {
            accumulator |= (arr[BYTE_INDEX + i] as u64) << (i << 3);
            i += 1;
        }
        let bit_mask = if BITS == 64 {
            !0u64
        } else {
            !0u64 >> (64 - BITS)
        };
        let aligned_mask = bit_mask << BIT_OFFSET;
        accumulator = (accumulator & !aligned_mask) | ((val as u64) << BIT_OFFSET);

        let mut i = 0;
        while i < BYTE_COUNT {
            arr[BYTE_INDEX + i] = (accumulator >> (i << 3)) as u8;
            i += 1;
        }
    } else {
        let mut accumulator = 0u128;
        let mut i = 0;
        while i < BYTE_COUNT {
            accumulator |= (arr[BYTE_INDEX + i] as u128) << (i << 3);
            i += 1;
        }
        let bit_mask = if BITS == 128 {
            !0u128
        } else {
            !0u128 >> (128 - BITS)
        };
        let aligned_mask = bit_mask << BIT_OFFSET;
        accumulator = (accumulator & !aligned_mask) | (val << BIT_OFFSET);

        let mut i = 0;
        while i < BYTE_COUNT {
            arr[BYTE_INDEX + i] = (accumulator >> (i << 3)) as u8;
            i += 1;
        }
    }
}

pub mod atomic_bitarray;
pub mod atomic_bitenum;
mod atomic_bitstruct;
pub mod bitarray;
mod bitenum;
mod bitstruct;
pub mod bytearray;
mod bytestruct;
mod byteval;
mod utils;

#[allow(unused_imports)]
pub use atomic_bitarray::*;
#[allow(unused_imports)]
pub use atomic_bitenum::*;
#[allow(unused_imports)]
pub use atomic_bitstruct::*;
#[allow(unused_imports)]
pub use bitarray::*;
#[allow(unused_imports)]
pub use bytearray::*;
pub use byteval::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use crate::*;

    bitenum! {
        pub enum EngineState(2) {
            OFF = 0,
            IDLE = 1,
            ACTIVE = 2,
        }
    }

    bitstruct! {
        pub struct VehicleState(u16) {
            pub is_running: bool = 1,         // Bit 0
            pub gear: u8 = 3,                 // Bits 1-3
            pub speed: u16 = 8,               // Bits 4-11
            pub state: EngineState = 4,       // Bits 12-15
        }
    }

    #[test]
    fn test_bitenum_conversions() {
        let state = EngineState::ACTIVE;
        assert_eq!(state.to_bits(), 2);

        let parsed = EngineState::from_bits(1);
        assert_eq!(parsed, EngineState::IDLE);
    }

    #[test]
    fn test_bitfield_bool_operations() {
        let mut v = VehicleState::default();
        assert!(!v.is_running());

        v.set_is_running(true);
        assert!(v.is_running());
        assert_eq!(v.to_bits(), 1);

        let v2 = v.with_is_running(false);
        assert!(!v2.is_running());
        assert!(v.is_running()); // Original unchanged
    }

    #[test]
    fn test_bitfield_int_operations() {
        let mut v = VehicleState::default();
        v.set_gear(5); // 5 takes 3 bits (101)
        assert_eq!(v.gear(), 5);

        // Verify strict bounds validation (binary 1000 = 8, which is 4 bits)
        assert_eq!(
            v.try_set_gear(8),
            Err(crate::BitstructError::Overflow {
                value: 8,
                allocated_bits: 3
            })
        );
        assert_eq!(v.gear(), 5); // Value should remain unchanged

        // 7 is the max for 3 bits
        v.set_gear(7);
        assert_eq!(v.gear(), 7);

        v.set_speed(200);
        assert_eq!(v.speed(), 200);

        // Chain with_ builders
        let v2 = VehicleState::default().with_gear(3).with_speed(150);
        assert_eq!(v2.gear(), 3);
        assert_eq!(v2.speed(), 150);

        // from_bits for VehicleState
        assert_eq!(VehicleState::from_bits(v2.to_bits()), v2);
    }

    #[test]
    fn test_bitfield_enum_operations() {
        let mut v = VehicleState::default();
        assert_eq!(v.state(), EngineState::OFF);

        v.set_state(EngineState::ACTIVE);
        assert_eq!(v.state(), EngineState::ACTIVE);

        let expected_val = 2_u16 << 12; // Active = 2, shifted by 12 bits
        assert_eq!(v.to_bits(), expected_val);

        let v2 = VehicleState::default().with_state(EngineState::IDLE);
        assert_eq!(v2.state(), EngineState::IDLE);
    }

    #[test]
    fn test_dense_packing() {
        let v = VehicleState::default()
            .with_is_running(true) // bit 0: 1
            .with_gear(4) // bits 1-3: 100
            .with_speed(120) // bits 4-11: 01111000
            .with_state(EngineState::ACTIVE); // bits 12-15: 0010

        // 0010_01111000_100_1 in binary = 10113
        assert_eq!(v.to_bits(), 10121);
    }

    #[test]
    fn test_bitfield_u32_u64_operations() {
        bitstruct! {
            pub struct WideConfig(u64) {
                pub a: u32 = 32,
                pub b: u32 = 32,
            }
        }

        let mut cfg = WideConfig::default();
        cfg.set_a(0xDEADBEEF);
        cfg.set_b(0xCAFEBABE);

        assert_eq!(cfg.a(), 0xDEADBEEF);
        assert_eq!(cfg.b(), 0xCAFEBABE);
        assert_eq!(cfg.to_bits(), 0xCAFEBABE_DEADBEEF);

        // bounds masking test for 32-bit width overflow
        cfg.set_a(0xFFFFFFFF);
        assert_eq!(cfg.a(), 0xFFFFFFFF);

        // from_bits for WideConfig
        assert_eq!(WideConfig::from_bits(cfg.to_bits()), cfg);
    }

    #[test]
    fn test_sparse_bit_packing() {
        bitstruct! {
            pub struct SparseConfig(u16) {
                pub first: u8 = 4,    // Bits 0-3
                pub second: u8 = 4,   // Bits 4-7
                // 8 bits unused (implicitly padded)
            }
        }

        // 1. Verify default is zero-initialized (all bits 0)
        let cfg = SparseConfig::default();
        assert_eq!(cfg.to_bits(), 0);

        // 2. Initial setup (bits 0-7: 0xBA, bits 8-15: 0x00)
        let mut cfg = cfg.with_first(0xA).with_second(0xB);
        assert_eq!(cfg.first(), 0xA);
        assert_eq!(cfg.second(), 0xB);
        assert_eq!(cfg.to_bits(), 0x00BA);

        // 3. Mutate (bits 0-7: 0xB1, bits 8-15: 0x00)
        cfg.set_first(0x1);
        assert_eq!(cfg.first(), 0x1);
        assert_eq!(cfg.to_bits(), 0x00B1);

        // 4. Force non-zero into unused bits (8-15) to see if setters preserve them
        // bits 0-7: 0xBA, bits 8-15: 0xFF
        let mut forced = SparseConfig::from_bits(0xFFBA);
        assert_eq!(forced.first(), 0xA);
        assert_eq!(forced.second(), 0xB);
        assert_eq!(forced.to_bits(), 0xFFBA);

        // Mutate field while unused bits are non-zero
        forced.set_first(0xC); // bits 0-7: 0xBC, bits 8-15: 0xFF
        assert_eq!(forced.first(), 0xC);
        assert_eq!(forced.to_bits(), 0xFFBC);
    }

    #[test]
    fn test_setter_isolation() {
        bitstruct! {
            pub struct IsolationTest(u32) {
                pub a: bool = 1,
                pub b: u8 = 7,
                pub c: u16 = 16,
                pub d: EngineState = 8,
            }
        }

        let mut t = IsolationTest::default();

        // Test a (bit 0)
        t.set_a(true);
        assert!(t.a());
        assert_eq!(t.to_bits(), 0x1);

        // Test b (bits 1-7) - should not affect a
        t.set_b(0x7F);
        assert_eq!(t.b(), 0x7F);
        assert!(t.a());
        assert_eq!(t.to_bits(), 0xFF);

        // Test c (bits 8-23) - should not affect a or b
        t.set_c(0xABCD);
        assert_eq!(t.c(), 0xABCD);
        assert_eq!(t.b(), 0x7F);
        assert!(t.a());
        assert_eq!(t.to_bits(), 0x00AB_CDFF);

        // Test d (bits 24-31) - should not affect a, b, or c
        t.set_d(EngineState::ACTIVE);
        assert_eq!(t.d(), EngineState::ACTIVE);
        assert_eq!(t.c(), 0xABCD);
        assert_eq!(t.b(), 0x7F);
        assert!(t.a());
        assert_eq!(t.to_bits(), 0x02AB_CDFF);

        // Builder pattern (with_)
        let t2 = IsolationTest::default()
            .with_a(true)
            .with_b(0x12)
            .with_c(0x5566)
            .with_d(EngineState::IDLE);
        assert!(t2.a());
        assert_eq!(t2.b(), 0x12);
        assert_eq!(t2.c(), 0x5566);
        assert_eq!(t2.d(), EngineState::IDLE);

        // Alias & Roundtrip
        assert_eq!(IsolationTest::from_bits(t2.to_bits()), t2);
    }

    #[test]
    fn test_bytestruct_packing() {
        bytestruct! {
            pub struct NodeLocation(5) {
                pub x: u16 = 16,
                pub y: u16 = 16,
                pub flags: u8 = 8,
            }
        }
        let loc = NodeLocation::from_u64(0x01_AABB_CCDD);
        assert_eq!(loc.x(), 0xCCDD);
        assert_eq!(loc.y(), 0xAABB);
        assert_eq!(loc.flags(), 1);
        assert_eq!(loc.to_u64(), 0x01_AABB_CCDD);

        let mut loc2 = NodeLocation::default();
        loc2.set_x(0x1234);
        loc2.set_y(0x5678);
        loc2.set_flags(0xFF);
        assert_eq!(loc2.to_u64(), 0xFF_5678_1234);
    }

    #[test]
    fn test_byteval_macro() {
        byteval! {
            pub struct PackedID(3);
        }
        let id = PackedID::from_u32(0xABCDEF);
        assert_eq!(id.to_u32(), 0xABCDEF);
        assert_eq!(id.0, [0xEF, 0xCD, 0xAB]);
        assert_eq!(id.value(), 0xABCDEF);

        let id2 = PackedID::default().with_value(0x112233);
        assert_eq!(id2.value(), 0x112233);

        let mut id3 = PackedID::default();
        id3.set_value(0x556677);
        assert_eq!(id3.value(), 0x556677);
        assert_eq!(id3.to_bits(), id3.to_u32());
    }

    #[test]
    fn test_bytestruct_sizes_and_primitives() {
        bytestruct! { struct B2(2) { pub v: u16 = 16 } }
        assert_eq!(B2::from_u16(0x1234).to_u16(), 0x1234);

        bytestruct! { struct B3(3) { pub v: u32 = 24 } }
        assert_eq!(B3::from_u32(0x112233).to_u32(), 0x112233);

        bytestruct! { struct B4(4) { pub v: u32 = 32 } }
        assert_eq!(B4::from_u32(0x11223344).to_u32(), 0x11223344);

        bytestruct! { struct B5(5) { pub v: u64 = 40 } }
        assert_eq!(B5::from_u64(0x1122334455).to_u64(), 0x1122334455);

        bytestruct! { struct B6(6) { pub v: u64 = 48 } }
        assert_eq!(B6::from_u64(0x112233445566).to_u64(), 0x112233445566);

        bytestruct! { struct B7(7) { pub v: u64 = 56 } }
        assert_eq!(B7::from_u64(0x11223344556677).to_u64(), 0x11223344556677);

        bytestruct! { struct B8(8) { pub v: u64 = 64 } }
        assert_eq!(
            B8::from_u64(0x1122334455667788).to_u64(),
            0x1122334455667788
        );

        bytestruct! { struct B9(9) { pub v: u128 = 72 } }
        assert_eq!(
            B9::from_u128(0x112233445566778899).to_u128(),
            0x112233445566778899
        );

        bytestruct! { struct B10(10) { pub v: u128 = 80 } }
        assert_eq!(
            B10::from_u128(0x11223344556677889900).to_u128(),
            0x11223344556677889900
        );

        bytestruct! { struct B11(11) { pub v: u128 = 88 } }
        assert_eq!(
            B11::from_u128(0x11223344556677889900AA).to_u128(),
            0x11223344556677889900AA
        );

        bytestruct! { struct B12(12) { pub v: u128 = 96 } }
        assert_eq!(
            B12::from_u128(0x11223344556677889900AABB).to_u128(),
            0x11223344556677889900AABB
        );

        bytestruct! { struct B13(13) { pub v: u128 = 104 } }
        assert_eq!(
            B13::from_u128(0x11223344556677889900AABBCC).to_u128(),
            0x11223344556677889900AABBCC
        );

        bytestruct! { struct B14(14) { pub v: u128 = 112 } }
        assert_eq!(
            B14::from_u128(0x11223344556677889900AABBCCDD).to_u128(),
            0x11223344556677889900AABBCCDD
        );

        bytestruct! { struct B15(15) { pub v: u128 = 120 } }
        assert_eq!(
            B15::from_u128(0x11223344556677889900AABBCCDDEE).to_u128(),
            0x11223344556677889900AABBCCDDEE
        );

        bytestruct! { struct B16(16) { pub v: u128 = 128 } }
        assert_eq!(
            B16::from_u128(0x11223344556677889900AABBCCDDEEFF).to_u128(),
            0x11223344556677889900AABBCCDDEEFF
        );
    }

    #[test]
    fn test_bitstruct_u128() {
        bitstruct! {
            pub struct WideId(u128) {
                pub id: u128 = 128
            }
        }
        let val = 0x11223344556677889900AABBCCDDEEFF;
        let s = WideId::from_bits(val);
        assert_eq!(s.id(), val);
        assert_eq!(s.with_id(0).to_bits(), 0);
    }

    #[test]
    fn test_bytestruct_mixed_fields() {
        bytestruct! {
            pub struct MixedArray(8) {
                pub flag: bool = 1,
                pub small: u8 = 8,
                pub medium: u16 = 16,
                pub state: EngineState = 2,
                pub large: u32 = 24, // Remaining 3 bytes
            }
        }

        let mut m = MixedArray::default();
        m.set_flag(true);
        m.set_small(0xAA);
        m.set_medium(0x1234);
        m.set_state(EngineState::ACTIVE);
        m.set_large(0xABCDEF);

        assert!(m.flag());
        assert_eq!(m.small(), 0xAA);
        assert_eq!(m.medium(), 0x1234);
        assert_eq!(m.state(), EngineState::ACTIVE);
        assert_eq!(m.large(), 0xABCDEF);

        // with_ builders
        let m2 = m
            .with_flag(false)
            .with_small(0)
            .with_medium(0)
            .with_state(EngineState::OFF)
            .with_large(0);
        assert!(!m2.flag());
        assert_eq!(m2.large(), 0);

        // set_ mutators
        let mut m3 = MixedArray::default();
        m3.set_flag(true);
        m3.set_small(1);
        m3.set_medium(2);
        m3.set_state(EngineState::IDLE);
        m3.set_large(3);
        assert!(m3.flag());
        assert_eq!(m3.large(), 3);

        // Functional updates (with_)
        let m2 = m
            .with_flag(false)
            .with_small(0xB)
            .with_medium(0x4321)
            .with_state(EngineState::OFF)
            .with_large(0x112233);
        assert!(!m2.flag());
        assert_eq!(m2.small(), 0xB);
        assert_eq!(m2.medium(), 0x4321);
        assert_eq!(m2.state(), EngineState::OFF);
        assert_eq!(m2.large(), 0x112233);

        // Alias verification
        assert_eq!(m.to_bits(), m.to_u64());
        assert_eq!(MixedArray::from_bits(m.to_bits()), m);

        // Verify dense packing: flag(1bit) small(8bit) medium(16bit) state(2bit) large(24bit)
        // bit shifts: flag(0) small(1) medium(9) state(25) large(27)
        assert_eq!(m.to_u64(), 1511207800695125);
    }

    #[test]
    fn test_bytestruct_le_ordering() {
        bytestruct! {
            struct LECheck(4) {
                pub b0: u8 = 8,
                pub b1: u8 = 8,
                pub b2: u8 = 8,
                pub b3: u8 = 8,
            }
        }
        let val = LECheck::from_u32(0x11223344);
        // Little Endian: 0x44 is least significant byte, stored at index 0
        assert_eq!(val.0, [0x44, 0x33, 0x22, 0x11]);
        assert_eq!(val.b0(), 0x44);
        assert_eq!(val.b3(), 0x11);
    }

    #[test]
    fn test_bytestruct_16byte_mixed() {
        bytestruct! {
            pub struct WideMixed(16) {
                pub flag_start: bool = 1,
                pub u8_field: u8 = 8,
                pub u16_field: u16 = 16,
                pub enum_field: EngineState = 2,
                pub u32_field: u32 = 32,
                pub u64_field: u64 = 64,
            }
        }

        let mut w = WideMixed::default();
        w.set_flag_start(true);
        w.set_u8_field(0xDE);
        w.set_u16_field(0xADBE);
        w.set_enum_field(EngineState::ACTIVE);
        w.set_u32_field(0xCAFEBABE);
        w.set_u64_field(0x22334455667788);

        assert!(w.flag_start());
        assert_eq!(w.u8_field(), 0xDE);
        assert_eq!(w.u16_field(), 0xADBE);
        assert_eq!(w.enum_field(), EngineState::ACTIVE);
        assert_eq!(w.u32_field(), 0xCAFEBABE);
        assert_eq!(w.u64_field(), 0x22334455667788);

        let w2 = w.with_flag_start(false).with_u64_field(0);
        assert!(!w2.flag_start());
        assert_eq!(w2.u64_field(), 0);
        assert_eq!(w2.u32_field(), 0xCAFEBABE);

        assert_eq!(WideMixed::from_bits(w.to_bits()), w);
    }

    #[test]
    fn test_bitenum_all_bases() {
        bitenum! { enum E16(16) { A = 0x1234 } }
        bitenum! { enum E32(32) { A = 0x12345678 } }
        bitenum! { enum E64(64) { A = 0x1234567890ABCDEF } }
        bitenum! { enum E128(128) { A = 0x11223344556677889900AABBCCDDEEFF } }

        assert_eq!(E16::A.to_bits(), 0x1234);
        assert_eq!(E32::A.to_bits(), 0x12345678);
        assert_eq!(E64::A.to_bits(), 0x1234567890ABCDEF);
        assert_eq!(E128::A.to_bits(), 0x11223344556677889900AABBCCDDEEFF);
    }

    #[test]
    fn test_bitstruct_const_init() {
        bitstruct! {
            pub struct ConstInit(u32) {
                pub a: u16 = 16,
                pub b: u16 = 16,
            }
        }
        const INIT: ConstInit = ConstInit::from_bits(0xCAFEBABE);
        const ZERO: ConstInit = ConstInit::from_bits(0);
        const WITH_A: ConstInit = ConstInit::from_bits(0).with_a(0x1234);

        assert_eq!(INIT.a(), 0xBABE);
        assert_eq!(INIT.b(), 0xCAFE);
        assert_eq!(ZERO.to_bits(), 0);
        assert_eq!(WITH_A.a(), 0x1234);
    }

    #[test]
    fn test_bytestruct_all_routes() {
        // Test 4, 6, 7, 8, 12, 16 bytes specifically to hit different acting primitives and conversions
        bytestruct! { struct B4(4) { pub v: u32 = 32 } }
        assert_eq!(B4::from_u32(0x11223344).to_u32(), 0x11223344);

        bytestruct! { struct B6(6) { pub v: u64 = 48 } }
        assert_eq!(B6::from_u64(0x112233445566).to_u64(), 0x112233445566);

        bytestruct! { struct B7(7) { pub v: u64 = 56 } }
        assert_eq!(B7::from_u64(0x11223344556677).to_u64(), 0x11223344556677);

        bytestruct! { struct B8(8) { pub v: u64 = 64 } }
        assert_eq!(
            B8::from_u64(0x1122334455667788).to_u64(),
            0x1122334455667788
        );

        bytestruct! { struct B12(12) { pub v: u128 = 96 } }
        assert_eq!(
            B12::from_u128(0x11223344556677889900AABB).to_u128(),
            0x11223344556677889900AABB
        );

        bytestruct! { struct B16(16) { pub v: u128 = 128 } }
        assert_eq!(
            B16::from_u128(0x11223344556677889900AABBCCDDEEFF).to_u128(),
            0x11223344556677889900AABBCCDDEEFF
        );
    }

    #[test]
    fn test_byteval_variations() {
        byteval! { struct V4(4); }
        assert_eq!(V4::from_u32(0x12345678).to_u32(), 0x12345678);

        byteval! { struct V5(5); }
        assert_eq!(V5::from_u64(0x123456789A).to_u64(), 0x123456789A);

        byteval! { struct V8(8); }
        assert_eq!(
            V8::from_u64(0x1234567890ABCDEF).to_u64(),
            0x1234567890ABCDEF
        );

        byteval! { struct V12(12); }
        assert_eq!(
            V12::from_u128(0x1234567890ABCDEF11223344).to_u128(),
            0x1234567890ABCDEF11223344
        );

        byteval! { struct V16(16); }
        assert_eq!(
            V16::from_u128(0x1234567890ABCDEF1122334455667788).to_u128(),
            0x1234567890ABCDEF1122334455667788
        );
    }

    #[test]
    #[should_panic(expected = "Value 8 overflows allocated 3 bits")]
    fn test_bitstruct_overflow_panic() {
        bitstruct! {
            pub struct OverflowCheck(u8) {
                pub val: u8 = 3,
            }
        }
        let mut x = OverflowCheck::default();
        x.set_val(8); // 8 requires 4 bits (1000), should panic in debug mode
    }

    #[test]
    #[should_panic(expected = "Value 258 overflows allocated 8 bits")]
    fn test_bytestruct_overflow_panic() {
        bytestruct! {
            pub struct ByteOverflowCheck(2) {
                pub val: u16 = 8,
            }
        }
        let mut x = ByteOverflowCheck::default();
        x.set_val(258); // 258 requires 9 bits, but field is 8 bits (1 byte)
    }

    #[test]
    fn test_strict_bitstruct_overflow_error() {
        use crate::BitstructError;
        bitstruct! {
            pub struct StrictOverflowCheck(u8) {
                pub val: u8 = 3,
            }
        }
        let mut x = StrictOverflowCheck::default();

        // Valid set
        assert_eq!(x.try_set_val(7), Ok(()));
        assert_eq!(x.val(), 7);

        // Invalid set
        assert_eq!(
            x.try_set_val(8),
            Err(BitstructError::Overflow {
                value: 8,
                allocated_bits: 3
            })
        );
        assert_eq!(x.val(), 7); // Ensure value was not modified

        // Valid with
        let y = x.try_with_val(4).unwrap();
        assert_eq!(y.val(), 4);

        // Invalid with
        assert_eq!(
            x.try_with_val(8),
            Err(BitstructError::Overflow {
                value: 8,
                allocated_bits: 3
            })
        );
    }

    #[test]
    fn test_strict_bytestruct_overflow_error() {
        use crate::BitstructError;
        bytestruct! {
            pub struct StrictByteOverflowCheck(2) {
                pub val: u16 = 8,
            }
        }
        let mut x = StrictByteOverflowCheck::default();

        // Valid set
        assert_eq!(x.try_set_val(255), Ok(()));
        assert_eq!(x.val(), 255);

        // Invalid set
        assert_eq!(
            x.try_set_val(256),
            Err(BitstructError::Overflow {
                value: 256,
                allocated_bits: 8
            })
        );
        assert_eq!(x.val(), 255); // Ensure value was not modified

        // Valid with
        let y = x.try_with_val(10).unwrap();
        assert_eq!(y.val(), 10);

        // Invalid with
        assert_eq!(
            x.try_with_val(256),
            Err(BitstructError::Overflow {
                value: 256,
                allocated_bits: 8
            })
        );
    }

    #[test]
    fn test_bitenum_try_from_bits() {
        use crate::BitstructError;
        bitenum! { enum TryOversized(3) { A = 0, B = 7 } } // Max value is 7

        // Valid creation
        let a = TryOversized::try_from_bits(7);
        assert!(a.is_ok());
        assert_eq!(a.unwrap().to_bits(), 7);

        // Invalid creation checking Result instead of Panic
        let b = TryOversized::try_from_bits(8);
        assert!(b.is_err());
        assert_eq!(
            b.unwrap_err(),
            BitstructError::InvalidVariant {
                value: 8,
                enum_name: "TryOversized"
            }
        );
    }

    #[test]
    fn test_bytemuck_integration() {
        bitstruct! {
            struct BitemuckTest(u32) {
                val: u32 = 32,
            }
        }

        bytestruct! {
            struct BitemuckByteTest(4) {
                val: u32 = 32,
            }
        }

        bitenum! {
            enum BitemuckEnum(2) {
                A = 0,
                B = 1,
            }
        }

        // Test Pod/Zeroable casting
        let raw = [0u8; 4];
        let struct_val: BitemuckTest = bytemuck::cast(raw);
        assert_eq!(struct_val.0, 0);

        let byte_struct_val: BitemuckByteTest = bytemuck::cast(raw);
        assert_eq!(byte_struct_val.0, [0u8; 4]);

        let enum_val: BitemuckEnum = bytemuck::cast(0u8);
        assert_eq!(enum_val.to_bits(), 0);
    }
}

/// --- NEGATIVE COMPILATION TESTS ---
/// These remain in the codebase as `compile_fail` doc-tests to verify and document
/// that invalid configurations (like overflowing boundaries or negative widths) are caught at compile-time.
///
/// ### Enforcing Enum Field Bit Bounds
/// Enumerations used as fields cannot request fewer bits than the enum inherently requires.
/// ```compile_fail
/// use bitcraft::{bitstruct, bitenum};
/// bitenum! { enum Oversized(3) { A = 0, B = 7 } } // Enum is 3 bits
/// bitstruct! {
///     struct SmallField(u8) { val: Oversized = 2 } // Field is 2 bits
/// }
/// ```
///
/// ### Enforcing Signed Base Type Bit Limits
/// Signed base types enforce that the sign bit is reserved, meaning an `i32` base can only store up to 31 bits.
/// ```compile_fail
/// use bitcraft::bitstruct;
/// bitstruct! {
///     struct SignedBase(i32) { val: u32 = 32 }
/// }
/// ```
///
/// ### Disallowing Signed Enum Values
/// ```compile_fail
/// use bitcraft::bitenum;
/// bitenum! {
///     enum SignedValue(8) { NEG = -1 }
/// }
/// ```
///
/// ### Disallowing Negative Bit Widths
/// ```compile_fail
/// use bitcraft::bitstruct;
/// bitstruct! {
///     struct NegativeWidth(u32) { val: u8 = -1 }
/// }
/// ```
mod _negative_tests_doc {}
