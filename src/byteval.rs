/// A unique shorthand macro for creating "NewType" byte-array wrappers with a primary value field.
///
/// This is a specialized feature of `bitstruct` designed for high-density cases like
/// 24-bit (3-byte) or 40-bit (5-byte) IDs where you want a typed wrapper over a
/// byte array that behaves like a first-class integer.
///
/// This solves the "Odd-Width Integer" problem without requiring external crates or
/// manual 4-byte padding.
///
/// # Examples
///
/// ### Standard Odd-Width ID Wrapper
/// ```rust
/// use bitcraft::byteval;
///
/// // Generates a typed wrapper identically sized to `[u8; 3]`
/// byteval! { pub struct OrderId(3); }
///
/// let id = OrderId::from_u32(0x123456);
/// assert_eq!(id.value(), 0x123456);
/// ```
///
/// ### Signed Hardware Sensors
/// Some external devices like 24-bit ADCs produce oddly-sized two's-complement
/// numbers. Using the `i` modifier perfectly sign-extends these into native `i32` variables.
/// ```rust
/// use bitcraft::byteval;
///
/// // Wrapping a 3-byte array as a signed integer
/// byteval! { pub struct AdcReading24(i 3); }
///
/// // 0xFFFFFF in 24-bit Two's Complement is -1
/// let reading = AdcReading24::from_bits(0xFFFFFF);
/// assert_eq!(reading.value(), -1); // Automatically sign-extended to native i32!
/// ```
///
/// ### Wide Arrays and Explicit Units
/// You can utilize storage units larger than `u8`.
/// ```rust
/// use bitcraft::byteval;
///
/// // Generates a wrapper over a `[u32; 3]` array, yielding a 96-bit value
/// byteval! { pub struct Hash96(3, u32); }
///
/// let hash = Hash96::from_u128(0xFFEEDDCC_BBAA9988_77665544);
/// assert_eq!(hash.value(), 0xFFEEDDCC_BBAA9988_77665544);
/// ```
///
/// # Implementation Details
///
/// `byteval!` is essentially syntactic sugar over `bytestruct!`. It automatically maps the specified `$count`
/// of bytes into a single field named `value`, which occupies the entirety of the allocated byte array.
///
/// For example, `byteval! { pub struct Id24(3); }` translates to:
/// ```rust,ignore
/// bytestruct! {
///     pub struct Id24([u8; 3]) {
///         pub value: u32 = 24
///     }
/// }
/// ```
/// It also supports explicit unit boundaries (e.g. `(3, u16)`) and signed integers (e.g. `(i 3)`).
/// For signed identifiers, `byteval!` uses `@unroll_signed` to correctly generate shift-based sign-extensions.
#[macro_export]
macro_rules! byteval {
    // Branch for specialized storage unit
    ($(#[$meta:meta])* $vis:vis struct $name:ident ($count:tt, $unit:tt);) => {
        $crate::bytestruct! {
            $(#[$meta])* $vis struct $name ([$unit; $count]) {
                pub value: (<$crate::Bits<{ $crate::bits_mul!($unit, $count) }> as $crate::BitenumType>::Prim) = @unroll($count)
            }
        }
    };
    // Special shorthand for byte-based counts (u8 default)
    ($(#[$meta:meta])* $vis:vis struct $name:ident (1);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (1, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (2);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (2, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (3);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (3, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (4);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (4, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (5);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (5, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (6);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (6, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (7);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (7, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (8);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (8, u8); } };

    // Branch for specialized storage unit (Signed)
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i $count:tt, $unit:tt);) => {
        $crate::bytestruct! {
            $(#[$meta])* $vis struct $name ([$unit; $count]) {
                pub value: (<$crate::Bits<{ $crate::bits_mul!($unit, $count) }> as $crate::SignedBitenumType>::Prim) = @unroll_signed($count)
            }
        }
    };
    // Special shorthand for byte-based counts (Signed, u8 default)
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 1);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 1, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 2, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 3);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 3, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 4, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 5);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 5, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 6);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 6, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 7);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 7, u8); } };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 8);) => { $crate::byteval! { $(#[$meta])* $vis struct $name (i 8, u8); } };

    // Generic fallback for signed count
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i $count:expr);) => {
        $crate::byteval! { $(#[$meta])* $vis struct $name (i $count, u8); }
    };

    // Generic fallback for unsigned count
    ($(#[$meta:meta])* $vis:vis struct $name:ident ($count:expr);) => {
        $crate::byteval! { $(#[$meta])* $vis struct $name ($count, u8); }
    };
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
