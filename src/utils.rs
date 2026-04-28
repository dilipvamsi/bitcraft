/// Internal utilities for Bitcraft, including compile-time const traits and multiplication macros.
/// Internal helper macro to perform zero-multiplication bit-width calculations.
#[macro_export]
#[doc(hidden)]
macro_rules! bits_mul {
    ($unit:tt, 1) => {
        <$unit as $crate::BitLength>::BITS
    };
    ($unit:tt, 2) => {
        <$unit as $crate::BitLength>::BITS_2
    };
    ($unit:tt, 3) => {
        <$unit as $crate::BitLength>::BITS_3
    };
    ($unit:tt, 4) => {
        <$unit as $crate::BitLength>::BITS_4
    };
    ($unit:tt, 5) => {
        <$unit as $crate::BitLength>::BITS_5
    };
    ($unit:tt, 6) => {
        <$unit as $crate::BitLength>::BITS_6
    };
    ($unit:tt, 7) => {
        <$unit as $crate::BitLength>::BITS_7
    };
    ($unit:tt, 8) => {
        <$unit as $crate::BitLength>::BITS_8
    };
    ($unit:tt, 9) => {
        <$unit as $crate::BitLength>::BITS_9
    };
    ($unit:tt, 10) => {
        <$unit as $crate::BitLength>::BITS_10
    };
    ($unit:tt, 11) => {
        <$unit as $crate::BitLength>::BITS_11
    };
    ($unit:tt, 12) => {
        <$unit as $crate::BitLength>::BITS_12
    };
    ($unit:tt, 13) => {
        <$unit as $crate::BitLength>::BITS_13
    };
    ($unit:tt, 14) => {
        <$unit as $crate::BitLength>::BITS_14
    };
    ($unit:tt, 15) => {
        <$unit as $crate::BitLength>::BITS_15
    };
    ($unit:tt, 16) => {
        <$unit as $crate::BitLength>::BITS_16
    };
    ($unit:tt, $any:expr) => {
        ($any) * <$unit as $crate::BitLength>::BITS
    };
}

use crate::byteval::*;

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

/// A hidden trait used to resolve the narrowest signed primitive type for a given bit-width.
#[doc(hidden)]
pub trait SignedBitenumType {
    /// The narrowest CPU signed primitive capable of holding $Bits$ bits.
    type Prim: Copy + Clone + PartialEq + Eq + core::fmt::Debug + Default;
}

macro_rules! impl_signed_bits {
    ($prim:ty, $($n:literal),+) => {
        $(
            impl SignedBitenumType for Bits<$n> {
                type Prim = $prim;
            }
        )+
    };
}

impl_signed_bits!(i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_signed_bits!(i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_signed_bits!(
    i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
impl_signed_bits!(
    i64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
    55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);
impl_signed_bits!(
    i128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
    87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
    108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
    127, 128
);
