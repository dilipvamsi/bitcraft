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
//! - **[`bytestruct!`]**: Define a byte-aligned struct over a fixed-size array (`[u8; N]`).
//! - **[`byteval!`]**: A shorthand for "NewType" byte-array wrappers (e.g., 24-bit IDs).
//! - **[`bitenum!`]**: Define strongly-typed, zero-cost enumerations for use within bitfields.
//!
//! ## Key Features
//!
//! - **Mechanical Sympathy**: Designed to align with physical memory layouts and CPU cache lines.
//! - **Zero-Cost**: All methods are `const fn` and compile down to the exact bitwise operations you'd write by hand.
//! - **LSB-First Ordering**: Fields are packed from the **Least Significant Bit (LSB)** upward.
//! - **No Manual Derives**: All generated types automatically implement `Default` (zero-initialization).
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

impl std::error::Error for BitstructError {}

/// **Internal Trait**: Used to enforce that only unsigned integers are used as base types.
#[doc(hidden)]
pub trait IsUnsignedInt {
    /// A constant that confirms this type is valid.
    const ASSERT_UNSIGNED: ();
}

impl IsUnsignedInt for u8 {
    const ASSERT_UNSIGNED: () = ();
}
impl IsUnsignedInt for u16 {
    const ASSERT_UNSIGNED: () = ();
}
impl IsUnsignedInt for u32 {
    const ASSERT_UNSIGNED: () = ();
}
impl IsUnsignedInt for u64 {
    const ASSERT_UNSIGNED: () = ();
}
impl IsUnsignedInt for u128 {
    const ASSERT_UNSIGNED: () = ();
}

/// **Internal Trait**: Used to retrieve the bit-width of a primitive type or array.
#[doc(hidden)]
pub trait BitLength {
    /// The number of bits in the type.
    const BITS: usize;
}

impl BitLength for u8 {
    const BITS: usize = 8;
}
impl BitLength for u16 {
    const BITS: usize = 16;
}
impl BitLength for u32 {
    const BITS: usize = 32;
}
impl BitLength for u64 {
    const BITS: usize = 64;
}
impl BitLength for u128 {
    const BITS: usize = 128;
}
impl<const N: usize> BitLength for [u8; N] {
    const BITS: usize = N << 3;
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
pub const fn read_le_bits<const SHIFT: usize, const BITS: usize, const N: usize>(
    arr: &[u8; N],
) -> u128 {
    let start = SHIFT >> 3;
    let off = SHIFT & 7;
    let len = ((SHIFT + BITS + 7) >> 3) - start;

    // Fast path: Byte-aligned reads that fit in primitives.
    if off == 0 {
        if BITS == 8 {
            return arr[start] as u128;
        }
        if BITS == 16 && start + 2 <= N {
            return u16::from_le_bytes([arr[start], arr[start + 1]]) as u128;
        }
        if BITS == 32 && start + 4 <= N {
            return u32::from_le_bytes([arr[start], arr[start + 1], arr[start + 2], arr[start + 3]])
                as u128;
        }
        if BITS == 64 && start + 8 <= N {
            return u64::from_le_bytes([
                arr[start],
                arr[start + 1],
                arr[start + 2],
                arr[start + 3],
                arr[start + 4],
                arr[start + 5],
                arr[start + 6],
                arr[start + 7],
            ]) as u128;
        }
        if BITS == 128 && start + 16 <= N {
            return u128::from_le_bytes([
                arr[start],
                arr[start + 1],
                arr[start + 2],
                arr[start + 3],
                arr[start + 4],
                arr[start + 5],
                arr[start + 6],
                arr[start + 7],
                arr[start + 8],
                arr[start + 9],
                arr[start + 10],
                arr[start + 11],
                arr[start + 12],
                arr[start + 13],
                arr[start + 14],
                arr[start + 15],
            ]);
        }
    }

    if len <= 8 {
        let mut v = 0u64;
        let mut i = 0;
        while i < len {
            v |= (arr[start + i] as u64) << (i << 3);
            i += 1;
        }
        let mask = if BITS == 64 {
            !0u64
        } else {
            !0u64 >> (64 - BITS)
        };
        ((v >> off) & mask) as u128
    } else {
        let mut v = 0u128;
        let mut i = 0;
        while i < len {
            v |= (arr[start + i] as u128) << (i << 3);
            i += 1;
        }
        let mask = if BITS == 128 {
            !0u128
        } else {
            !0u128 >> (128 - BITS)
        };
        (v >> off) & mask
    }
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
pub const fn write_le_bits<const SHIFT: usize, const BITS: usize, const N: usize>(
    arr: &mut [u8; N],
    val: u128,
) {
    let start = SHIFT >> 3;
    let off = SHIFT & 7;
    let len = ((SHIFT + BITS + 7) >> 3) - start;

    // Fast path: Byte-aligned writes that fit in primitives.
    if off == 0 {
        if BITS == 8 {
            arr[start] = val as u8;
            return;
        }
        if BITS == 16 && start + 2 <= N {
            let bytes = (val as u16).to_le_bytes();
            arr[start] = bytes[0];
            arr[start + 1] = bytes[1];
            return;
        }
        if BITS == 32 && start + 4 <= N {
            let bytes = (val as u32).to_le_bytes();
            arr[start] = bytes[0];
            arr[start + 1] = bytes[1];
            arr[start + 2] = bytes[2];
            arr[start + 3] = bytes[3];
            return;
        }
        if BITS == 64 && start + 8 <= N {
            let bytes = (val as u64).to_le_bytes();
            let mut i = 0;
            while i < 8 {
                arr[start + i] = bytes[i];
                i += 1;
            }
            return;
        }
        if BITS == 128 && start + 16 <= N {
            let bytes = val.to_le_bytes();
            let mut i = 0;
            while i < 16 {
                arr[start + i] = bytes[i];
                i += 1;
            }
            return;
        }
    }

    if len <= 8 {
        let mut v = 0u64;
        let mut i = 0;
        while i < len {
            v |= (arr[start + i] as u64) << (i << 3);
            i += 1;
        }
        let mask = if BITS == 64 {
            !0u64
        } else {
            !0u64 >> (64 - BITS)
        };
        let shifted_mask = mask << off;
        v = (v & !shifted_mask) | ((val as u64) << off);
        let mut i = 0;
        while i < len {
            arr[start + i] = (v >> (i << 3)) as u8;
            i += 1;
        }
    } else {
        let mut v = 0u128;
        let mut i = 0;
        while i < len {
            v |= (arr[start + i] as u128) << (i << 3);
            i += 1;
        }
        let mask = if BITS == 128 {
            !0u128
        } else {
            !0u128 >> (128 - BITS)
        };
        let shifted_mask = mask << off;
        v = (v & !shifted_mask) | (val << off);
        let mut i = 0;
        while i < len {
            arr[start + i] = (v >> (i << 3)) as u8;
            i += 1;
        }
    }
}

/// A declarative macro for generating zero-cost, strictly packed bitfields.
///
/// This macro generates a `#[repr(transparent)]` struct wrapping a base integer type
/// (e.g. `u8`, `u16`, `u32`, `u64`). It automatically generates `const fn` getters,
/// mutable setters (`set_xyz`), and immutable builder methods (`with_xyz`) for each field.
///
/// # Example
///
/// ```rust
/// use bitcraft::bitstruct;
///
/// bitstruct! {
///     /// A configuration bitmask.
///     pub struct Config(u8) {
///         pub enabled: bool = 1,
///         pub mode: u8 = 3,
///         pub padding: u8 = 4,
///     }
/// }
///
/// let mut c = Config::default().with_enabled(true).with_mode(5);
/// assert!(c.enabled());
/// assert_eq!(c.mode(), 5);
/// assert_eq!(c.to_bits(), (5 << 1) | 1); // mode starts at bit 1, enabled at bit 0
/// ```
#[macro_export]
macro_rules! bitstruct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident ($base_type:ty) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Default)]
        #[derive($crate::bytemuck::Pod, $crate::bytemuck::Zeroable)]
        #[repr(transparent)]
        $vis struct $struct_name(pub $base_type);

        impl core::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("raw", &self.0)
                    $(
                        .field(stringify!($field_name), &self.$field_name())
                    )*
                    .finish()
            }
        }

        const _: () = {
            // Compile-time check: Ensure the base storage is an unsigned primitive.
            let _ = <$base_type as $crate::IsUnsignedInt>::ASSERT_UNSIGNED;

            // Compile-time check: Ensure each field type is valid.
            $crate::bitstruct!(@check_fields $($field_type)*);

            let total_bits = 0 $( + $bits )*;
            assert!(total_bits <= <$base_type as $crate::BitLength>::BITS, "Sum of field bits exceeds base type size");
        };

        impl $struct_name {
            // Start the TT (Token-Tree) muncher recursion to generate field-specific methods.
            $crate::bitstruct!(@impl_getters_setters $base_type, 0, $($field_vis $field_name $field_type $bits)*);

            /// Returns the raw interior integer value.
            ///
            /// This is useful for serializing the struct or passing it to external APIs.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn to_bits(self) -> $base_type { self.0 }

            /// Creates a new instance from a raw integer value.
            ///
            /// # Safety
            /// While this method is safe, providing values with bits set outside
            /// the defined field ranges may result in those bits being preserved
            /// (padded) or ignored depending on the getters used.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn from_bits(val: $base_type) -> Self { Self(val) }
        }
    };

    // --- INTERNAL MACRO IMPLEMENTATION ---
    // The following branches use the "Token-Tree (TT) Muncher" pattern.
    // They recursively consume field definitions, incrementing the bit `$shift` offset with each pass.

    // Base Case: No more fields left to process, recursion terminates.
    (@impl_getters_setters $base_type:ty, $shift:expr, ) => {};

    // TT Muncher: Check each field type is valid
    (@check_fields ) => {};
    (@check_fields $field_type:tt $($rest:tt)*) => {
        let _ = <$field_type as $crate::ValidField>::ASSERT_VALID;
        $crate::bitstruct!(@check_fields $($rest)*);
    };

    // TT Muncher: Specialized traversal arm for `bool` type fields.
    // Bools are handled specifically to return Rust native `true`/`false` rather than `1`/`0`.
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident bool $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            const [<$field_name:upper _OFFSET>]: usize = $shift;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = (!0 as $base_type) >> (<$base_type as $crate::BitLength>::BITS - $bits);

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Returns the boolean value mapping to the `", stringify!($field_name), "` flag.")]
            $field_vis const fn $field_name(self) -> bool {
                ((self.0 >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) != 0
            }

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Inline mutation to set the `", stringify!($field_name), "` flag.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: bool) {
                // bool inherently cannot overflow its bit requirement.
                let val_masked = val as $base_type;
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bitfield with the `", stringify!($field_name), "` flag specified.")]
            $field_vis const fn [<with_ $field_name>](self, val: bool) -> Self {
                let val_masked = val as $base_type;
                // Clear the target bits using the inverted mask, then OR with the new value.
                Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
            }

            #[allow(dead_code)]
            #[doc = concat!("Inline mutation to set the `", stringify!($field_name), "` flag. Returns `Ok(())` since booleans cannot overflow.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: bool) -> Result<(), $crate::BitstructError> {
                self.[<set_ $field_name>](val);
                Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bitfield with the `", stringify!($field_name), "` flag specified. Returns `Ok(Self)` since booleans cannot overflow.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: bool) -> Result<Self, $crate::BitstructError> {
                Ok(self.[<with_ $field_name>](val))
            }
        }

        $crate::bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };

    // Specialized routing for primitives (to allow them to work without trait impls)
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u8 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u8 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u16 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u16 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u32 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u32 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u64 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u64 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u128 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u128 $bits $($rest)*); };

    // Standard integer implementation: Extracts the requested bit width, shifting by the accumulated offset.
    (@impl_int $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            const [<$field_name:upper _OFFSET>]: usize = $shift;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = (!0 as $base_type) >> (<$base_type as $crate::BitLength>::BITS - $bits);

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Returns the `", stringify!($field_name), "` property as a `", stringify!($field_type), "`.")]
            $field_vis const fn $field_name(self) -> $field_type {
                ((self.0 >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as $field_type
            }

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Inline mutation to apply the `", stringify!($field_name), "` property. Masks inputs over ", stringify!($bits), " bits.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: $field_type) {
                debug_assert!((val as $base_type) <= Self::[<$field_name:upper _MASK>], "Value {} overflows allocated {} bits", val, $bits);
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bitfield with the `", stringify!($field_name), "` property mapped. Masks inputs over ", stringify!($bits), " bits.")]
            $field_vis const fn [<with_ $field_name>](self, val: $field_type) -> Self {
                debug_assert!((val as $base_type) <= Self::[<$field_name:upper _MASK>], "Value overflows allocated bits");
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict inline mutation to apply the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                if (val as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: (val as $base_type) as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
                Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict cloned evaluation to apply the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                if (val as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: (val as $base_type) as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                Ok(Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>])))
            }
        }

        $crate::bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };


    // TT Muncher Fallback: Handles strictly-typed BitEnums (Types not caught by bool/u8/u16/u32/u64).
    // Uses `from_bits` to upcast/downcast the extracted integer bits back into the Enum variant safely.
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            const [<$field_name:upper _OFFSET>]: usize = $shift;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = (!0 as $base_type) >> (<$base_type as $crate::BitLength>::BITS - $bits);

            #[allow(dead_code)]
            #[doc = concat!("Returns the `", stringify!($field_name), "` variant strictly typed to the `", stringify!($field_type), "` enumeration.")]
            $field_vis const fn $field_name(self) -> $field_type {
                // Extract the bits, then cast them into the Enum's base primitive via `_` inference.
                // This relies on the bitenum! macro providing a `from_bits` method.
                $field_type::from_bits(((self.0 >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as _)
            }

            #[allow(dead_code)]
            #[doc = concat!("Inline mutation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: $field_type) {
                debug_assert!((val.to_bits() as $base_type) <= Self::[<$field_name:upper _MASK>], "Enum variant overflows allocated {} bits", $bits);
                // Cast the enum's inner value up to the bitfield's base storage type before shifting
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bitfield bounded by the `", stringify!($field_type), "` enumeration supplied to `", stringify!($field_name), "`.")]
            $field_vis const fn [<with_ $field_name>](self, val: $field_type) -> Self {
                debug_assert!((val.to_bits() as $base_type) <= Self::[<$field_name:upper _MASK>], "Enum variant overflows allocated bits");
                // Cast the enum's inner value up to the bitfield's base storage type before shifting
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict inline mutation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                if (val.to_bits() as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: val.to_bits() as u128, allocated_bits: $bits });
                }
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
                Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict cloned evaluation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                if (val.to_bits() as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: val.to_bits() as u128, allocated_bits: $bits });
                }
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                Ok(Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>])))
            }
        }

        $crate::bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };
}

/// A unique declarative macro for generating bitfields backed by fixed-size byte arrays.
///
/// Unlike standard bitfield libraries that restrict storage to primitives (u8-u128),
/// `bytestruct` allows arbitrary array-backed storage (`[u8; N]`) while maintaining
/// register-wide optimization through "Acting Primitives".
///
/// This macro generates a struct wrapping `[u8; N]`. It uses an internal "acting primitive"
/// (u32, u64, or u128) based on $N$ to perform efficient bitwise operations.
///
/// # Example
///
/// ```rust
/// use bitcraft::bytestruct;
///
/// bytestruct! {
///     pub struct Example(2) {
///         pub a: u8 = 4,   // 4 bits
///         pub b: bool = 1, // 1 bit
///         pub c: u16 = 11, // 11 bits
///     }
/// }
/// ```
#[macro_export]
macro_rules! bytestruct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident ($N:tt) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Default)]
        #[derive($crate::bytemuck::Pod, $crate::bytemuck::Zeroable)]
        #[repr(transparent)]
        $vis struct $name(pub [u8; $N]);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("raw", &self.0)
                    $(
                        .field(stringify!($field_name), &self.$field_name())
                    )*
                    .finish()
            }
        }

        const _: () = {
            assert!($N >= 1 && $N <= 16, "bytestruct! requires 1-16 bytes");
            let bit_sum = 0 $( + $bits )*;
            assert!(bit_sum <= $N << 3, "Sum of field bits exceeds array size");

            // Compile-time check: Ensure each field type is valid.
            $crate::bitstruct!(@check_fields $($field_type)*);
        };

        impl $name {
            /// Returns the underlying raw byte array.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn to_array(self) -> [u8; $N] { self.0 }

            /// Creates a new instance from a raw byte array.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn from_array(arr: [u8; $N]) -> Self { Self(arr) }

            // Implementation of to_uXX/from_uXX and to_bits/from_bits.
            $crate::bytestruct!(@impl_conversions $name, $N);

            // Route fields based on the total array size and choose the appropriate acting primitive.
            $crate::bytestruct!(@route_fields $name, $N, $($field_vis $field_name $field_type $bits)*);
        }
    };

    // --- INTERNAL CONVERSIONS ---
    (@impl_conversions $name:ident, 1) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_u8(self) -> u8 { self.0[0] }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_u8(val: u8) -> Self { Self([val]) }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_bits(self) -> u8 { self.to_u8() }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_bits(val: u8) -> Self { Self::from_u8(val) }
    };
    (@impl_conversions $name:ident, 2) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_u16(self) -> u16 { u16::from_le_bytes(self.0) }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_u16(val: u16) -> Self { Self(val.to_le_bytes()) }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_bits(self) -> u16 { self.to_u16() }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_bits(val: u16) -> Self { Self::from_u16(val) }
    };
    (@impl_conversions $name:ident, 3) => { $crate::bytestruct!(@impl_wide_conv $name, 3, u32, to_u32, from_u32); };
    (@impl_conversions $name:ident, 4) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_u32(self) -> u32 { u32::from_le_bytes(self.0) }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_u32(val: u32) -> Self { Self(val.to_le_bytes()) }
        /// Returns the raw interior value as bits.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_bits(self) -> u32 { self.to_u32() }
        /// Creates a bytestruct from raw bits.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_bits(val: u32) -> Self { Self::from_u32(val) }
    };
    (@impl_conversions $name:ident, 5) => { $crate::bytestruct!(@impl_wide_conv $name, 5, u64, to_u64, from_u64); };
    (@impl_conversions $name:ident, 6) => { $crate::bytestruct!(@impl_wide_conv $name, 6, u64, to_u64, from_u64); };
    (@impl_conversions $name:ident, 7) => { $crate::bytestruct!(@impl_wide_conv $name, 7, u64, to_u64, from_u64); };
    (@impl_conversions $name:ident, 8) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_u64(self) -> u64 { u64::from_le_bytes(self.0) }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_u64(val: u64) -> Self { Self(val.to_le_bytes()) }
        /// Returns the raw interior value as bits.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_bits(self) -> u64 { self.to_u64() }
        /// Creates a bytestruct from raw bits.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_bits(val: u64) -> Self { Self::from_u64(val) }
    };
    (@impl_conversions $name:ident, 9) => { $crate::bytestruct!(@impl_wide_conv $name, 9, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 10) => { $crate::bytestruct!(@impl_wide_conv $name, 10, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 11) => { $crate::bytestruct!(@impl_wide_conv $name, 11, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 12) => { $crate::bytestruct!(@impl_wide_conv $name, 12, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 13) => { $crate::bytestruct!(@impl_wide_conv $name, 13, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 14) => { $crate::bytestruct!(@impl_wide_conv $name, 14, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 15) => { $crate::bytestruct!(@impl_wide_conv $name, 15, u128, to_u128, from_u128); };
    (@impl_conversions $name:ident, 16) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_u128(self) -> u128 { u128::from_le_bytes(self.0) }
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_u128(val: u128) -> Self { Self(val.to_le_bytes()) }
        /// Returns the raw interior value as bits.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_bits(self) -> u128 { self.to_u128() }
        /// Creates a bytestruct from raw bits.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_bits(val: u128) -> Self { Self::from_u128(val) }
    };

    // Implementation for multi-byte wide conversions that don't map directly to a primitive sized array.
    (@impl_wide_conv $name:ident, $N:tt, $prim:ty, $to_name:ident, $from_name:ident) => {
        // Extrapolates the array into a Little-Endian primitive.
        //
        // This is useful for bitwise manipulation or passing to external APIs.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn $to_name(self) -> $prim {
            let mut val = 0 as $prim;
            let mut i = 0;
            // Constant-length while loop that LLVM can easily unroll for small $N$.
            while i < $N {
                val |= (self.0[i] as $prim) << (i << 3);
                i += 1;
            }
            val
        }
        // Creates a new instance from a Little-Endian primitive.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn $from_name(val: $prim) -> Self {
            let mut arr = [0u8; $N];
            let mut i = 0;
            while i < $N {
                arr[i] = (val >> (i << 3)) as u8;
                i += 1;
            }
            Self(arr)
        }
        // Alias for the primary integer conversion method for this struct.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn to_bits(self) -> $prim { self.$to_name() }
        // Alias for the primary integer conversion method for this struct.
        #[inline(always)]
        #[allow(dead_code)]
        pub const fn from_bits(val: $prim) -> Self { Self::$from_name(val) }
    };
    // Note: Converted values are handled as Little-Endian for consistent bitfield mapping.

    // Routing based on byte size $N$ to select the most efficient "acting primitive" for bitwise operations.
    (@route_fields $name:ident, 1, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u8,   0, $($rest)*); };
    (@route_fields $name:ident, 2, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u16,  0, $($rest)*); };
    (@route_fields $name:ident, 3, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u32,  0, $($rest)*); };
    (@route_fields $name:ident, 4, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u32,  0, $($rest)*); };
    (@route_fields $name:ident, 5, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u64,  0, $($rest)*); };
    (@route_fields $name:ident, 6, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u64,  0, $($rest)*); };
    (@route_fields $name:ident, 7, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u64,  0, $($rest)*); };
    (@route_fields $name:ident, 8, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u64,  0, $($rest)*); };
    (@route_fields $name:ident, 9, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 10, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 11, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 12, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 13, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 14, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 15, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };
    (@route_fields $name:ident, 16, $($rest:tt)*) => { $crate::bytestruct!(@impl_fields $name, u128, 0, $($rest)*); };

    (@impl_fields $name:ident, $prim:ty, $shift:expr, ) => {};

    // Boolean arm
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident bool $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            const [<$field_name:upper _OFFSET>]: usize = $shift;

            #[doc(hidden)]
            #[allow(dead_code)]
            const [<$field_name:upper _MASK>]: u128 = (!0u128) >> (128 - $bits);

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Returns the boolean value mapping to the `", stringify!($field_name), "` flag.")]
            $field_vis const fn $field_name(self) -> bool {
                let val = $crate::bytestruct!(@read_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits);
                val != 0
            }

            #[allow(dead_code)]
            #[doc = concat!("Inline mutation to set the `", stringify!($field_name), "` flag.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: bool) {
                $crate::bytestruct!(@write_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits, val as u128);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bytestruct with the `", stringify!($field_name), "` flag specified.")]
            $field_vis const fn [<with_ $field_name>](mut self, val: bool) -> Self {
                $crate::bytestruct!(@write_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits, val as u128);
                self
            }
        }
        $crate::bytestruct!(@impl_fields $name, $prim, $shift + $bits, $($rest)*);
    };

    // Standard integer arms (u8..u128)
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident u8 $bits:tt $($rest:tt)*) => { $crate::bytestruct!(@impl_int $name, $prim, $shift, $field_vis $field_name u8 $bits $($rest)*); };
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident u16 $bits:tt $($rest:tt)*) => { $crate::bytestruct!(@impl_int $name, $prim, $shift, $field_vis $field_name u16 $bits $($rest)*); };
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident u32 $bits:tt $($rest:tt)*) => { $crate::bytestruct!(@impl_int $name, $prim, $shift, $field_vis $field_name u32 $bits $($rest)*); };
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident u64 $bits:tt $($rest:tt)*) => { $crate::bytestruct!(@impl_int $name, $prim, $shift, $field_vis $field_name u64 $bits $($rest)*); };
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident u128 $bits:tt $($rest:tt)*) => { $crate::bytestruct!(@impl_int $name, $prim, $shift, $field_vis $field_name u128 $bits $($rest)*); };

    (@impl_int $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            const [<$field_name:upper _OFFSET>]: usize = $shift;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: u128 = (!0u128) >> (128 - $bits);

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Returns the `", stringify!($field_name), "` property as a `", stringify!($field_type), "`.")]
            $field_vis const fn $field_name(self) -> $field_type {
                let val = $crate::bytestruct!(@read_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits);
                val as $field_type
            }

            #[allow(dead_code)]
            #[doc = concat!("Inline mutation to apply the `", stringify!($field_name), "` property. Masks inputs over ", stringify!($bits), " bits.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: $field_type) {
                debug_assert!((val as u128) <= Self::[<$field_name:upper _MASK>], "Value {} overflows allocated {} bits", val, $bits);
                $crate::bytestruct!(@write_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits, val as u128);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bytestruct with the `", stringify!($field_name), "` property mapped. Masks inputs over ", stringify!($bits), " bits.")]
            $field_vis const fn [<with_ $field_name>](mut self, val: $field_type) -> Self {
                debug_assert!((val as u128) <= Self::[<$field_name:upper _MASK>], "Value overflows allocated bits");
                $crate::bytestruct!(@write_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits, val as u128);
                self
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict inline mutation to apply the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                if (val as u128) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: val as u128, allocated_bits: $bits });
                }
                self.[<set_ $field_name>](val);
                Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict cloned evaluation to apply the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                if (val as u128) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: val as u128, allocated_bits: $bits });
                }
                Ok(self.[<with_ $field_name>](val))
            }
        }
        $crate::bytestruct!(@impl_fields $name, $prim, $shift + $bits, $($rest)*);
    };


    // Enum arm
    (@impl_fields $name:ident, $prim:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            const [<$field_name:upper _OFFSET>]: usize = $shift;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: u128 = (!0u128) >> (128 - $bits);

            #[allow(dead_code)]
            #[doc = concat!("Returns the `", stringify!($field_name), "` variant strictly typed to the `", stringify!($field_type), "` enumeration.")]
            $field_vis const fn $field_name(self) -> $field_type {
                let val = $crate::bytestruct!(@read_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits);
                $field_type::from_bits(val as _)
            }

            #[allow(dead_code)]
            #[doc = concat!("Inline mutation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: $field_type) {
                let raw = val.to_bits() as u128;
                debug_assert!(raw <= Self::[<$field_name:upper _MASK>], "Enum variant overflows allocated bits");
                $crate::bytestruct!(@write_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits, raw);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bytestruct bounded by the `", stringify!($field_type), "` enumeration supplied to `", stringify!($field_name), "`.")]
            $field_vis const fn [<with_ $field_name>](mut self, val: $field_type) -> Self {
                debug_assert!((val.to_bits() as u128) <= Self::[<$field_name:upper _MASK>], "Enum variant overflows allocated bits");
                $crate::bytestruct!(@write_localized_prim self.0, Self::[<$field_name:upper _OFFSET>], $bits, val.to_bits() as u128);
                self
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict inline mutation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                 if (val.to_bits() as u128) > Self::[<$field_name:upper _MASK>] {
                     return Err($crate::BitstructError::Overflow { value: val.to_bits() as u128, allocated_bits: $bits });
                 }
                 self.[<set_ $field_name>](val);
                 Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict cloned evaluation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                 if (val.to_bits() as u128) > Self::[<$field_name:upper _MASK>] {
                     return Err($crate::BitstructError::Overflow { value: val.to_bits() as u128, allocated_bits: $bits });
                 }
                 Ok(self.[<with_ $field_name>](val))
            }
        }
        $crate::bytestruct!(@impl_fields $name, $prim, $shift + $bits, $($rest)*);
    };
    // --- LOCALIZED PRIMITIVE HELPERS ---
    //
    // These helpers implement the "Literal Guard" pattern.
    // Instead of a dynamic loop (which the compiler often fails to unroll for variable-width bitfields),
    // we use a sequence of literal index checks.
    //
    // Since $bits and $shift are constants provided during macro expansion,
    // LLVM can perfectly constant-fold these `if len > N` branches, leaving
    // only a flat sequence of bitwise operations.
    //
    // We specialize in 64-bit registers (u64) for all spans <= 8 bytes to reduce register pressure.

    (@read_localized_prim $arr:expr, $shift:expr, $bits:tt) => {
        $crate::read_le_bits::<{$shift}, {$bits}, _>(&$arr)
    };

    (@write_localized_prim $arr:expr, $shift:expr, $bits:tt, $val:expr) => {
        $crate::write_le_bits::<{$shift}, {$bits}, _>(&mut $arr, $val)
    };
}

/// A unique shorthand macro for creating "NewType" byte-array wrappers with a primary value field.
///
/// This is a specialized feature of `bitstruct` designed for high-density cases like
/// 24-bit (3-byte) or 40-bit (5-byte) IDs where you want a typed wrapper over a
/// byte array that behaves like a first-class integer.
///
/// This solves the "Odd-Width Integer" problem without requiring external crates or
/// manual 4-byte padding.
///
/// # Example
/// ```rust
/// use bitcraft::byteval;
/// byteval! { pub struct OrderId(3); } // 3-byte array field
/// let id = OrderId::from_u32(0x123456);
/// assert_eq!(id.value(), 0x123456);
/// ```
#[macro_export]
macro_rules! byteval {
    ($(#[$meta:meta])* $vis:vis struct $name:ident (1);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (1) { pub value: u8 = 8 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (2);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (2) { pub value: u16 = 16 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (3);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (3) { pub value: u32 = 24 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (4);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (4) { pub value: u32 = 32 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (5);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (5) { pub value: u64 = 40 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (6);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (6) { pub value: u64 = 48 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (7);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (7) { pub value: u64 = 56 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (8);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (8) { pub value: u64 = 64 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (9);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (9) { pub value: u128 = 72 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (10);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (10) { pub value: u128 = 80 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (11);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (11) { pub value: u128 = 88 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (12);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (12) { pub value: u128 = 96 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (13);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (13) { pub value: u128 = 104 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (14);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (14) { pub value: u128 = 112 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (15);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (15) { pub value: u128 = 120 } } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (16);) => { $crate::bytestruct! { $(#[$meta])* $vis struct $name (16) { pub value: u128 = 128 } } };
}
/// A hidden trait used to resolve the narrowest primitive type for a given bit-width.
#[doc(hidden)]
pub trait BitenumType {
    /// The narrowest CPU primitive capable of holding $Bits$ bits.
    type Prim: Copy + Clone + PartialEq + Eq + core::fmt::Debug + Default;
}
/// A hidden marker struct used for primitive type resolution.
#[doc(hidden)]
pub struct Bits<const N: usize>;

macro_rules! impl_bits {
    ($prim:ty, $($n:literal),+) => {
        $(
            impl BitenumType for Bits<$n> {
                type Prim = $prim;
            }
        )+
    };
}
impl_bits!(u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_bits!(u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_bits!(
    u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
impl_bits!(
    u64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
    55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);
impl_bits!(
    u128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
    87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
    108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
    127, 128
);

/// A declarative macro for generating zero-cost bitenums.
///
/// This generates a `#[repr(transparent)]` struct with associated constants, simulating an enum.
/// It takes the exact number of bits required and automatically resolves to the narrowest
/// standard CPU primitive (`u8` to `u128`) capable of holding those bits.
///
/// # Example
///
/// ```rust
/// use bitcraft::bitenum;
///
/// bitenum! {
///     /// Connection state tracking.
///     pub enum ConnectionState(2) {
///         DISCONNECTED = 0,
///         CONNECTING = 1,
///         CONNECTED = 2,
///     }
/// }
///
/// let state = ConnectionState::CONNECTED;
/// assert_eq!(state.to_bits(), 2);
/// ```
#[macro_export]
macro_rules! bitenum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident ($bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Default)]
        #[derive($crate::bytemuck::Pod, $crate::bytemuck::Zeroable)]
        #[repr(transparent)]
        $vis struct $enum_name(pub <$crate::Bits<$bits> as $crate::BitenumType>::Prim);

        impl core::fmt::Debug for $enum_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let s = match self.0 {
                    $(
                        $val => stringify!($variant),
                    )*
                    _ => "UNKNOWN",
                };
                write!(f, "{}({})::{}", stringify!($enum_name), self.0, s)
            }
        }

        impl $enum_name {
            $(
                #[allow(non_upper_case_globals, dead_code)]
                #[doc = concat!("Enumeration variant for `", stringify!($variant), "` with raw value `", stringify!($val), "`.")]
                pub const $variant: Self = Self($val);
            )*

            #[allow(dead_code)]
            /// The number of bits allocated for this enumeration in memory.
            pub const BITS: usize = $bits;

            /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
            ///
            /// Useful for manually validating raw input before conversion.
            pub const MASK: <$crate::Bits<$bits> as $crate::BitenumType>::Prim = {
                type Prim = <$crate::Bits<$bits> as $crate::BitenumType>::Prim;
                let total_bits = <Prim as $crate::BitLength>::BITS;
                (!0 as Prim) >> (total_bits - $bits)
            };

            /// Returns true if the raw value corresponds to a defined enumeration variant.
            ///
            /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
            #[inline(always)]
            pub const fn is_defined(self) -> bool {
                match self.0 {
                    $( $val => true, )*
                    _ => false,
                }
            }

            /// Returns the raw integer value of the enumeration variant.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn to_bits(self) -> <$crate::Bits<$bits> as $crate::BitenumType>::Prim { self.0 }

            /// Creates an enumeration variant from a raw integer value.
            ///
            /// # Panics
            /// In debug mode, this will panic if the value exceeds the allocated bit width.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn from_bits(val: <$crate::Bits<$bits> as $crate::BitenumType>::Prim) -> Self {
                debug_assert!(val <= Self::MASK, "Value overflows allocated bit width for this enumeration");
                Self(val)
            }

            /// Creates an enumeration variant from a raw integer value, returning an error if it is invalid.
            ///
            /// This returns `Ok(Self)` if the value corresponds to a defined variant,
            /// or `Err(BitstructError::InvalidVariant)` if it does not.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn try_from_bits(val: <$crate::Bits<$bits> as $crate::BitenumType>::Prim) -> Result<Self, $crate::BitstructError> {
                let s = Self(val);
                if s.is_defined() {
                    Ok(s)
                } else {
                    Err($crate::BitstructError::InvalidVariant { value: val as u128, enum_name: stringify!($enum_name) })
                }
            }
        }

        impl $crate::ValidField for $enum_name {
            const ASSERT_VALID: () = ();
        }

        const _: () = {
            type Prim = <$crate::Bits<$bits> as $crate::BitenumType>::Prim;
            $(
                assert!(
                    ($val as Prim) <= ((!0 as Prim) >> (<Prim as $crate::BitLength>::BITS - $bits)),
                    "Enum variant exceeds the maximum value for the allocated bit width"
                );
            )*
        };
    };
}

#[cfg(test)]
mod tests {

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
    #[should_panic(expected = "Enum variant overflows allocated 2 bits")]
    fn test_bitenum_overflow_panic() {
        bitenum! { enum Oversized(3) { A = 0, B = 7 } } // B requires 3 bits
        bitstruct! {
            pub struct EnumOverflowCheck(u8) {
                pub val: Oversized = 2, // Only allocates 2 bits
            }
        }
        let mut x = EnumOverflowCheck::default();
        x.set_val(Oversized::B); // 7 into 2 bits, should panic
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
/// that invalid configurations (like signed integers) are caught at compile-time.
///
/// ### Disallowing Signed Base Types
/// ```compile_fail
/// use bitcraft::bitstruct;
/// bitstruct! {
///     struct SignedBase(i32) { val: u32 = 32 }
/// }
/// ```
///
/// ### Disallowing Signed Field Types
/// ```compile_fail
/// use bitcraft::bitstruct;
/// bitstruct! {
///     struct SignedField(u32) { val: i32 = 32 }
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
