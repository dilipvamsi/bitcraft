/// A declarative macro for generating zero-cost bitenums.
///
/// This generates a `#[repr(transparent)]` struct with associated constants, simulating an enum.
/// It takes the exact number of bits required and automatically resolves to the narrowest
/// standard CPU primitive (`u8` to `u128`) capable of holding those bits.
///
/// # Examples
///
/// ### Standard Enumeration
/// The standard definition yields an unsigned enum mapped to the narrowest primitive.
/// ```rust
/// use bitcraft::bitenum;
///
/// bitenum! {
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
///
/// ### Signed Enumeration
/// Enums can use the `i` modifier before the bit width to indicate they represent
/// signed Two's Complement states. This is extremely useful for error codes.
/// ```rust
/// use bitcraft::bitenum;
///
/// bitenum! {
///     pub enum ErrorCode(i 3) {
///         SUCCESS = 0,
///         TIMEOUT = -1,
///         OVERFLOW = -2,
///         CORRUPTED = -3,
///         FATAL = -4, // -4 is the exact minimum bound for a 3-bit signed integer
///     }
/// }
///
/// let code = ErrorCode::from_bits(-1);
/// assert_eq!(code, ErrorCode::TIMEOUT);
/// ```
///
/// ### Safe Parsing from External Data
/// Because bitenums act as wrappers, they expose `try_from_bits` to safely validate incoming data
/// from the network or disk.
/// ```rust
/// use bitcraft::{bitenum, BitstructError};
///
/// bitenum! {
///     pub enum PacketType(2) {
///         SYN = 0,
///         ACK = 1,
///     }
/// }
///
/// let res = PacketType::try_from_bits(3);
/// assert!(matches!(res, Err(BitstructError::InvalidVariant { .. })));
/// ```
///
/// # Implementation Details
///
/// `bitenum!` uses `crate::Bits<$bits>` and the `BitenumType` or `SignedBitenumType` traits
/// to evaluate the narrowest CPU primitive capable of holding `$bits` bits at compile-time.
/// The resulting struct wraps this primitive, and validates bounds using `debug_assert!` based
/// on the specific number of allocated bits rather than the primitive's maximum capacity.
///
/// When used inside `bitstruct!` or `bytestruct!`, the macro leverages a specialized TT muncher arm
/// to enforce that the assigned field width perfectly safely matches or accommodates the bounds of the
/// bitenum enumeration without truncation.
#[macro_export]
macro_rules! bitenum {
    // Explicit Unsigned Enum
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::bitenum! {
            @impl_unsigned_enum
            [ $($meta)* ]
            $vis $enum_name $bits,
            { $($variant = $val),* }
        }
    };

    // Explicit Signed Enum
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::bitenum! {
            @impl_signed_enum
            [ $($meta)* ]
            $vis $enum_name $bits,
            { $($variant = $val),* }
        }
    };

    // Legacy Fallback (defaults to unsigned)
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident ($bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::bitenum! {
            @impl_unsigned_enum
            [ $($meta)* ]
            $vis $enum_name $bits,
            { $($variant = $val),* }
        }
    };

    // Internal implementation for unsigned enums
    (
        @impl_unsigned_enum
        [ $($meta:meta)* ]
        $vis:vis $enum_name:ident $bits:expr,
        {
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

            #[allow(dead_code)]
            /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
            ///
            /// Useful for manually validating raw input before conversion.
            pub const MASK: <$crate::Bits<$bits> as $crate::BitenumType>::Prim = {
                type Prim = <$crate::Bits<$bits> as $crate::BitenumType>::Prim;
                #[allow(dead_code)]
                const TOTAL_BITS: usize = <Prim as $crate::BitLength>::BITS;
                (!0 as Prim) >> (TOTAL_BITS - $bits)
            };

            /// Returns true if the raw value corresponds to a defined enumeration variant.
            ///
            /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
            #[inline(always)]
            #[allow(dead_code)]
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

    // Internal implementation for signed enums
    (
        @impl_signed_enum
        [ $($meta:meta)* ]
        $vis:vis $enum_name:ident $bits:expr,
        {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Default)]
        #[derive($crate::bytemuck::Pod, $crate::bytemuck::Zeroable)]
        #[repr(transparent)]
        $vis struct $enum_name(pub <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim);

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

            #[allow(dead_code)]
            /// The minimum value allowed for this enumeration variant based on the allocated $bits bits.
            pub const MIN: <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim = (!0 as <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim) << ($bits - 1);

            #[allow(dead_code)]
            /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
            pub const MAX: <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim = !Self::MIN;

            /// Returns true if the raw value corresponds to a defined enumeration variant.
            ///
            /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn is_defined(self) -> bool {
                match self.0 {
                    $( $val => true, )*
                    _ => false,
                }
            }

            /// Returns the raw integer value of the enumeration variant.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn to_bits(self) -> <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim { self.0 }

            /// Creates an enumeration variant from a raw integer value.
            ///
            /// # Panics
            /// In debug mode, this will panic if the value exceeds the allocated bit width.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn from_bits(mut val: <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim) -> Self {
                const SHIFT_UP: usize = <<$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim as $crate::BitLength>::BITS - $bits;
                val = (val << SHIFT_UP) >> SHIFT_UP;
                debug_assert!(val >= Self::MIN && val <= Self::MAX, "Value overflows allocated bit width for this signed enumeration");
                Self(val)
            }

            /// Creates an enumeration variant from a raw integer value, returning an error if it is invalid.
            ///
            /// This returns `Ok(Self)` if the value corresponds to a defined variant,
            /// or `Err(BitstructError::InvalidVariant)` if it does not.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn try_from_bits(mut val: <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim) -> Result<Self, $crate::BitstructError> {
                const SHIFT_UP: usize = <<$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim as $crate::BitLength>::BITS - $bits;
                val = (val << SHIFT_UP) >> SHIFT_UP;
                let s = Self(val);
                if s.is_defined() {
                    Ok(s)
                } else {
                    Err($crate::BitstructError::InvalidVariant { value: (val as i128) as u128, enum_name: stringify!($enum_name) })
                }
            }
        }

        impl $crate::ValidField for $enum_name {
            const ASSERT_VALID: () = ();
        }

        const _: () = {
            type Prim = <$crate::Bits<$bits> as $crate::SignedBitenumType>::Prim;
            $(
                assert!(
                    ($val as Prim) >= ((!0 as Prim) << ($bits - 1)) && ($val as Prim) <= !((!0 as Prim) << ($bits - 1)),
                    "Enum variant exceeds the maximum bounds for the allocated signed bit width"
                );
            )*
        };
    };
}
