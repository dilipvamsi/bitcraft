#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use bitcraft::{bitenum, bitstruct, bytestruct, byteval};
/// A sample enumeration for status tracking.
#[repr(transparent)]
pub struct Status(pub <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim);
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Pod>() {}
        assert_impl::<<::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim>();
    }
};
unsafe impl ::bytemuck::Pod for Status {}
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Zeroable>() {}
        assert_impl::<<::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim>();
    }
};
unsafe impl ::bytemuck::Zeroable for Status {}
#[automatically_derived]
impl ::core::marker::Copy for Status {}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Status {}
#[automatically_derived]
impl ::core::clone::Clone for Status {
    #[inline]
    fn clone(&self) -> Status {
        let _: ::core::clone::AssertParamIsClone<
            <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim,
        >;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Status {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Status {
    #[inline]
    fn eq(&self, other: &Status) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Status {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_fields_are_eq(&self) {
        let _: ::core::cmp::AssertParamIsEq<
            <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim,
        >;
    }
}
#[automatically_derived]
impl ::core::default::Default for Status {
    #[inline]
    fn default() -> Status {
        Status(::core::default::Default::default())
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self.0 {
            0 => "OFF",
            1 => "ON",
            2 => "FAULT",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{0}({1})::{2}", "Status", self.0, s))
    }
}
impl Status {
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `OFF` with raw value `0`.
    pub const OFF: Self = Self(0);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `ON` with raw value `1`.
    pub const ON: Self = Self(1);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `FAULT` with raw value `2`.
    pub const FAULT: Self = Self(2);
    #[allow(dead_code)]
    /// The number of bits allocated for this enumeration in memory.
    pub const BITS: usize = 2;
    /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
    ///
    /// Useful for manually validating raw input before conversion.
    pub const MASK: <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim = {
        type Prim = <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim;
        const TOTAL_BITS: usize = <Prim as ::bitcraft::BitLength>::BITS;
        (!0 as Prim) >> (TOTAL_BITS - 2)
    };
    /// Returns true if the raw value corresponds to a defined enumeration variant.
    ///
    /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
    #[inline(always)]
    pub const fn is_defined(self) -> bool {
        match self.0 {
            0 => true,
            1 => true,
            2 => true,
            _ => false,
        }
    }
    /// Returns the raw integer value of the enumeration variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim {
        self.0
    }
    /// Creates an enumeration variant from a raw integer value.
    ///
    /// # Panics
    /// In debug mode, this will panic if the value exceeds the allocated bit width.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !(val <= Self::MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value overflows allocated bit width for this enumeration",
                        ),
                    );
                }
            }
        }
        Self(val)
    }
    /// Creates an enumeration variant from a raw integer value, returning an error if it is invalid.
    ///
    /// This returns `Ok(Self)` if the value corresponds to a defined variant,
    /// or `Err(BitstructError::InvalidVariant)` if it does not.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn try_from_bits(
        val: <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        let s = Self(val);
        if s.is_defined() {
            Ok(s)
        } else {
            Err(::bitcraft::BitstructError::InvalidVariant {
                value: val as u128,
                enum_name: "Status",
            })
        }
    }
}
impl ::bitcraft::ValidField for Status {
    const ASSERT_VALID: () = ();
}
const _: () = {
    type Prim = <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim;
    if !((0 as Prim) <= ((!0 as Prim) >> (<Prim as ::bitcraft::BitLength>::BITS - 2))) {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "Enum variant exceeds the maximum value for the allocated bit width",
                ),
            );
        }
    }
    if !((1 as Prim) <= ((!0 as Prim) >> (<Prim as ::bitcraft::BitLength>::BITS - 2))) {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "Enum variant exceeds the maximum value for the allocated bit width",
                ),
            );
        }
    }
    if !((2 as Prim) <= ((!0 as Prim) >> (<Prim as ::bitcraft::BitLength>::BITS - 2))) {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "Enum variant exceeds the maximum value for the allocated bit width",
                ),
            );
        }
    }
};
/// A sample 16-bit packed configuration.
#[repr(transparent)]
pub struct Config(pub u16);
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Pod>() {}
        assert_impl::<u16>();
    }
};
unsafe impl ::bytemuck::Pod for Config {}
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Zeroable>() {}
        assert_impl::<u16>();
    }
};
unsafe impl ::bytemuck::Zeroable for Config {}
#[automatically_derived]
impl ::core::marker::Copy for Config {}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Config {}
#[automatically_derived]
impl ::core::clone::Clone for Config {
    #[inline]
    fn clone(&self) -> Config {
        let _: ::core::clone::AssertParamIsClone<u16>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Config {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Config {
    #[inline]
    fn eq(&self, other: &Config) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Config {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_fields_are_eq(&self) {
        let _: ::core::cmp::AssertParamIsEq<u16>;
    }
}
#[automatically_derived]
impl ::core::default::Default for Config {
    #[inline]
    fn default() -> Config {
        Config(::core::default::Default::default())
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("raw", &self.0)
            .field("enabled", &self.enabled())
            .field("mode", &self.mode())
            .field("status", &self.status())
            .field("data", &self.data())
            .finish()
    }
}
const _: () = {
    let _ = <u16 as ::bitcraft::IsUnsignedInt>::ASSERT_UNSIGNED;
    let _ = <bool as ::bitcraft::ValidField>::ASSERT_VALID;
    let _ = <u8 as ::bitcraft::ValidField>::ASSERT_VALID;
    let _ = <Status as ::bitcraft::ValidField>::ASSERT_VALID;
    let _ = <u16 as ::bitcraft::ValidField>::ASSERT_VALID;
    const TOTAL_BITS: usize = 0 + 1 + 3 + 2 + 10;
    if !(TOTAL_BITS <= <u16 as ::bitcraft::BitLength>::BITS) {
        {
            ::core::panicking::panic_fmt(
                format_args!("Sum of field bits exceeds base type size"),
            );
        }
    }
};
impl Config {
    #[doc(hidden)]
    #[allow(dead_code)]
    const ENABLED_OFFSET: usize = 0;
    #[doc(hidden)]
    const ENABLED_MASK: u16 = (!0 as u16) >> (<u16 as ::bitcraft::BitLength>::BITS - 1);
    #[allow(dead_code)]
    #[inline]
    ///Returns the boolean value mapping to the `enabled` flag.
    pub const fn enabled(self) -> bool {
        ((self.0 >> Self::ENABLED_OFFSET) & Self::ENABLED_MASK) != 0
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to set the `enabled` flag.
    pub fn set_enabled(&mut self, val: bool) {
        let val_masked = val as u16;
        self.0 = (self.0 & !(Self::ENABLED_MASK << Self::ENABLED_OFFSET))
            | (val_masked << Self::ENABLED_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `enabled` flag specified.
    pub const fn with_enabled(self, val: bool) -> Self {
        let val_masked = val as u16;
        Self(
            (self.0 & !(Self::ENABLED_MASK << Self::ENABLED_OFFSET))
                | (val_masked << Self::ENABLED_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Inline mutation to set the `enabled` flag. Returns `Ok(())` since booleans cannot overflow.
    pub fn try_set_enabled(
        &mut self,
        val: bool,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_enabled(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `enabled` flag specified. Returns `Ok(Self)` since booleans cannot overflow.
    pub const fn try_with_enabled(
        self,
        val: bool,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_enabled(val))
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const MODE_OFFSET: usize = 0 + 1;
    #[doc(hidden)]
    const MODE_MASK: u16 = (!0 as u16) >> (<u16 as ::bitcraft::BitLength>::BITS - 3);
    #[allow(dead_code)]
    #[inline]
    ///Returns the `mode` property as a `u8`.
    pub const fn mode(self) -> u8 {
        ((self.0 >> Self::MODE_OFFSET) & Self::MODE_MASK) as u8
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `mode` property. Masks inputs over 3 bits.
    pub fn set_mode(&mut self, val: u8) {
        if true {
            if !((val as u16) <= Self::MODE_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 3),
                    );
                }
            }
        }
        let val_masked = (val as u16) & Self::MODE_MASK;
        self.0 = (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
            | (val_masked << Self::MODE_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `mode` property mapped. Masks inputs over 3 bits.
    pub const fn with_mode(self, val: u8) -> Self {
        if true {
            if !((val as u16) <= Self::MODE_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u16) & Self::MODE_MASK;
        Self(
            (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
                | (val_masked << Self::MODE_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `mode` property. Returns a `BitstructError` if the value overflows 3 bits.
    pub fn try_set_mode(&mut self, val: u8) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u16) > Self::MODE_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u16) as u128,
                allocated_bits: 3,
            });
        }
        let val_masked = (val as u16) & Self::MODE_MASK;
        self.0 = (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
            | (val_masked << Self::MODE_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `mode` property. Returns a `BitstructError` if the value overflows 3 bits.
    pub const fn try_with_mode(
        self,
        val: u8,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u16) > Self::MODE_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u16) as u128,
                allocated_bits: 3,
            });
        }
        let val_masked = (val as u16) & Self::MODE_MASK;
        Ok(
            Self(
                (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
                    | (val_masked << Self::MODE_OFFSET),
            ),
        )
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const STATUS_OFFSET: usize = 0 + 1 + 3;
    #[doc(hidden)]
    const STATUS_MASK: u16 = (!0 as u16) >> (<u16 as ::bitcraft::BitLength>::BITS - 2);
    #[allow(dead_code)]
    ///Returns the `status` variant strictly typed to the `Status` enumeration.
    pub const fn status(self) -> Status {
        Status::from_bits(((self.0 >> Self::STATUS_OFFSET) & Self::STATUS_MASK) as _)
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the bounded `Status` enumeration to the `status` property.
    pub fn set_status(&mut self, val: Status) {
        if true {
            if !((val.to_bits() as u16) <= Self::STATUS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Enum variant overflows allocated {0} bits", 2),
                    );
                }
            }
        }
        let val_masked = (val.to_bits() as u16) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield bounded by the `Status` enumeration supplied to `status`.
    pub const fn with_status(self, val: Status) -> Self {
        if true {
            if !((val.to_bits() as u16) <= Self::STATUS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Enum variant overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val.to_bits() as u16) & Self::STATUS_MASK;
        Self(
            (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
                | (val_masked << Self::STATUS_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the bounded `Status` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub fn try_set_status(
        &mut self,
        val: Status,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val.to_bits() as u16) > Self::STATUS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val.to_bits() as u128,
                allocated_bits: 2,
            });
        }
        let val_masked = (val.to_bits() as u16) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the bounded `Status` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub const fn try_with_status(
        self,
        val: Status,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val.to_bits() as u16) > Self::STATUS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val.to_bits() as u128,
                allocated_bits: 2,
            });
        }
        let val_masked = (val.to_bits() as u16) & Self::STATUS_MASK;
        Ok(
            Self(
                (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
                    | (val_masked << Self::STATUS_OFFSET),
            ),
        )
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const DATA_OFFSET: usize = 0 + 1 + 3 + 2;
    #[doc(hidden)]
    const DATA_MASK: u16 = (!0 as u16) >> (<u16 as ::bitcraft::BitLength>::BITS - 10);
    #[allow(dead_code)]
    #[inline]
    ///Returns the `data` property as a `u16`.
    pub const fn data(self) -> u16 {
        ((self.0 >> Self::DATA_OFFSET) & Self::DATA_MASK) as u16
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `data` property. Masks inputs over 10 bits.
    pub fn set_data(&mut self, val: u16) {
        if true {
            if !((val as u16) <= Self::DATA_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 10),
                    );
                }
            }
        }
        let val_masked = (val as u16) & Self::DATA_MASK;
        self.0 = (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
            | (val_masked << Self::DATA_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `data` property mapped. Masks inputs over 10 bits.
    pub const fn with_data(self, val: u16) -> Self {
        if true {
            if !((val as u16) <= Self::DATA_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u16) & Self::DATA_MASK;
        Self(
            (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
                | (val_masked << Self::DATA_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `data` property. Returns a `BitstructError` if the value overflows 10 bits.
    pub fn try_set_data(&mut self, val: u16) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u16) > Self::DATA_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u16) as u128,
                allocated_bits: 10,
            });
        }
        let val_masked = (val as u16) & Self::DATA_MASK;
        self.0 = (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
            | (val_masked << Self::DATA_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `data` property. Returns a `BitstructError` if the value overflows 10 bits.
    pub const fn try_with_data(
        self,
        val: u16,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u16) > Self::DATA_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u16) as u128,
                allocated_bits: 10,
            });
        }
        let val_masked = (val as u16) & Self::DATA_MASK;
        Ok(
            Self(
                (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
                    | (val_masked << Self::DATA_OFFSET),
            ),
        )
    }
    /// Returns the raw interior integer value.
    ///
    /// This is useful for serializing the struct or passing it to external APIs.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u16 {
        self.0
    }
    /// Creates a new instance from a raw integer value.
    ///
    /// # Safety
    /// While this method is safe, providing values with bits set outside
    /// the defined field ranges may result in those bits being preserved
    /// (padded) or ignored depending on the getters used.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(val: u16) -> Self {
        Self(val)
    }
}
/// A sample 24-bit identifier (default u8 unit).
pub struct PackedId(pub [u8; 3]);
impl PackedId {
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<
        { (3) * <u8 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0] as u128
            | ((self.0[0 + 1] as u128) << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2] as u128) << (2 * <u8 as ::bitcraft::BitLength>::BITS)))
            >> 0) & ((!0u128) >> (128 - { (3) * <u8 as ::bitcraft::BitLength>::BITS })))
            as _
    }
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<
            { (3) * <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 3];
        {
            let mut full = (arr[0] as u128)
                | ((arr[0 + 1] as u128) << <u8 as ::bitcraft::BitLength>::BITS)
                | ((arr[0 + 2] as u128) << (2 * <u8 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128) >> (128 - { (3) * <u8 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> (2 * <u8 as ::bitcraft::BitLength>::BITS)) as u8;
        };
        Self(arr)
    }
    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self.to_bits() as u8
    }
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u16(self) -> u16 {
        self.to_bits() as u16
    }
    #[inline(always)]
    pub const fn from_u16(val: u16) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self.to_bits() as u32
    }
    #[inline(always)]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        self.to_bits() as u128
    }
    #[inline(always)]
    pub const fn from_u128(val: u128) -> Self {
        Self::from_bits(val as _)
    }
    pub const fn value(self) -> u128 {
        (((self.0[0] as u128
            | ((self.0[0 + 1] as u128) << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2] as u128) << (2 * <u8 as ::bitcraft::BitLength>::BITS)))
            >> 0) & ((!0u128) >> (128 - { (3) * <u8 as ::bitcraft::BitLength>::BITS })))
            as u128
    }
    pub fn set_value(&mut self, val: u128) {
        {
            let mut full = (self.0[0] as u128)
                | ((self.0[0 + 1] as u128) << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2] as u128) << (2 * <u8 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128) >> (128 - { (3) * <u8 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> (2 * <u8 as ::bitcraft::BitLength>::BITS)) as u8;
        };
    }
    pub const fn with_value(mut self, val: u128) -> Self {
        {
            let mut full = (self.0[0] as u128)
                | ((self.0[0 + 1] as u128) << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2] as u128) << (2 * <u8 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128) >> (128 - { (3) * <u8 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> (2 * <u8 as ::bitcraft::BitLength>::BITS)) as u8;
        };
        self
    }
}
/// A sample 48-bit identifier (u16 unit).
pub struct Id48(pub [u16; 3]);
impl Id48 {
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<
        { (3) * <u16 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0] as u128
            | ((self.0[0 + 1] as u128) << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2] as u128) << (2 * <u16 as ::bitcraft::BitLength>::BITS)))
            >> 0) & ((!0u128) >> (128 - { (3) * <u16 as ::bitcraft::BitLength>::BITS })))
            as _
    }
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<
            { (3) * <u16 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u16; 3];
        {
            let mut full = (arr[0] as u128)
                | ((arr[0 + 1] as u128) << <u16 as ::bitcraft::BitLength>::BITS)
                | ((arr[0 + 2] as u128) << (2 * <u16 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128)
                >> (128 - { (3) * <u16 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            arr[0] = full as u16;
            arr[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            arr[0 + 2] = (full >> (2 * <u16 as ::bitcraft::BitLength>::BITS)) as u16;
        };
        Self(arr)
    }
    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self.to_bits() as u8
    }
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u16(self) -> u16 {
        self.to_bits() as u16
    }
    #[inline(always)]
    pub const fn from_u16(val: u16) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self.to_bits() as u32
    }
    #[inline(always)]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        self.to_bits() as u128
    }
    #[inline(always)]
    pub const fn from_u128(val: u128) -> Self {
        Self::from_bits(val as _)
    }
    pub const fn value(self) -> u128 {
        (((self.0[0] as u128
            | ((self.0[0 + 1] as u128) << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2] as u128) << (2 * <u16 as ::bitcraft::BitLength>::BITS)))
            >> 0) & ((!0u128) >> (128 - { (3) * <u16 as ::bitcraft::BitLength>::BITS })))
            as u128
    }
    pub fn set_value(&mut self, val: u128) {
        {
            let mut full = (self.0[0] as u128)
                | ((self.0[0 + 1] as u128) << <u16 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2] as u128)
                    << (2 * <u16 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128)
                >> (128 - { (3) * <u16 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            self.0[0] = full as u16;
            self.0[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            self.0[0 + 2] = (full >> (2 * <u16 as ::bitcraft::BitLength>::BITS)) as u16;
        };
    }
    pub const fn with_value(mut self, val: u128) -> Self {
        {
            let mut full = (self.0[0] as u128)
                | ((self.0[0 + 1] as u128) << <u16 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2] as u128)
                    << (2 * <u16 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128)
                >> (128 - { (3) * <u16 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            self.0[0] = full as u16;
            self.0[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            self.0[0 + 2] = (full >> (2 * <u16 as ::bitcraft::BitLength>::BITS)) as u16;
        };
        self
    }
}
/// A sample 96-bit identifier with manual alignment.
#[repr(align(4))]
pub struct AlignedId96(pub [u32; 3]);
impl AlignedId96 {
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<
        { (3) * <u32 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0] as u128
            | ((self.0[0 + 1] as u128) << <u32 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2] as u128) << (2 * <u32 as ::bitcraft::BitLength>::BITS)))
            >> 0) & ((!0u128) >> (128 - { (3) * <u32 as ::bitcraft::BitLength>::BITS })))
            as _
    }
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<
            { (3) * <u32 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u32; 3];
        {
            let mut full = (arr[0] as u128)
                | ((arr[0 + 1] as u128) << <u32 as ::bitcraft::BitLength>::BITS)
                | ((arr[0 + 2] as u128) << (2 * <u32 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128)
                >> (128 - { (3) * <u32 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            arr[0] = full as u32;
            arr[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
            arr[0 + 2] = (full >> (2 * <u32 as ::bitcraft::BitLength>::BITS)) as u32;
        };
        Self(arr)
    }
    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self.to_bits() as u8
    }
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u16(self) -> u16 {
        self.to_bits() as u16
    }
    #[inline(always)]
    pub const fn from_u16(val: u16) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self.to_bits() as u32
    }
    #[inline(always)]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        self.to_bits() as u128
    }
    #[inline(always)]
    pub const fn from_u128(val: u128) -> Self {
        Self::from_bits(val as _)
    }
    pub const fn value(self) -> u128 {
        (((self.0[0] as u128
            | ((self.0[0 + 1] as u128) << <u32 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2] as u128) << (2 * <u32 as ::bitcraft::BitLength>::BITS)))
            >> 0) & ((!0u128) >> (128 - { (3) * <u32 as ::bitcraft::BitLength>::BITS })))
            as u128
    }
    pub fn set_value(&mut self, val: u128) {
        {
            let mut full = (self.0[0] as u128)
                | ((self.0[0 + 1] as u128) << <u32 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2] as u128)
                    << (2 * <u32 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128)
                >> (128 - { (3) * <u32 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            self.0[0] = full as u32;
            self.0[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
            self.0[0 + 2] = (full >> (2 * <u32 as ::bitcraft::BitLength>::BITS)) as u32;
        };
    }
    pub const fn with_value(mut self, val: u128) -> Self {
        {
            let mut full = (self.0[0] as u128)
                | ((self.0[0 + 1] as u128) << <u32 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2] as u128)
                    << (2 * <u32 as ::bitcraft::BitLength>::BITS));
            let mask = (!0u128)
                >> (128 - { (3) * <u32 as ::bitcraft::BitLength>::BITS });
            full = (full & !(mask << 0)) | ((val as u128 & mask) << 0);
            self.0[0] = full as u32;
            self.0[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
            self.0[0 + 2] = (full >> (2 * <u32 as ::bitcraft::BitLength>::BITS)) as u32;
        };
        self
    }
}
fn main() {
    let config = Config::default()
        .with_enabled(true)
        .with_mode(2)
        .with_status(Status::ON)
        .with_data(0x123);
    {
        ::std::io::_print(format_args!("Config: {0:?}\n", config));
    };
    {
        ::std::io::_print(format_args!("Enabled: {0}\n", config.enabled()));
    };
    {
        ::std::io::_print(format_args!("Mode: {0}\n", config.mode()));
    };
    {
        ::std::io::_print(format_args!("Status: {0:?}\n", config.status()));
    };
    {
        ::std::io::_print(format_args!("Data: 0x{0:X}\n", config.data()));
    };
    let coord = Coordinate::default().with_x(100).with_y(200).with_flags(0xFF);
    {
        ::std::io::_print(format_args!("\nCoordinate: {0:?}\n", coord));
    };
    {
        ::std::io::_print(
            format_args!(
                "X: {0}, Y: {1}, Flags: 0x{2:X}\n",
                coord.x(),
                coord.y(),
                coord.flags(),
            ),
        );
    };
    let id = PackedId::from_u32(0xABCDEF);
    {
        ::std::io::_print(format_args!("\nPackedId: {0:?}\n", id));
    };
    {
        ::std::io::_print(format_args!("Value: 0x{0:X}\n", id.value()));
    };
    let id48 = Id48::from_u64(0x112233445566);
    {
        ::std::io::_print(format_args!("\nId48 (48-bit): {0:?}\n", id48));
    };
    {
        ::std::io::_print(format_args!("Value: 0x{0:X}\n", id48.value()));
    };
    let al_id96 = AlignedId96::from_bits(0xDEADBEEFCAFEBABE11223344);
    {
        ::std::io::_print(format_args!("\nAlignedId96 (align 8): {0:?}\n", al_id96));
    };
    {
        ::std::io::_print(format_args!("Value: 0x{0:X}\n", al_id96.value()));
    };
    {
        ::std::io::_print(
            format_args!("Alignment: {0}\n", std::mem::align_of::<AlignedId96>()),
        );
    };
    {
        ::std::io::_print(
            format_args!("Size: {0}\n", std::mem::size_of::<AlignedId96>()),
        );
    };
}
