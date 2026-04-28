/// A declarative macro for generating zero-cost, strictly packed bitfields.
///
/// This macro generates a `#[repr(transparent)]` struct wrapping a base integer type
/// (e.g. `u8`, `u16`, `u32`, `u64`). It automatically generates `const fn` getters,
/// mutable setters (`set_xyz`), and immutable builder methods (`with_xyz`) for each field.
///
/// # Examples
///
/// ### Basic Unsigned & Enum Packing
/// ```rust
/// use bitcraft::{bitstruct, bitenum};
///
/// bitenum! {
///     pub enum EngineState(2) {
///         OFF = 0,
///         IDLE = 1,
///         ACTIVE = 2,
///     }
/// }
///
/// bitstruct! {
///     pub struct VehicleState(u16) {
///         pub is_running: bool = 1,         // Bit 0
///         pub gear: u8 = 3,                 // Bits 1-3
///         pub speed: u16 = 8,               // Bits 4-11
///         pub state: EngineState = 4,       // Bits 12-15
///     }
/// }
///
/// let mut state = VehicleState::default()
///     .with_is_running(true)
///     .with_gear(3)
///     .with_state(EngineState::ACTIVE);
///
/// assert!(state.is_running());
/// assert_eq!(state.gear(), 3);
/// assert_eq!(state.state(), EngineState::ACTIVE);
/// ```
///
/// ### Advanced: Signed Fields and Strict Boundaries
/// `bitstruct!` natively supports Two's Complement signed integers. It automatically handles
/// correct sign-extension on reads and limits-checking on writes.
/// ```rust
/// use bitcraft::{bitstruct, BitstructError};
///
/// bitstruct! {
///     pub struct SensorData(u32) {
///         pub id: u8 = 4,
///         pub temperature: i16 = 12, // 12-bit signed integer (-2048 to 2047)
///         pub humidity: u16 = 10,
///     }
/// }
///
/// let mut data = SensorData::default();
/// data.set_temperature(-500); // Perfectly valid, natively handled
/// assert_eq!(data.temperature(), -500);
///
/// // Strict validation utilizing `try_set_`
/// let res = data.try_set_temperature(3000);
/// assert!(matches!(res, Err(BitstructError::Overflow { .. })));
/// ```
///
/// # Implementation Details
///
/// The `bitstruct!` macro utilizes a functional "Token-Tree Muncher" to incrementally track
/// bit-offsets at compile-time (`$shift`). For each field, it generates a perfect bitmask
/// shifted appropriately. Because all masks and shifts are strictly resolved at compile time,
/// the resulting getters and setters are perfectly optimizable by LLVM into zero-cost single instructions.
///
/// The macro supports signed field definitions (`i8` through `i128`), generating dynamic arithmetic
/// right-shifts for automatic sign-extension without branching.
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
            // Compile-time check: Ensure the base storage is a valid primitive.
            let _ = <$base_type as $crate::IsValidBaseInt>::ASSERT_VALID;

            // Compile-time check: Ensure each field type is valid.
            $crate::bitstruct!(@check_fields $($field_type)*);

            #[allow(dead_code)]
            const TOTAL_BITS: usize = 0 $( + $bits )*;
            assert!(TOTAL_BITS <= <$base_type as $crate::IsValidBaseInt>::MAX_BITS, "Sum of field bits exceeds base type max bits");
        };

        impl $struct_name {
            #[allow(dead_code)]
            pub const BITS: usize = <$base_type as $crate::BitLength>::BITS;

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
            /// The bit-offset of the `$field_name` property within the underlying storage.
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            /// The number of bits allocated for the `$field_name` property.
            pub const [<$field_name:upper _BITS>]: usize = $bits;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

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
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i8 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i8 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i16 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i16 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i32 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i32 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i64 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i64 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i128 $bits:tt $($rest:tt)*) => { $crate::bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i128 $bits $($rest)*); };

    // Standard integer implementation: Extracts the requested bit width, shifting by the accumulated offset.
    (@impl_int $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            /// The bit-offset of the `$field_name` property within the underlying storage.
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            /// The number of bits allocated for the `$field_name` property.
            pub const [<$field_name:upper _BITS>]: usize = $bits;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

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

    // Signed integer implementation: Handles sign extension on read and bounds-checked writing for two's complement fields.
    //
    // This macro arm intercepts explicit signed field specifications (`i8` through `i128`).
    // Read operations perform a zero-cost "shift trick":
    // 1. Mask the underlying base integer down to the requested bits.
    // 2. Cast into the signed target primitive.
    // 3. Shift the isolated bits up so the target's sign bit aligns with the Most Significant Bit of the field length.
    // 4. Arithmetic right-shift down to naturally propagate the sign bit across the full integer span.
    //
    // Write operations dynamically calculate MIN and MAX limits using an overflow-safe `(!0 << (bits - 1))` shift,
    // ensuring data integrity across partial boundaries.
    (@impl_signed_int $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            /// The bit-offset of the `$field_name` property within the underlying storage.
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            /// The number of bits allocated for the `$field_name` property.
            pub const [<$field_name:upper _BITS>]: usize = $bits;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[doc(hidden)]
            pub const [<$field_name:upper _MIN>]: $field_type = (!0 as $field_type) << (Self::[<$field_name:upper _BITS>] - 1);
            #[doc(hidden)]
            pub const [<$field_name:upper _MAX>]: $field_type = !Self::[<$field_name:upper _MIN>];
            #[doc(hidden)]
            const [<$field_name:upper _SHIFT_UP>]: usize = <$field_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>];

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Returns the `", stringify!($field_name), "` property as a signed `", stringify!($field_type), "`.")]
            $field_vis const fn $field_name(self) -> $field_type {
                let raw = ((self.0 >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as $field_type;
                (raw << Self::[<$field_name:upper _SHIFT_UP>]) >> Self::[<$field_name:upper _SHIFT_UP>]
            }

            #[allow(dead_code)]
            #[inline]
            #[doc = concat!("Inline mutation to apply the `", stringify!($field_name), "` signed property. Ensures bounds checking.")]
            $field_vis fn [<set_ $field_name>](&mut self, val: $field_type) {
                debug_assert!(val >= Self::[<$field_name:upper _MIN>] && val <= Self::[<$field_name:upper _MAX>], "Value {} out of bounds for {} bits", val, $bits);
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bitfield with the `", stringify!($field_name), "` signed property mapped.")]
            $field_vis const fn [<with_ $field_name>](self, val: $field_type) -> Self {
                debug_assert!(val >= Self::[<$field_name:upper _MIN>] && val <= Self::[<$field_name:upper _MAX>], "Value overflows allocated bits");
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict inline mutation to apply the `", stringify!($field_name), "` signed property. Returns a `BitstructError` if out of bounds.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                if val < Self::[<$field_name:upper _MIN>] || val > Self::[<$field_name:upper _MAX>] {
                    return Err($crate::BitstructError::Overflow { value: val as i128 as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
                Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict cloned evaluation to apply the `", stringify!($field_name), "` signed property. Returns a `BitstructError` if out of bounds.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                if val < Self::[<$field_name:upper _MIN>] || val > Self::[<$field_name:upper _MAX>] {
                    return Err($crate::BitstructError::Overflow { value: val as i128 as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                Ok(Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>])))
            }
        }

        $crate::bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };


    // Wrapped Type arm: Similar to impl_int, but handles types passed in parentheses (like from byteval! expansion)
    // or trait-associated types. Since these are expected to be primitives, we use `as` casting.
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident ($field_type:ty) $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            /// The bit-offset of the `$field_name` property within the underlying storage.
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            /// The number of bits allocated for the `$field_name` property.
            pub const [<$field_name:upper _BITS>]: usize = $bits;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[allow(dead_code)]
            #[inline]
            $field_vis const fn $field_name(self) -> $field_type {
                ((self.0 >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as $field_type
            }

            #[allow(dead_code)]
            #[inline]
            $field_vis fn [<set_ $field_name>](&mut self, val: $field_type) {
                debug_assert!((val as $base_type) <= Self::[<$field_name:upper _MASK>], "Value {} overflows allocated {} bits", val, $bits);
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
            }

            #[allow(dead_code)]
            $field_vis const fn [<with_ $field_name>](self, val: $field_type) -> Self {
                debug_assert!((val as $base_type) <= Self::[<$field_name:upper _MASK>], "Value overflows allocated bits");
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
            }

            #[allow(dead_code)]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                if (val as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: (val as $base_type) as u128, allocated_bits: $bits });
                }
                self.[<set_ $field_name>](val);
                Ok(())
            }

            #[allow(dead_code)]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                if (val as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: (val as $base_type) as u128, allocated_bits: $bits });
                }
                Ok(self.[<with_ $field_name>](val))
            }
        }

        $crate::bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };

    // TT Muncher Fallback
    // Uses `from_bits` to upcast/downcast the extracted integer bits back into the Enum variant safely.
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            /// The bit-offset of the `$field_name` property within the underlying storage.
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            /// The number of bits allocated for the `$field_name` property.
            pub const [<$field_name:upper _BITS>]: usize = $bits;

            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

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
                const _: () = assert!(<$field_type>::BITS <= $bits, "Enum bit width exceeds allocated field width");
                // Cast the enum's inner value up to the bitfield's base storage type before shifting
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0 = (self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]);
            }

            #[allow(dead_code)]
            #[doc = concat!("Returns a cloned copy of the bitfield bounded by the `", stringify!($field_type), "` enumeration supplied to `", stringify!($field_name), "`.")]
            $field_vis const fn [<with_ $field_name>](self, val: $field_type) -> Self {
                const _: () = assert!(<$field_type>::BITS <= $bits, "Enum bit width exceeds allocated field width");
                // Cast the enum's inner value up to the bitfield's base storage type before shifting
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                Self((self.0 & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict inline mutation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis fn [<try_set_ $field_name>](&mut self, val: $field_type) -> Result<(), $crate::BitstructError> {
                self.[<set_ $field_name>](val);
                Ok(())
            }

            #[allow(dead_code)]
            #[doc = concat!("Strict cloned evaluation to apply the bounded `", stringify!($field_type), "` enumeration to the `", stringify!($field_name), "` property. Returns a `BitstructError` if the value overflows ", stringify!($bits), " bits.")]
            $field_vis const fn [<try_with_ $field_name>](self, val: $field_type) -> Result<Self, $crate::BitstructError> {
                Ok(self.[<with_ $field_name>](val))
            }
        }

        $crate::bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };
}
