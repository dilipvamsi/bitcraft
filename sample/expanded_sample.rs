#[repr(transparent)]
pub struct Status(pub <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim);

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
    fn assert_receiver_is_total_eq(&self) -> () {
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
    #[allow(dead_code)]
    /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
    ///
    /// Useful for manually validating raw input before conversion.
    pub const MASK: <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim = {
        type Prim = <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim;
        #[allow(dead_code)]
        const TOTAL_BITS: usize = <Prim as ::bitcraft::BitLength>::BITS;
        (!0 as Prim) >> (TOTAL_BITS - 2)
    };
    /// Returns true if the raw value corresponds to a defined enumeration variant.
    ///
    /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
    #[inline(always)]
    #[allow(dead_code)]
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

#[repr(transparent)]
pub struct SignedStatus(
    pub <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim,
);

#[automatically_derived]
impl ::core::marker::Copy for SignedStatus {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedStatus {}

#[automatically_derived]
impl ::core::clone::Clone for SignedStatus {
    #[inline]
    fn clone(&self) -> SignedStatus {
        let _: ::core::clone::AssertParamIsClone<
            <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim,
        >;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedStatus {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedStatus {
    #[inline]
    fn eq(&self, other: &SignedStatus) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedStatus {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim,
        >;
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedStatus {
    #[inline]
    fn default() -> SignedStatus {
        SignedStatus(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SignedStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self.0 {
            0 => "OFF",
            1 => "ON",
            -1 => "FAULT",
            -2 => "ERROR",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{0}({1})::{2}", "SignedStatus", self.0, s))
    }
}

impl SignedStatus {
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `OFF` with raw value `0`.
    pub const OFF: Self = Self(0);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `ON` with raw value `1`.
    pub const ON: Self = Self(1);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `FAULT` with raw value `-1`.
    pub const FAULT: Self = Self(-1);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `ERROR` with raw value `-2`.
    pub const ERROR: Self = Self(-2);
    #[allow(dead_code)]
    /// The number of bits allocated for this enumeration in memory.
    pub const BITS: usize = 2;
    #[allow(dead_code)]
    /// The minimum value allowed for this enumeration variant based on the allocated $bits bits.
    pub const MIN: <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim = (!0
        as <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim) << (2 - 1);
    #[allow(dead_code)]
    /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
    pub const MAX: <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim = !Self::MIN;
    /// Returns true if the raw value corresponds to a defined enumeration variant.
    ///
    /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_defined(self) -> bool {
        match self.0 {
            0 => true,
            1 => true,
            -1 => true,
            -2 => true,
            _ => false,
        }
    }
    /// Returns the raw integer value of the enumeration variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim {
        self.0
    }
    /// Creates an enumeration variant from a raw integer value.
    ///
    /// # Panics
    /// In debug mode, this will panic if the value exceeds the allocated bit width.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(
        mut val: <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Self {
        const SHIFT_UP: usize = <<::bitcraft::Bits<
            2,
        > as ::bitcraft::SignedBitenumType>::Prim as ::bitcraft::BitLength>::BITS - 2;
        val = (val << SHIFT_UP) >> SHIFT_UP;
        if true {
            if !(val >= Self::MIN && val <= Self::MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value overflows allocated bit width for this signed enumeration",
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
        mut val: <::bitcraft::Bits<2> as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        const SHIFT_UP: usize = <<::bitcraft::Bits<
            2,
        > as ::bitcraft::SignedBitenumType>::Prim as ::bitcraft::BitLength>::BITS - 2;
        val = (val << SHIFT_UP) >> SHIFT_UP;
        let s = Self(val);
        if s.is_defined() {
            Ok(s)
        } else {
            Err(::bitcraft::BitstructError::InvalidVariant {
                value: (val as i128) as u128,
                enum_name: "SignedStatus",
            })
        }
    }
}

impl ::bitcraft::ValidField for SignedStatus {
    const ASSERT_VALID: () = ();
}

#[repr(transparent)]
pub struct Config(pub u16);

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
    fn assert_receiver_is_total_eq(&self) -> () {
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

impl Config {
    #[allow(dead_code)]
    pub const BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const ENABLED_OFFSET: usize = 0;
    /// The number of bits allocated for the `$field_name` property.
    pub const ENABLED_BITS: usize = 1;
    #[doc(hidden)]
    const ENABLED_MASK: u16 = ((!0 as <u16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u16 as ::bitcraft::BitLength>::BITS - Self::ENABLED_BITS)) as u16;
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
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const MODE_OFFSET: usize = 0 + 1;
    /// The number of bits allocated for the `$field_name` property.
    pub const MODE_BITS: usize = 3;
    #[doc(hidden)]
    const MODE_MASK: u16 = ((!0 as <u16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u16 as ::bitcraft::BitLength>::BITS - Self::MODE_BITS)) as u16;
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
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const STATUS_OFFSET: usize = 0 + 1 + 3;
    /// The number of bits allocated for the `$field_name` property.
    pub const STATUS_BITS: usize = 2;
    #[doc(hidden)]
    const STATUS_MASK: u16 = ((!0 as <u16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u16 as ::bitcraft::BitLength>::BITS - Self::STATUS_BITS)) as u16;
    #[allow(dead_code)]
    ///Returns the `status` variant strictly typed to the `Status` enumeration.
    pub const fn status(self) -> Status {
        Status::from_bits(((self.0 >> Self::STATUS_OFFSET) & Self::STATUS_MASK) as _)
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the bounded `Status` enumeration to the `status` property.
    pub fn set_status(&mut self, val: Status) {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as u16) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield bounded by the `Status` enumeration supplied to `status`.
    pub const fn with_status(self, val: Status) -> Self {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
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
        self.set_status(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the bounded `Status` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub const fn try_with_status(
        self,
        val: Status,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_status(val))
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const DATA_OFFSET: usize = 0 + 1 + 3 + 2;
    /// The number of bits allocated for the `$field_name` property.
    pub const DATA_BITS: usize = 10;
    #[doc(hidden)]
    const DATA_MASK: u16 = ((!0 as <u16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u16 as ::bitcraft::BitLength>::BITS - Self::DATA_BITS)) as u16;
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

#[repr(transparent)]
pub struct SignedConfig(pub i16);

#[automatically_derived]
impl ::core::marker::Copy for SignedConfig {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedConfig {}

#[automatically_derived]
impl ::core::clone::Clone for SignedConfig {
    #[inline]
    fn clone(&self) -> SignedConfig {
        let _: ::core::clone::AssertParamIsClone<i16>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedConfig {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedConfig {
    #[inline]
    fn eq(&self, other: &SignedConfig) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedConfig {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i16>;
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedConfig {
    #[inline]
    fn default() -> SignedConfig {
        SignedConfig(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SignedConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SignedConfig")
            .field("raw", &self.0)
            .field("enabled", &self.enabled())
            .field("mode", &self.mode())
            .field("status", &self.status())
            .field("data", &self.data())
            .finish()
    }
}

impl SignedConfig {
    #[allow(dead_code)]
    pub const BITS: usize = <i16 as ::bitcraft::BitLength>::BITS;
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const ENABLED_OFFSET: usize = 0;
    /// The number of bits allocated for the `$field_name` property.
    pub const ENABLED_BITS: usize = 1;
    #[doc(hidden)]
    const ENABLED_MASK: i16 = ((!0 as <i16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<i16 as ::bitcraft::BitLength>::BITS - Self::ENABLED_BITS)) as i16;
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
        let val_masked = val as i16;
        self.0 = (self.0 & !(Self::ENABLED_MASK << Self::ENABLED_OFFSET))
            | (val_masked << Self::ENABLED_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `enabled` flag specified.
    pub const fn with_enabled(self, val: bool) -> Self {
        let val_masked = val as i16;
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
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const MODE_OFFSET: usize = 0 + 1;
    /// The number of bits allocated for the `$field_name` property.
    pub const MODE_BITS: usize = 3;
    #[doc(hidden)]
    const MODE_MASK: i16 = ((!0 as <i16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<i16 as ::bitcraft::BitLength>::BITS - Self::MODE_BITS)) as i16;
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
            if !((val as i16) <= Self::MODE_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 3),
                    );
                }
            }
        }
        let val_masked = (val as i16) & Self::MODE_MASK;
        self.0 = (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
            | (val_masked << Self::MODE_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `mode` property mapped. Masks inputs over 3 bits.
    pub const fn with_mode(self, val: u8) -> Self {
        if true {
            if !((val as i16) <= Self::MODE_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as i16) & Self::MODE_MASK;
        Self(
            (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
                | (val_masked << Self::MODE_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `mode` property. Returns a `BitstructError` if the value overflows 3 bits.
    pub fn try_set_mode(&mut self, val: u8) -> Result<(), ::bitcraft::BitstructError> {
        if (val as i16) > Self::MODE_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i16) as u128,
                allocated_bits: 3,
            });
        }
        let val_masked = (val as i16) & Self::MODE_MASK;
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
        if (val as i16) > Self::MODE_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i16) as u128,
                allocated_bits: 3,
            });
        }
        let val_masked = (val as i16) & Self::MODE_MASK;
        Ok(
            Self(
                (self.0 & !(Self::MODE_MASK << Self::MODE_OFFSET))
                    | (val_masked << Self::MODE_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const STATUS_OFFSET: usize = 0 + 1 + 3;
    /// The number of bits allocated for the `$field_name` property.
    pub const STATUS_BITS: usize = 2;
    #[doc(hidden)]
    const STATUS_MASK: i16 = ((!0 as <i16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<i16 as ::bitcraft::BitLength>::BITS - Self::STATUS_BITS)) as i16;
    #[allow(dead_code)]
    ///Returns the `status` variant strictly typed to the `Status` enumeration.
    pub const fn status(self) -> Status {
        Status::from_bits(((self.0 >> Self::STATUS_OFFSET) & Self::STATUS_MASK) as _)
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the bounded `Status` enumeration to the `status` property.
    pub fn set_status(&mut self, val: Status) {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as i16) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield bounded by the `Status` enumeration supplied to `status`.
    pub const fn with_status(self, val: Status) -> Self {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as i16) & Self::STATUS_MASK;
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
        self.set_status(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the bounded `Status` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub const fn try_with_status(
        self,
        val: Status,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_status(val))
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const DATA_OFFSET: usize = 0 + 1 + 3 + 2;
    /// The number of bits allocated for the `$field_name` property.
    pub const DATA_BITS: usize = 9;
    #[doc(hidden)]
    const DATA_MASK: i16 = ((!0 as <i16 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<i16 as ::bitcraft::BitLength>::BITS - Self::DATA_BITS)) as i16;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `data` property as a `u16`.
    pub const fn data(self) -> u16 {
        ((self.0 >> Self::DATA_OFFSET) & Self::DATA_MASK) as u16
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `data` property. Masks inputs over 9 bits.
    pub fn set_data(&mut self, val: u16) {
        if true {
            if !((val as i16) <= Self::DATA_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 9),
                    );
                }
            }
        }
        let val_masked = (val as i16) & Self::DATA_MASK;
        self.0 = (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
            | (val_masked << Self::DATA_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `data` property mapped. Masks inputs over 9 bits.
    pub const fn with_data(self, val: u16) -> Self {
        if true {
            if !((val as i16) <= Self::DATA_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as i16) & Self::DATA_MASK;
        Self(
            (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
                | (val_masked << Self::DATA_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `data` property. Returns a `BitstructError` if the value overflows 9 bits.
    pub fn try_set_data(&mut self, val: u16) -> Result<(), ::bitcraft::BitstructError> {
        if (val as i16) > Self::DATA_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i16) as u128,
                allocated_bits: 9,
            });
        }
        let val_masked = (val as i16) & Self::DATA_MASK;
        self.0 = (self.0 & !(Self::DATA_MASK << Self::DATA_OFFSET))
            | (val_masked << Self::DATA_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `data` property. Returns a `BitstructError` if the value overflows 9 bits.
    pub const fn try_with_data(
        self,
        val: u16,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as i16) > Self::DATA_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i16) as u128,
                allocated_bits: 9,
            });
        }
        let val_masked = (val as i16) & Self::DATA_MASK;
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
    pub const fn to_bits(self) -> i16 {
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
    pub const fn from_bits(val: i16) -> Self {
        Self(val)
    }
}

#[repr(C)]
pub struct Coordinate(pub [u8; 5]);

#[automatically_derived]
impl ::core::marker::Copy for Coordinate {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Coordinate {}

#[automatically_derived]
impl ::core::clone::Clone for Coordinate {
    #[inline]
    fn clone(&self) -> Coordinate {
        let _: ::core::clone::AssertParamIsClone<[u8; 5]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Coordinate {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Coordinate {
    #[inline]
    fn eq(&self, other: &Coordinate) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Coordinate {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 5]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Coordinate {
    #[inline]
    fn default() -> Coordinate {
        Coordinate(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Coordinate")
            .field("raw", &self.0)
            .field("x", &self.x())
            .field("y", &self.y())
            .field("flags", &self.flags())
            .finish()
    }
}

impl Coordinate {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_5;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 5] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 5]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)
            | ((self.0[0 + 4]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_4)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 5];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            arr[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            arr[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const X_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const X_BITS: usize = 16;
    #[allow(dead_code)]
    #[doc(hidden)]
    const X_MASK: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::X_BITS);
    #[allow(dead_code)]
    pub const fn x(self) -> u16 {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 16 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 16)))
            }
        }) as u16
    }
    #[allow(dead_code)]
    pub fn set_x(&mut self, val: u16) {
        if true {
            if !((val as u128) <= Self::X_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 16),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 16 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_x(mut self, val: u16) -> Self {
        if true {
            if !((val as u128) <= Self::X_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 16 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_x(&mut self, val: u16) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::X_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        self.set_x(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_x(self, val: u16) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::X_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        Ok(self.with_x(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const Y_OFFSET: usize = 0 + 16;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const Y_BITS: usize = 16;
    #[allow(dead_code)]
    #[doc(hidden)]
    const Y_MASK: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::Y_BITS);
    #[allow(dead_code)]
    pub const fn y(self) -> u16 {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16) as usize + 16 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 16)))
            }
        }) as u16
    }
    #[allow(dead_code)]
    pub fn set_y(&mut self, val: u16) {
        if true {
            if !((val as u128) <= Self::Y_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 16),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16) as usize + 16 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_y(mut self, val: u16) -> Self {
        if true {
            if !((val as u128) <= Self::Y_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16) as usize + 16 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_y(&mut self, val: u16) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::Y_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        self.set_y(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_y(self, val: u16) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::Y_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        Ok(self.with_y(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const FLAGS_OFFSET: usize = 0 + 16 + 16;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const FLAGS_BITS: usize = 8;
    #[allow(dead_code)]
    #[doc(hidden)]
    const FLAGS_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::FLAGS_BITS);
    #[allow(dead_code)]
    pub const fn flags(self) -> u8 {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16 + 16) as usize + 8 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 8)))
            }
        }) as u8
    }
    #[allow(dead_code)]
    pub fn set_flags(&mut self, val: u8) {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 8),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16 + 16) as usize + 8 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 8);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_flags(mut self, val: u8) -> Self {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16 + 16) as usize + 8 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 8);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_flags(&mut self, val: u8) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 8,
            });
        }
        self.set_flags(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_flags(
        self,
        val: u8,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 8,
            });
        }
        Ok(self.with_flags(val))
    }
}

#[repr(C)]
pub struct Telemetry(pub [u8; 7]);

#[automatically_derived]
impl ::core::marker::Copy for Telemetry {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Telemetry {}

#[automatically_derived]
impl ::core::clone::Clone for Telemetry {
    #[inline]
    fn clone(&self) -> Telemetry {
        let _: ::core::clone::AssertParamIsClone<[u8; 7]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Telemetry {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Telemetry {
    #[inline]
    fn eq(&self, other: &Telemetry) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Telemetry {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 7]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Telemetry {
    #[inline]
    fn default() -> Telemetry {
        Telemetry(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Telemetry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Telemetry")
            .field("raw", &self.0)
            .field("sensor_a", &self.sensor_a())
            .field("sensor_b", &self.sensor_b())
            .field("alert", &self.alert())
            .field("status", &self.status())
            .field("counter", &self.counter())
            .finish()
    }
}

impl Telemetry {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_7;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 7] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 7]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)
            | ((self.0[0 + 4]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_4)
            | ((self.0[0 + 5]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_5)
            | ((self.0[0 + 6]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_6)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 7];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            arr[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            arr[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
            arr[0 + 5] = (full >> <u8 as ::bitcraft::BitLength>::BITS_5) as u8;
            arr[0 + 6] = (full >> <u8 as ::bitcraft::BitLength>::BITS_6) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const SENSOR_A_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const SENSOR_A_BITS: usize = 20;
    #[allow(dead_code)]
    #[doc(hidden)]
    const SENSOR_A_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::SENSOR_A_BITS);
    #[allow(dead_code)]
    pub const fn sensor_a(self) -> u128 {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 20 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 20)))
            }
        }) as u128
    }
    #[allow(dead_code)]
    pub fn set_sensor_a(&mut self, val: u128) {
        if true {
            if !((val as u128) <= Self::SENSOR_A_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 20),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 20 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 20);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_sensor_a(mut self, val: u128) -> Self {
        if true {
            if !((val as u128) <= Self::SENSOR_A_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 20 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 20);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_sensor_a(
        &mut self,
        val: u128,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::SENSOR_A_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 20,
            });
        }
        self.set_sensor_a(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_sensor_a(
        self,
        val: u128,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::SENSOR_A_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 20,
            });
        }
        Ok(self.with_sensor_a(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const SENSOR_B_OFFSET: usize = 0 + 20;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const SENSOR_B_BITS: usize = 20;
    #[allow(dead_code)]
    #[doc(hidden)]
    const SENSOR_B_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::SENSOR_B_BITS);
    #[allow(dead_code)]
    pub const fn sensor_b(self) -> u128 {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20) as usize + 20 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 20)))
            }
        }) as u128
    }
    #[allow(dead_code)]
    pub fn set_sensor_b(&mut self, val: u128) {
        if true {
            if !((val as u128) <= Self::SENSOR_B_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 20),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20) as usize + 20 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 20);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_sensor_b(mut self, val: u128) -> Self {
        if true {
            if !((val as u128) <= Self::SENSOR_B_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20) as usize + 20 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 20);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_sensor_b(
        &mut self,
        val: u128,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::SENSOR_B_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 20,
            });
        }
        self.set_sensor_b(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_sensor_b(
        self,
        val: u128,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::SENSOR_B_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 20,
            });
        }
        Ok(self.with_sensor_b(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const ALERT_OFFSET: usize = 0 + 20 + 20;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const ALERT_BITS: usize = 1;
    #[allow(dead_code)]
    #[doc(hidden)]
    const ALERT_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::ALERT_BITS);
    #[allow(dead_code)]
    pub const fn alert(self) -> bool {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20) as usize + 1 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 1)))
            }
        }) != 0
    }
    #[allow(dead_code)]
    pub fn set_alert(&mut self, val: bool) {
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20) as usize + 1 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 1);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_alert(mut self, val: bool) -> Self {
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20) as usize + 1 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 1);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_alert(
        &mut self,
        val: bool,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_alert(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_alert(
        self,
        val: bool,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_alert(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const STATUS_OFFSET: usize = 0 + 20 + 20 + 1;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const STATUS_BITS: usize = 2;
    #[allow(dead_code)]
    #[doc(hidden)]
    const STATUS_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::STATUS_BITS);
    #[allow(dead_code)]
    pub const fn status(self) -> Status {
        Status::from_bits(
            {
                const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
                const WORD_INDEX: usize = (0 + 20 + 20 + 1) / UNIT_BITS;
                const BIT_OFFSET: usize = (0 + 20 + 20 + 1) % UNIT_BITS;
                const WORD_COUNT: usize = ((0 + 20 + 20 + 1) as usize + 2 as usize)
                    .div_ceil(UNIT_BITS) - WORD_INDEX;
                {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> BIT_OFFSET)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - 2)))
                }
            } as _,
        )
    }
    #[allow(dead_code)]
    pub fn set_status(&mut self, val: Status) {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20 + 1) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20 + 1) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20 + 1) as usize + 2 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 2);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val.to_bits()
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_status(mut self, val: Status) -> Self {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20 + 1) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20 + 1) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20 + 1) as usize + 2 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 2);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val.to_bits()
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_status(
        &mut self,
        val: Status,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_status(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_status(
        self,
        val: Status,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_status(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const COUNTER_OFFSET: usize = 0 + 20 + 20 + 1 + 2;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const COUNTER_BITS: usize = 13;
    #[allow(dead_code)]
    #[doc(hidden)]
    const COUNTER_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::COUNTER_BITS);
    #[allow(dead_code)]
    pub const fn counter(self) -> u16 {
        ({
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20 + 1 + 2) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20 + 1 + 2) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20 + 1 + 2) as usize + 13 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 13)))
            }
        }) as u16
    }
    #[allow(dead_code)]
    pub fn set_counter(&mut self, val: u16) {
        if true {
            if !((val as u128) <= Self::COUNTER_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 13),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20 + 1 + 2) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20 + 1 + 2) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20 + 1 + 2) as usize + 13 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 13);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_counter(mut self, val: u16) -> Self {
        if true {
            if !((val as u128) <= Self::COUNTER_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 20 + 20 + 1 + 2) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 20 + 20 + 1 + 2) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 20 + 20 + 1 + 2) as usize + 13 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 13);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_counter(
        &mut self,
        val: u16,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::COUNTER_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 13,
            });
        }
        self.set_counter(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_counter(
        self,
        val: u16,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::COUNTER_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 13,
            });
        }
        Ok(self.with_counter(val))
    }
}

#[repr(C)]
pub struct U16Packet(pub [u16; 3]);

#[automatically_derived]
impl ::core::marker::Copy for U16Packet {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for U16Packet {}

#[automatically_derived]
impl ::core::clone::Clone for U16Packet {
    #[inline]
    fn clone(&self) -> U16Packet {
        let _: ::core::clone::AssertParamIsClone<[u16; 3]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for U16Packet {}

#[automatically_derived]
impl ::core::cmp::PartialEq for U16Packet {
    #[inline]
    fn eq(&self, other: &U16Packet) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for U16Packet {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u16; 3]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for U16Packet {
    #[inline]
    fn default() -> U16Packet {
        U16Packet(::core::default::Default::default())
    }
}

impl core::fmt::Debug for U16Packet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("U16Packet")
            .field("raw", &self.0)
            .field("field_a", &self.field_a())
            .field("field_b", &self.field_b())
            .finish()
    }
}

impl U16Packet {
    #[allow(dead_code)]
    pub const BITS: usize = <u16 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u16; 3] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u16; 3]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u16; 3];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u16;
            arr[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            arr[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const FIELD_A_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const FIELD_A_BITS: usize = 24;
    #[allow(dead_code)]
    #[doc(hidden)]
    const FIELD_A_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::FIELD_A_BITS);
    #[allow(dead_code)]
    pub const fn field_a(self) -> u32 {
        ({
            const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 24 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u16 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 24)))
            }
        }) as u32
    }
    #[allow(dead_code)]
    pub fn set_field_a(&mut self, val: u32) {
        if true {
            if !((val as u128) <= Self::FIELD_A_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 24),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 24 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 24);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u16;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_field_a(mut self, val: u32) -> Self {
        if true {
            if !((val as u128) <= Self::FIELD_A_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 24 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 24);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u16;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_field_a(
        &mut self,
        val: u32,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::FIELD_A_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 24,
            });
        }
        self.set_field_a(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_field_a(
        self,
        val: u32,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::FIELD_A_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 24,
            });
        }
        Ok(self.with_field_a(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const FIELD_B_OFFSET: usize = 0 + 24;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const FIELD_B_BITS: usize = 24;
    #[allow(dead_code)]
    #[doc(hidden)]
    const FIELD_B_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::FIELD_B_BITS);
    #[allow(dead_code)]
    pub const fn field_b(self) -> u32 {
        ({
            const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 24) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 24) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 24) as usize + 24 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u16 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 24)))
            }
        }) as u32
    }
    #[allow(dead_code)]
    pub fn set_field_b(&mut self, val: u32) {
        if true {
            if !((val as u128) <= Self::FIELD_B_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 24),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 24) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 24) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 24) as usize + 24 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 24);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u16;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_field_b(mut self, val: u32) -> Self {
        if true {
            if !((val as u128) <= Self::FIELD_B_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 24) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 24) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 24) as usize + 24 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 24);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u16;
                        shift += <u16 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_field_b(
        &mut self,
        val: u32,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::FIELD_B_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 24,
            });
        }
        self.set_field_b(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_field_b(
        self,
        val: u32,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::FIELD_B_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 24,
            });
        }
        Ok(self.with_field_b(val))
    }
}

#[repr(C)]
pub struct U32Packet(pub [u32; 2]);

#[automatically_derived]
impl ::core::marker::Copy for U32Packet {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for U32Packet {}

#[automatically_derived]
impl ::core::clone::Clone for U32Packet {
    #[inline]
    fn clone(&self) -> U32Packet {
        let _: ::core::clone::AssertParamIsClone<[u32; 2]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for U32Packet {}

#[automatically_derived]
impl ::core::cmp::PartialEq for U32Packet {
    #[inline]
    fn eq(&self, other: &U32Packet) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for U32Packet {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u32; 2]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for U32Packet {
    #[inline]
    fn default() -> U32Packet {
        U32Packet(::core::default::Default::default())
    }
}

impl core::fmt::Debug for U32Packet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("U32Packet")
            .field("raw", &self.0)
            .field("header", &self.header())
            .field("body", &self.body())
            .finish()
    }
}

impl U32Packet {
    #[allow(dead_code)]
    pub const BITS: usize = <u32 as ::bitcraft::BitLength>::BITS_2;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u32; 2] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u32; 2]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u32 as ::bitcraft::BitLength>::BITS)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u32; 2];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u32;
            arr[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const HEADER_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const HEADER_BITS: usize = 8;
    #[allow(dead_code)]
    #[doc(hidden)]
    const HEADER_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::HEADER_BITS);
    #[allow(dead_code)]
    pub const fn header(self) -> u8 {
        ({
            const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 8 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u32 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 8)))
            }
        }) as u8
    }
    #[allow(dead_code)]
    pub fn set_header(&mut self, val: u8) {
        if true {
            if !((val as u128) <= Self::HEADER_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 8),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 8 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 8);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u32;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_header(mut self, val: u8) -> Self {
        if true {
            if !((val as u128) <= Self::HEADER_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 8 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 8);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u32;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_header(&mut self, val: u8) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::HEADER_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 8,
            });
        }
        self.set_header(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_header(
        self,
        val: u8,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::HEADER_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 8,
            });
        }
        Ok(self.with_header(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const BODY_OFFSET: usize = 0 + 8;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const BODY_BITS: usize = 56;
    #[allow(dead_code)]
    #[doc(hidden)]
    const BODY_MASK: <::bitcraft::Bits<
        { Self::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::BODY_BITS);
    #[allow(dead_code)]
    pub const fn body(self) -> u64 {
        ({
            const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 8) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 8) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 8) as usize + 56 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u32 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 56)))
            }
        }) as u64
    }
    #[allow(dead_code)]
    pub fn set_body(&mut self, val: u64) {
        if true {
            if !((val as u128) <= Self::BODY_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 56),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 8) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 8) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 8) as usize + 56 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 56);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u32;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_body(mut self, val: u64) -> Self {
        if true {
            if !((val as u128) <= Self::BODY_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 8) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 8) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 8) as usize + 56 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 56);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u32;
                        shift += <u32 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_body(&mut self, val: u64) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::BODY_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 56,
            });
        }
        self.set_body(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_body(
        self,
        val: u64,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::BODY_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 56,
            });
        }
        Ok(self.with_body(val))
    }
}

#[repr(transparent)]
pub struct SignedFieldsConfig(pub u64);

#[automatically_derived]
impl ::core::marker::Copy for SignedFieldsConfig {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedFieldsConfig {}

#[automatically_derived]
impl ::core::clone::Clone for SignedFieldsConfig {
    #[inline]
    fn clone(&self) -> SignedFieldsConfig {
        let _: ::core::clone::AssertParamIsClone<u64>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedFieldsConfig {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedFieldsConfig {
    #[inline]
    fn eq(&self, other: &SignedFieldsConfig) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedFieldsConfig {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u64>;
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedFieldsConfig {
    #[inline]
    fn default() -> SignedFieldsConfig {
        SignedFieldsConfig(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SignedFieldsConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SignedFieldsConfig")
            .field("raw", &self.0)
            .field("temperature", &self.temperature())
            .field("altitude", &self.altitude())
            .field("offset_x", &self.offset_x())
            .field("status", &self.status())
            .finish()
    }
}

impl SignedFieldsConfig {
    #[allow(dead_code)]
    pub const BITS: usize = <u64 as ::bitcraft::BitLength>::BITS;
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const TEMPERATURE_OFFSET: usize = 0;
    /// The number of bits allocated for the `$field_name` property.
    pub const TEMPERATURE_BITS: usize = 8;
    #[doc(hidden)]
    const TEMPERATURE_MASK: u64 = ((!0 as <u64 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u64 as ::bitcraft::BitLength>::BITS - Self::TEMPERATURE_BITS)) as u64;
    #[doc(hidden)]
    pub const TEMPERATURE_MIN: i8 = (!0 as i8) << (Self::TEMPERATURE_BITS - 1);
    #[doc(hidden)]
    pub const TEMPERATURE_MAX: i8 = !Self::TEMPERATURE_MIN;
    #[doc(hidden)]
    const TEMPERATURE_SHIFT_UP: usize = <i8 as ::bitcraft::BitLength>::BITS
        - Self::TEMPERATURE_BITS;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `temperature` property as a signed `i8`.
    pub const fn temperature(self) -> i8 {
        let raw = ((self.0 >> Self::TEMPERATURE_OFFSET) & Self::TEMPERATURE_MASK) as i8;
        (raw << Self::TEMPERATURE_SHIFT_UP) >> Self::TEMPERATURE_SHIFT_UP
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `temperature` signed property. Ensures bounds checking.
    pub fn set_temperature(&mut self, val: i8) {
        if true {
            if !(val >= Self::TEMPERATURE_MIN && val <= Self::TEMPERATURE_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} out of bounds for {1} bits", val, 8),
                    );
                }
            }
        }
        let val_masked = (val as u64) & Self::TEMPERATURE_MASK;
        self.0 = (self.0 & !(Self::TEMPERATURE_MASK << Self::TEMPERATURE_OFFSET))
            | (val_masked << Self::TEMPERATURE_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `temperature` signed property mapped.
    pub const fn with_temperature(self, val: i8) -> Self {
        if true {
            if !(val >= Self::TEMPERATURE_MIN && val <= Self::TEMPERATURE_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u64) & Self::TEMPERATURE_MASK;
        Self(
            (self.0 & !(Self::TEMPERATURE_MASK << Self::TEMPERATURE_OFFSET))
                | (val_masked << Self::TEMPERATURE_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `temperature` signed property. Returns a `BitstructError` if out of bounds.
    pub fn try_set_temperature(
        &mut self,
        val: i8,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if val < Self::TEMPERATURE_MIN || val > Self::TEMPERATURE_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 8,
            });
        }
        let val_masked = (val as u64) & Self::TEMPERATURE_MASK;
        self.0 = (self.0 & !(Self::TEMPERATURE_MASK << Self::TEMPERATURE_OFFSET))
            | (val_masked << Self::TEMPERATURE_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `temperature` signed property. Returns a `BitstructError` if out of bounds.
    pub const fn try_with_temperature(
        self,
        val: i8,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if val < Self::TEMPERATURE_MIN || val > Self::TEMPERATURE_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 8,
            });
        }
        let val_masked = (val as u64) & Self::TEMPERATURE_MASK;
        Ok(
            Self(
                (self.0 & !(Self::TEMPERATURE_MASK << Self::TEMPERATURE_OFFSET))
                    | (val_masked << Self::TEMPERATURE_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const ALTITUDE_OFFSET: usize = 0 + 8;
    /// The number of bits allocated for the `$field_name` property.
    pub const ALTITUDE_BITS: usize = 16;
    #[doc(hidden)]
    const ALTITUDE_MASK: u64 = ((!0 as <u64 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u64 as ::bitcraft::BitLength>::BITS - Self::ALTITUDE_BITS)) as u64;
    #[doc(hidden)]
    pub const ALTITUDE_MIN: i16 = (!0 as i16) << (Self::ALTITUDE_BITS - 1);
    #[doc(hidden)]
    pub const ALTITUDE_MAX: i16 = !Self::ALTITUDE_MIN;
    #[doc(hidden)]
    const ALTITUDE_SHIFT_UP: usize = <i16 as ::bitcraft::BitLength>::BITS
        - Self::ALTITUDE_BITS;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `altitude` property as a signed `i16`.
    pub const fn altitude(self) -> i16 {
        let raw = ((self.0 >> Self::ALTITUDE_OFFSET) & Self::ALTITUDE_MASK) as i16;
        (raw << Self::ALTITUDE_SHIFT_UP) >> Self::ALTITUDE_SHIFT_UP
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `altitude` signed property. Ensures bounds checking.
    pub fn set_altitude(&mut self, val: i16) {
        if true {
            if !(val >= Self::ALTITUDE_MIN && val <= Self::ALTITUDE_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} out of bounds for {1} bits", val, 16),
                    );
                }
            }
        }
        let val_masked = (val as u64) & Self::ALTITUDE_MASK;
        self.0 = (self.0 & !(Self::ALTITUDE_MASK << Self::ALTITUDE_OFFSET))
            | (val_masked << Self::ALTITUDE_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `altitude` signed property mapped.
    pub const fn with_altitude(self, val: i16) -> Self {
        if true {
            if !(val >= Self::ALTITUDE_MIN && val <= Self::ALTITUDE_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u64) & Self::ALTITUDE_MASK;
        Self(
            (self.0 & !(Self::ALTITUDE_MASK << Self::ALTITUDE_OFFSET))
                | (val_masked << Self::ALTITUDE_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `altitude` signed property. Returns a `BitstructError` if out of bounds.
    pub fn try_set_altitude(
        &mut self,
        val: i16,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if val < Self::ALTITUDE_MIN || val > Self::ALTITUDE_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 16,
            });
        }
        let val_masked = (val as u64) & Self::ALTITUDE_MASK;
        self.0 = (self.0 & !(Self::ALTITUDE_MASK << Self::ALTITUDE_OFFSET))
            | (val_masked << Self::ALTITUDE_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `altitude` signed property. Returns a `BitstructError` if out of bounds.
    pub const fn try_with_altitude(
        self,
        val: i16,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if val < Self::ALTITUDE_MIN || val > Self::ALTITUDE_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 16,
            });
        }
        let val_masked = (val as u64) & Self::ALTITUDE_MASK;
        Ok(
            Self(
                (self.0 & !(Self::ALTITUDE_MASK << Self::ALTITUDE_OFFSET))
                    | (val_masked << Self::ALTITUDE_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const OFFSET_X_OFFSET: usize = 0 + 8 + 16;
    /// The number of bits allocated for the `$field_name` property.
    pub const OFFSET_X_BITS: usize = 20;
    #[doc(hidden)]
    const OFFSET_X_MASK: u64 = ((!0 as <u64 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u64 as ::bitcraft::BitLength>::BITS - Self::OFFSET_X_BITS)) as u64;
    #[doc(hidden)]
    pub const OFFSET_X_MIN: i32 = (!0 as i32) << (Self::OFFSET_X_BITS - 1);
    #[doc(hidden)]
    pub const OFFSET_X_MAX: i32 = !Self::OFFSET_X_MIN;
    #[doc(hidden)]
    const OFFSET_X_SHIFT_UP: usize = <i32 as ::bitcraft::BitLength>::BITS
        - Self::OFFSET_X_BITS;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `offset_x` property as a signed `i32`.
    pub const fn offset_x(self) -> i32 {
        let raw = ((self.0 >> Self::OFFSET_X_OFFSET) & Self::OFFSET_X_MASK) as i32;
        (raw << Self::OFFSET_X_SHIFT_UP) >> Self::OFFSET_X_SHIFT_UP
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `offset_x` signed property. Ensures bounds checking.
    pub fn set_offset_x(&mut self, val: i32) {
        if true {
            if !(val >= Self::OFFSET_X_MIN && val <= Self::OFFSET_X_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} out of bounds for {1} bits", val, 20),
                    );
                }
            }
        }
        let val_masked = (val as u64) & Self::OFFSET_X_MASK;
        self.0 = (self.0 & !(Self::OFFSET_X_MASK << Self::OFFSET_X_OFFSET))
            | (val_masked << Self::OFFSET_X_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `offset_x` signed property mapped.
    pub const fn with_offset_x(self, val: i32) -> Self {
        if true {
            if !(val >= Self::OFFSET_X_MIN && val <= Self::OFFSET_X_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u64) & Self::OFFSET_X_MASK;
        Self(
            (self.0 & !(Self::OFFSET_X_MASK << Self::OFFSET_X_OFFSET))
                | (val_masked << Self::OFFSET_X_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `offset_x` signed property. Returns a `BitstructError` if out of bounds.
    pub fn try_set_offset_x(
        &mut self,
        val: i32,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if val < Self::OFFSET_X_MIN || val > Self::OFFSET_X_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 20,
            });
        }
        let val_masked = (val as u64) & Self::OFFSET_X_MASK;
        self.0 = (self.0 & !(Self::OFFSET_X_MASK << Self::OFFSET_X_OFFSET))
            | (val_masked << Self::OFFSET_X_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `offset_x` signed property. Returns a `BitstructError` if out of bounds.
    pub const fn try_with_offset_x(
        self,
        val: i32,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if val < Self::OFFSET_X_MIN || val > Self::OFFSET_X_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 20,
            });
        }
        let val_masked = (val as u64) & Self::OFFSET_X_MASK;
        Ok(
            Self(
                (self.0 & !(Self::OFFSET_X_MASK << Self::OFFSET_X_OFFSET))
                    | (val_masked << Self::OFFSET_X_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const STATUS_OFFSET: usize = 0 + 8 + 16 + 20;
    /// The number of bits allocated for the `$field_name` property.
    pub const STATUS_BITS: usize = 2;
    #[doc(hidden)]
    const STATUS_MASK: u64 = ((!0 as <u64 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u64 as ::bitcraft::BitLength>::BITS - Self::STATUS_BITS)) as u64;
    #[allow(dead_code)]
    ///Returns the `status` variant strictly typed to the `SignedStatus` enumeration.
    pub const fn status(self) -> SignedStatus {
        SignedStatus::from_bits(
            ((self.0 >> Self::STATUS_OFFSET) & Self::STATUS_MASK) as _,
        )
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the bounded `SignedStatus` enumeration to the `status` property.
    pub fn set_status(&mut self, val: SignedStatus) {
        const _: () = if !(<SignedStatus>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as u64) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield bounded by the `SignedStatus` enumeration supplied to `status`.
    pub const fn with_status(self, val: SignedStatus) -> Self {
        const _: () = if !(<SignedStatus>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as u64) & Self::STATUS_MASK;
        Self(
            (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
                | (val_masked << Self::STATUS_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the bounded `SignedStatus` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub fn try_set_status(
        &mut self,
        val: SignedStatus,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_status(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the bounded `SignedStatus` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub const fn try_with_status(
        self,
        val: SignedStatus,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_status(val))
    }
    /// Returns the raw interior integer value.
    ///
    /// This is useful for serializing the struct or passing it to external APIs.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u64 {
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
    pub const fn from_bits(val: u64) -> Self {
        Self(val)
    }
}

#[repr(C)]
pub struct SignedCoordinate(pub [u8; 4]);

#[automatically_derived]
impl ::core::marker::Copy for SignedCoordinate {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedCoordinate {}

#[automatically_derived]
impl ::core::clone::Clone for SignedCoordinate {
    #[inline]
    fn clone(&self) -> SignedCoordinate {
        let _: ::core::clone::AssertParamIsClone<[u8; 4]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedCoordinate {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedCoordinate {
    #[inline]
    fn eq(&self, other: &SignedCoordinate) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedCoordinate {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 4]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedCoordinate {
    #[inline]
    fn default() -> SignedCoordinate {
        SignedCoordinate(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SignedCoordinate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SignedCoordinate")
            .field("raw", &self.0)
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

impl SignedCoordinate {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_4;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 4] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 4]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 4];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            arr[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self.to_bits() as u32
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const X_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const X_BITS: usize = 16;
    #[allow(dead_code)]
    #[doc(hidden)]
    const X_MASK: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::X_BITS);
    #[allow(dead_code)]
    #[doc(hidden)]
    pub const X_MIN: i16 = (!0 as i16) << (Self::X_BITS - 1);
    #[allow(dead_code)]
    #[doc(hidden)]
    pub const X_MAX: i16 = !Self::X_MIN;
    #[allow(dead_code)]
    #[doc(hidden)]
    const X_SHIFT_UP: usize = <i16 as ::bitcraft::BitLength>::BITS - Self::X_BITS;
    #[allow(dead_code)]
    pub const fn x(self) -> i16 {
        let raw = {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 16 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 16)))
            }
        } as i16;
        (raw << Self::X_SHIFT_UP) >> Self::X_SHIFT_UP
    }
    #[allow(dead_code)]
    pub fn set_x(&mut self, val: i16) {
        if true {
            if !(val >= Self::X_MIN && val <= Self::X_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} out of bounds for {1} bits", val, 16),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 16 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_x(mut self, val: i16) -> Self {
        if true {
            if !(val >= Self::X_MIN && val <= Self::X_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = 0 / UNIT_BITS;
            const BIT_OFFSET: usize = 0 % UNIT_BITS;
            const WORD_COUNT: usize = (0 as usize + 16 as usize).div_ceil(UNIT_BITS)
                - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_x(&mut self, val: i16) -> Result<(), ::bitcraft::BitstructError> {
        if val < Self::X_MIN || val > Self::X_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 16,
            });
        }
        self.set_x(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_x(self, val: i16) -> Result<Self, ::bitcraft::BitstructError> {
        if val < Self::X_MIN || val > Self::X_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 16,
            });
        }
        Ok(self.with_x(val))
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const Y_OFFSET: usize = 0 + 16;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property.
    pub const Y_BITS: usize = 16;
    #[allow(dead_code)]
    #[doc(hidden)]
    const Y_MASK: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { Self::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::Y_BITS);
    #[allow(dead_code)]
    #[doc(hidden)]
    pub const Y_MIN: i16 = (!0 as i16) << (Self::Y_BITS - 1);
    #[allow(dead_code)]
    #[doc(hidden)]
    pub const Y_MAX: i16 = !Self::Y_MIN;
    #[allow(dead_code)]
    #[doc(hidden)]
    const Y_SHIFT_UP: usize = <i16 as ::bitcraft::BitLength>::BITS - Self::Y_BITS;
    #[allow(dead_code)]
    pub const fn y(self) -> i16 {
        let raw = {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16) as usize + 16 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut i = 0;
                let mut acc = 0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim;
                let mut shift = 0;
                while i < WORD_COUNT {
                    acc
                        |= (self.0[WORD_INDEX + i]
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim) << shift;
                    shift += <u8 as ::bitcraft::BitLength>::BITS;
                    i += 1;
                }
                ((acc >> BIT_OFFSET)
                    & ((!0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim)
                        >> (<<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                            - 16)))
            }
        } as i16;
        (raw << Self::Y_SHIFT_UP) >> Self::Y_SHIFT_UP
    }
    #[allow(dead_code)]
    pub fn set_y(&mut self, val: i16) {
        if true {
            if !(val >= Self::Y_MIN && val <= Self::Y_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} out of bounds for {1} bits", val, 16),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16) as usize + 16 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
    }
    #[allow(dead_code)]
    pub const fn with_y(mut self, val: i16) -> Self {
        if true {
            if !(val >= Self::Y_MIN && val <= Self::Y_MAX) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
            const WORD_INDEX: usize = (0 + 16) / UNIT_BITS;
            const BIT_OFFSET: usize = (0 + 16) % UNIT_BITS;
            const WORD_COUNT: usize = ((0 + 16) as usize + 16 as usize)
                .div_ceil(UNIT_BITS) - WORD_INDEX;
            {
                let mut full = {
                    let mut i = 0;
                    let mut acc = 0
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        acc
                            |= (self.0[WORD_INDEX + i]
                                as <::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim) << shift;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                    ((acc >> 0)
                        & ((!0
                            as <::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim)
                            >> (<<::bitcraft::Bits<
                                { Self::BITS },
                            > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                                - <<::bitcraft::Bits<
                                    { Self::BITS },
                                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)))
                };
                let mask = (!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - 16);
                full = (full & !(mask << BIT_OFFSET))
                    | ((val
                        as <::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim & mask) << BIT_OFFSET);
                {
                    let mut i = 0;
                    let mut shift = 0;
                    while i < WORD_COUNT {
                        self.0[WORD_INDEX + i] = (full >> shift) as u8;
                        shift += <u8 as ::bitcraft::BitLength>::BITS;
                        i += 1;
                    }
                };
            }
        };
        self
    }
    #[allow(dead_code)]
    pub fn try_set_y(&mut self, val: i16) -> Result<(), ::bitcraft::BitstructError> {
        if val < Self::Y_MIN || val > Self::Y_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 16,
            });
        }
        self.set_y(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_y(self, val: i16) -> Result<Self, ::bitcraft::BitstructError> {
        if val < Self::Y_MIN || val > Self::Y_MAX {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as i128 as u128,
                allocated_bits: 16,
            });
        }
        Ok(self.with_y(val))
    }
}

#[repr(C)]
pub struct SmallId(pub [u8; 1]);

#[automatically_derived]
impl ::core::marker::Copy for SmallId {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SmallId {}

#[automatically_derived]
impl ::core::clone::Clone for SmallId {
    #[inline]
    fn clone(&self) -> SmallId {
        let _: ::core::clone::AssertParamIsClone<[u8; 1]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SmallId {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SmallId {
    #[inline]
    fn eq(&self, other: &SmallId) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SmallId {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 1]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for SmallId {
    #[inline]
    fn default() -> SmallId {
        SmallId(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SmallId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SmallId")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl SmallId {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 1] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 1]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        ((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 1];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self.to_bits() as u8
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim {
        ((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = ((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = ((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct Id16(pub [u8; 2]);

#[automatically_derived]
impl ::core::marker::Copy for Id16 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Id16 {}

#[automatically_derived]
impl ::core::clone::Clone for Id16 {
    #[inline]
    fn clone(&self) -> Id16 {
        let _: ::core::clone::AssertParamIsClone<[u8; 2]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Id16 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Id16 {
    #[inline]
    fn eq(&self, other: &Id16) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Id16 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 2]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Id16 {
    #[inline]
    fn default() -> Id16 {
        Id16(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Id16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Id16")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl Id16 {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_2;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 2] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 2]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 2];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u16(self) -> u16 {
        self.to_bits() as u16
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u16(val: u16) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_2;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_2 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_2 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct PackedId(pub [u8; 3]);

#[automatically_derived]
impl ::core::marker::Copy for PackedId {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for PackedId {}

#[automatically_derived]
impl ::core::clone::Clone for PackedId {
    #[inline]
    fn clone(&self) -> PackedId {
        let _: ::core::clone::AssertParamIsClone<[u8; 3]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for PackedId {}

#[automatically_derived]
impl ::core::cmp::PartialEq for PackedId {
    #[inline]
    fn eq(&self, other: &PackedId) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for PackedId {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 3]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for PackedId {
    #[inline]
    fn default() -> PackedId {
        PackedId(::core::default::Default::default())
    }
}

impl core::fmt::Debug for PackedId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PackedId")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl PackedId {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 3] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 3]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 3];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self.to_bits() as u32
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct Id40(pub [u8; 5]);

#[automatically_derived]
impl ::core::marker::Copy for Id40 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Id40 {}

#[automatically_derived]
impl ::core::clone::Clone for Id40 {
    #[inline]
    fn clone(&self) -> Id40 {
        let _: ::core::clone::AssertParamIsClone<[u8; 5]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Id40 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Id40 {
    #[inline]
    fn eq(&self, other: &Id40) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Id40 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 5]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Id40 {
    #[inline]
    fn default() -> Id40 {
        Id40(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Id40 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Id40")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl Id40 {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_5;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 5] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 5]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)
            | ((self.0[0 + 4]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_4)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 5];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            arr[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            arr[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_5;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_5 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_5 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_5 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_5 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)
            | ((self.0[0 + 4]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_4)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_5 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)
                | ((self.0[0 + 3]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_3)
                | ((self.0[0 + 4]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_4)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            self.0[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            self.0[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_5 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)
                | ((self.0[0 + 3]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_3)
                | ((self.0[0 + 4]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_4)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            self.0[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            self.0[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_5 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_5 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct Id48(pub [u16; 3]);

#[automatically_derived]
impl ::core::marker::Copy for Id48 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Id48 {}

#[automatically_derived]
impl ::core::clone::Clone for Id48 {
    #[inline]
    fn clone(&self) -> Id48 {
        let _: ::core::clone::AssertParamIsClone<[u16; 3]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Id48 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Id48 {
    #[inline]
    fn eq(&self, other: &Id48) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Id48 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u16; 3]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Id48 {
    #[inline]
    fn default() -> Id48 {
        Id48(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Id48 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Id48")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl Id48 {
    #[allow(dead_code)]
    pub const BITS: usize = <u16 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u16; 3] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u16; 3]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u16; 3];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u16;
            arr[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            arr[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u16 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u16 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u16;
            self.0[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            self.0[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u16;
            self.0[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            self.0[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct SignedId24(pub [u8; 3]);

#[automatically_derived]
impl ::core::marker::Copy for SignedId24 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedId24 {}

#[automatically_derived]
impl ::core::clone::Clone for SignedId24 {
    #[inline]
    fn clone(&self) -> SignedId24 {
        let _: ::core::clone::AssertParamIsClone<[u8; 3]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedId24 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedId24 {
    #[inline]
    fn eq(&self, other: &SignedId24) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedId24 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 3]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedId24 {
    #[inline]
    fn default() -> SignedId24 {
        SignedId24(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SignedId24 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SignedId24")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl SignedId24 {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 3] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 3]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 3];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self.to_bits() as u32
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    pub const VALUE_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::SignedBitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_SHIFT_UP: usize = <<::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::SignedBitenumType>::Prim as ::bitcraft::BitLength>::BITS
        - Self::VALUE_BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::SignedBitenumType>::Prim {
        let val = (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS)));
        let mut signed_val = val
            as <::bitcraft::Bits<
                { <u8 as ::bitcraft::BitLength>::BITS_3 },
            > as ::bitcraft::SignedBitenumType>::Prim;
        signed_val = (signed_val << Self::VALUE_SHIFT_UP) >> Self::VALUE_SHIFT_UP;
        signed_val
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) {
        if true {
            if !((val
                >= ((!0
                    as <::bitcraft::Bits<
                        { <u8 as ::bitcraft::BitLength>::BITS_3 },
                    > as ::bitcraft::SignedBitenumType>::Prim)
                    << (Self::VALUE_BITS - 1)))
                && (val
                    <= !((!0
                        as <::bitcraft::Bits<
                            { <u8 as ::bitcraft::BitLength>::BITS_3 },
                        > as ::bitcraft::SignedBitenumType>::Prim)
                        << (Self::VALUE_BITS - 1))))
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let unsigned_val = val
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim;
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0)) | ((unsigned_val & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val
                >= ((!0
                    as <::bitcraft::Bits<
                        { <u8 as ::bitcraft::BitLength>::BITS_3 },
                    > as ::bitcraft::SignedBitenumType>::Prim)
                    << (Self::VALUE_BITS - 1)))
                && (val
                    <= !((!0
                        as <::bitcraft::Bits<
                            { <u8 as ::bitcraft::BitLength>::BITS_3 },
                        > as ::bitcraft::SignedBitenumType>::Prim)
                        << (Self::VALUE_BITS - 1))))
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let unsigned_val = val
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim;
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0)) | ((unsigned_val & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        let min = (!0
            as <::bitcraft::Bits<
                { <u8 as ::bitcraft::BitLength>::BITS_3 },
            > as ::bitcraft::SignedBitenumType>::Prim) << (Self::VALUE_BITS - 1);
        let max = !min;
        if val < min || val > max {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i128) as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        let min = (!0
            as <::bitcraft::Bits<
                { <u8 as ::bitcraft::BitLength>::BITS_3 },
            > as ::bitcraft::SignedBitenumType>::Prim) << (Self::VALUE_BITS - 1);
        let max = !min;
        if val < min || val > max {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i128) as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct SignedId48(pub [u16; 3]);

#[automatically_derived]
impl ::core::marker::Copy for SignedId48 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedId48 {}

#[automatically_derived]
impl ::core::clone::Clone for SignedId48 {
    #[inline]
    fn clone(&self) -> SignedId48 {
        let _: ::core::clone::AssertParamIsClone<[u16; 3]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedId48 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedId48 {
    #[inline]
    fn eq(&self, other: &SignedId48) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedId48 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u16; 3]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedId48 {
    #[inline]
    fn default() -> SignedId48 {
        SignedId48(::core::default::Default::default())
    }
}

impl core::fmt::Debug for SignedId48 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SignedId48")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl SignedId48 {
    #[allow(dead_code)]
    pub const BITS: usize = <u16 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u16; 3] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u16; 3]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u16; 3];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u16;
            arr[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            arr[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    pub const VALUE_BITS: usize = <u16 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u16 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::SignedBitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_SHIFT_UP: usize = <<::bitcraft::Bits<
        { <u16 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::SignedBitenumType>::Prim as ::bitcraft::BitLength>::BITS
        - Self::VALUE_BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u16 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::SignedBitenumType>::Prim {
        let val = (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS)));
        let mut signed_val = val
            as <::bitcraft::Bits<
                { <u16 as ::bitcraft::BitLength>::BITS_3 },
            > as ::bitcraft::SignedBitenumType>::Prim;
        signed_val = (signed_val << Self::VALUE_SHIFT_UP) >> Self::VALUE_SHIFT_UP;
        signed_val
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) {
        if true {
            if !((val
                >= ((!0
                    as <::bitcraft::Bits<
                        { <u16 as ::bitcraft::BitLength>::BITS_3 },
                    > as ::bitcraft::SignedBitenumType>::Prim)
                    << (Self::VALUE_BITS - 1)))
                && (val
                    <= !((!0
                        as <::bitcraft::Bits<
                            { <u16 as ::bitcraft::BitLength>::BITS_3 },
                        > as ::bitcraft::SignedBitenumType>::Prim)
                        << (Self::VALUE_BITS - 1))))
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let unsigned_val = val
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim;
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0)) | ((unsigned_val & mask) << 0);
            self.0[0] = full as u16;
            self.0[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            self.0[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val
                >= ((!0
                    as <::bitcraft::Bits<
                        { <u16 as ::bitcraft::BitLength>::BITS_3 },
                    > as ::bitcraft::SignedBitenumType>::Prim)
                    << (Self::VALUE_BITS - 1)))
                && (val
                    <= !((!0
                        as <::bitcraft::Bits<
                            { <u16 as ::bitcraft::BitLength>::BITS_3 },
                        > as ::bitcraft::SignedBitenumType>::Prim)
                        << (Self::VALUE_BITS - 1))))
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let unsigned_val = val
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim;
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u16 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0)) | ((unsigned_val & mask) << 0);
            self.0[0] = full as u16;
            self.0[0 + 1] = (full >> <u16 as ::bitcraft::BitLength>::BITS) as u16;
            self.0[0 + 2] = (full >> <u16 as ::bitcraft::BitLength>::BITS_2) as u16;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        let min = (!0
            as <::bitcraft::Bits<
                { <u16 as ::bitcraft::BitLength>::BITS_3 },
            > as ::bitcraft::SignedBitenumType>::Prim) << (Self::VALUE_BITS - 1);
        let max = !min;
        if val < min || val > max {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i128) as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u16 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::SignedBitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        let min = (!0
            as <::bitcraft::Bits<
                { <u16 as ::bitcraft::BitLength>::BITS_3 },
            > as ::bitcraft::SignedBitenumType>::Prim) << (Self::VALUE_BITS - 1);
        let max = !min;
        if val < min || val > max {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as i128) as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct Id64(pub [u8; 8]);

#[automatically_derived]
impl ::core::marker::Copy for Id64 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Id64 {}

#[automatically_derived]
impl ::core::clone::Clone for Id64 {
    #[inline]
    fn clone(&self) -> Id64 {
        let _: ::core::clone::AssertParamIsClone<[u8; 8]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Id64 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Id64 {
    #[inline]
    fn eq(&self, other: &Id64) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Id64 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 8]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Id64 {
    #[inline]
    fn default() -> Id64 {
        Id64(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Id64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Id64")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl Id64 {
    #[allow(dead_code)]
    pub const BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_8;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 8] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u8; 8]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)
            | ((self.0[0 + 4]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_4)
            | ((self.0[0 + 5]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_5)
            | ((self.0[0 + 6]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_6)
            | ((self.0[0 + 7]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_7)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u8; 8];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u8;
            arr[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            arr[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            arr[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            arr[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
            arr[0 + 5] = (full >> <u8 as ::bitcraft::BitLength>::BITS_5) as u8;
            arr[0 + 6] = (full >> <u8 as ::bitcraft::BitLength>::BITS_6) as u8;
            arr[0 + 7] = (full >> <u8 as ::bitcraft::BitLength>::BITS_7) as u8;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.to_bits() as u64
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u64(val: u64) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u8 as ::bitcraft::BitLength>::BITS_8;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_8 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_8 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_8 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u8 as ::bitcraft::BitLength>::BITS_8 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_2)
            | ((self.0[0 + 3]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_3)
            | ((self.0[0 + 4]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_4)
            | ((self.0[0 + 5]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_5)
            | ((self.0[0 + 6]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_6)
            | ((self.0[0 + 7]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u8 as ::bitcraft::BitLength>::BITS_7)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_8 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)
                | ((self.0[0 + 3]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_3)
                | ((self.0[0 + 4]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_4)
                | ((self.0[0 + 5]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_5)
                | ((self.0[0 + 6]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_6)
                | ((self.0[0 + 7]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_7)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            self.0[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            self.0[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
            self.0[0 + 5] = (full >> <u8 as ::bitcraft::BitLength>::BITS_5) as u8;
            self.0[0 + 6] = (full >> <u8 as ::bitcraft::BitLength>::BITS_6) as u8;
            self.0[0 + 7] = (full >> <u8 as ::bitcraft::BitLength>::BITS_7) as u8;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_8 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_2)
                | ((self.0[0 + 3]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_3)
                | ((self.0[0 + 4]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_4)
                | ((self.0[0 + 5]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_5)
                | ((self.0[0 + 6]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_6)
                | ((self.0[0 + 7]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u8 as ::bitcraft::BitLength>::BITS_7)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u8;
            self.0[0 + 1] = (full >> <u8 as ::bitcraft::BitLength>::BITS) as u8;
            self.0[0 + 2] = (full >> <u8 as ::bitcraft::BitLength>::BITS_2) as u8;
            self.0[0 + 3] = (full >> <u8 as ::bitcraft::BitLength>::BITS_3) as u8;
            self.0[0 + 4] = (full >> <u8 as ::bitcraft::BitLength>::BITS_4) as u8;
            self.0[0 + 5] = (full >> <u8 as ::bitcraft::BitLength>::BITS_5) as u8;
            self.0[0 + 6] = (full >> <u8 as ::bitcraft::BitLength>::BITS_6) as u8;
            self.0[0 + 7] = (full >> <u8 as ::bitcraft::BitLength>::BITS_7) as u8;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_8 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u8 as ::bitcraft::BitLength>::BITS_8 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(align(4))]
#[repr(C)]
pub struct AlignedId96(pub [u32; 3]);

#[automatically_derived]
impl ::core::marker::Copy for AlignedId96 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AlignedId96 {}

#[automatically_derived]
impl ::core::clone::Clone for AlignedId96 {
    #[inline]
    fn clone(&self) -> AlignedId96 {
        let _: ::core::clone::AssertParamIsClone<[u32; 3]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AlignedId96 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for AlignedId96 {
    #[inline]
    fn eq(&self, other: &AlignedId96) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for AlignedId96 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u32; 3]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for AlignedId96 {
    #[inline]
    fn default() -> AlignedId96 {
        AlignedId96(::core::default::Default::default())
    }
}

impl core::fmt::Debug for AlignedId96 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AlignedId96")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl AlignedId96 {
    #[allow(dead_code)]
    pub const BITS: usize = <u32 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u32; 3] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u32; 3]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u32 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u32 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u32; 3];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u32;
            arr[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
            arr[0 + 2] = (full >> <u32 as ::bitcraft::BitLength>::BITS_2) as u32;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        self.to_bits() as u128
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u128(val: u128) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u32 as ::bitcraft::BitLength>::BITS_3;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u32 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u32 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u32 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u32 as ::bitcraft::BitLength>::BITS_3 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u32 as ::bitcraft::BitLength>::BITS)
            | ((self.0[0 + 2]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u32 as ::bitcraft::BitLength>::BITS_2)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u32 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u32 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u32 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u32;
            self.0[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
            self.0[0 + 2] = (full >> <u32 as ::bitcraft::BitLength>::BITS_2) as u32;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u32 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u32 as ::bitcraft::BitLength>::BITS)
                | ((self.0[0 + 2]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u32 as ::bitcraft::BitLength>::BITS_2)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u32;
            self.0[0 + 1] = (full >> <u32 as ::bitcraft::BitLength>::BITS) as u32;
            self.0[0 + 2] = (full >> <u32 as ::bitcraft::BitLength>::BITS_2) as u32;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u32 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u32 as ::bitcraft::BitLength>::BITS_3 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct Id128(pub [u128; 1]);

#[automatically_derived]
impl ::core::marker::Copy for Id128 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Id128 {}

#[automatically_derived]
impl ::core::clone::Clone for Id128 {
    #[inline]
    fn clone(&self) -> Id128 {
        let _: ::core::clone::AssertParamIsClone<[u128; 1]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Id128 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for Id128 {
    #[inline]
    fn eq(&self, other: &Id128) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for Id128 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u128; 1]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for Id128 {
    #[inline]
    fn default() -> Id128 {
        Id128(::core::default::Default::default())
    }
}

impl core::fmt::Debug for Id128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Id128")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl Id128 {
    #[allow(dead_code)]
    pub const BITS: usize = <u128 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u128 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u128; 1] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u128; 1]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        ((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u128; 1];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u128;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        self.to_bits() as u128
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u128(val: u128) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u128 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u128 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u128 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u128 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u128 as ::bitcraft::BitLength>::BITS },
    > as ::bitcraft::BitenumType>::Prim {
        ((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u128 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = ((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u128;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u128 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = ((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u128;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u128 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u128 as ::bitcraft::BitLength>::BITS },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(C)]
pub struct DualId128(pub [u64; 2]);

#[automatically_derived]
impl ::core::marker::Copy for DualId128 {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for DualId128 {}

#[automatically_derived]
impl ::core::clone::Clone for DualId128 {
    #[inline]
    fn clone(&self) -> DualId128 {
        let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for DualId128 {}

#[automatically_derived]
impl ::core::cmp::PartialEq for DualId128 {
    #[inline]
    fn eq(&self, other: &DualId128) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for DualId128 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u64; 2]>;
    }
}

#[automatically_derived]
impl ::core::default::Default for DualId128 {
    #[inline]
    fn default() -> DualId128 {
        DualId128(::core::default::Default::default())
    }
}

impl core::fmt::Debug for DualId128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DualId128")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl DualId128 {
    #[allow(dead_code)]
    pub const BITS: usize = <u64 as ::bitcraft::BitLength>::BITS_2;
    #[allow(dead_code)]
    pub const UNIT_BITS: usize = <u64 as ::bitcraft::BitLength>::BITS;
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_array(self) -> [u64; 2] {
        self.0
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_array(arr: [u64; 2]) -> Self {
        Self(arr)
    }
    /// Converts the packed structure into its raw bit representation as the narrowest possible
    /// primitive integer (u32, u64, or u128) that can hold all bits.
    ///
    /// This method utilizes the fully-unrolled bitwise engine for maximum register efficiency.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u64 as ::bitcraft::BitLength>::BITS)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS))) as _
    }
    /// Reconstructs the packed structure from a raw bit representation.
    ///
    /// This uses a specialized 'fresh write' path that initializes the underlying array
    /// in a single unrolled pass, avoiding redundant masking operations on empty storage.
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        let mut arr = [0 as u64; 2];
        {
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::BITS);
            let full = (val & mask) << 0;
            arr[0] = full as u64;
            arr[0 + 1] = (full >> <u64 as ::bitcraft::BitLength>::BITS) as u64;
        };
        Self(arr)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        self.to_bits() as u128
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn from_u128(val: u128) -> Self {
        Self::from_bits(val as _)
    }
    #[allow(dead_code)]
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const VALUE_OFFSET: usize = 0;
    #[allow(dead_code)]
    /// The number of bits allocated for the `$field_name` property (unrolled).
    pub const VALUE_BITS: usize = <u64 as ::bitcraft::BitLength>::BITS_2;
    #[allow(dead_code)]
    #[doc(hidden)]
    const VALUE_MASK: <::bitcraft::Bits<
        { <u64 as ::bitcraft::BitLength>::BITS_2 },
    > as ::bitcraft::BitenumType>::Prim = (!0
        as <::bitcraft::Bits<
            { <u64 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim)
        >> (<<::bitcraft::Bits<
            { <u64 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
            - Self::VALUE_BITS);
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn value(
        self,
    ) -> <::bitcraft::Bits<
        { <u64 as ::bitcraft::BitLength>::BITS_2 },
    > as ::bitcraft::BitenumType>::Prim {
        (((self.0[0]
            as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
            | ((self.0[0 + 1]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                << <u64 as ::bitcraft::BitLength>::BITS)) >> 0)
            & ((!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS))) as _
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u64 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Value {0} overflows allocated {1} bits",
                            val,
                            Self::VALUE_BITS,
                        ),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u64 as ::bitcraft::BitLength>::BITS)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u64;
            self.0[0 + 1] = (full >> <u64 as ::bitcraft::BitLength>::BITS) as u64;
        };
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn with_value(
        mut self,
        val: <::bitcraft::Bits<
            { <u64 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK as u128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            let mut full = (((self.0[0]
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim
                | ((self.0[0 + 1]
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    << <u64 as ::bitcraft::BitLength>::BITS)) >> 0)
                & ((!0
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim)
                    >> (<<::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                        - <<::bitcraft::Bits<
                            { Self::BITS },
                        > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS)));
            let mask = (!0
                as <::bitcraft::Bits<{ Self::BITS }> as ::bitcraft::BitenumType>::Prim)
                >> (<<::bitcraft::Bits<
                    { Self::BITS },
                > as ::bitcraft::BitenumType>::Prim as ::bitcraft::BitLength>::BITS
                    - Self::VALUE_BITS);
            full = (full & !(mask << 0))
                | ((val
                    as <::bitcraft::Bits<
                        { Self::BITS },
                    > as ::bitcraft::BitenumType>::Prim & mask) << 0);
            self.0[0] = full as u64;
            self.0[0 + 1] = (full >> <u64 as ::bitcraft::BitLength>::BITS) as u64;
        };
        self
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_set_value(
        &mut self,
        val: <::bitcraft::Bits<
            { <u64 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn try_with_value(
        self,
        val: <::bitcraft::Bits<
            { <u64 as ::bitcraft::BitLength>::BITS_2 },
        > as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: Self::VALUE_BITS,
            });
        }
        Ok(self.with_value(val))
    }
}

#[repr(transparent)]
pub struct AtomicPoolTracker(pub ::bitcraft::reexport::portable_atomic::AtomicU32);

impl core::fmt::Debug for AtomicPoolTracker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AtomicPoolTracker")
            .field(
                "raw",
                &self.0.load(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .field(
                "is_active",
                &self.is_active(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .field(
                "active_connections",
                &self
                    .active_connections(
                        ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                    ),
            )
            .field(
                "status",
                &self.status(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .finish()
    }
}

impl Default for AtomicPoolTracker {
    fn default() -> Self {
        Self::new(0)
    }
}

impl AtomicPoolTracker {
    #[allow(dead_code)]
    pub const BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
    /// Creates a new instance from a raw integer value.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn new(val: u32) -> Self {
        Self(<::bitcraft::reexport::portable_atomic::AtomicU32>::new(val))
    }
    /// Returns the raw interior integer value via `load`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn load(&self, order: ::bitcraft::reexport::portable_atomic::Ordering) -> u32 {
        self.0.load(order)
    }
    /// Stores a raw integer value via `store`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn store(
        &self,
        val: u32,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        self.0.store(val, order)
    }
    pub const IS_ACTIVE_OFFSET: usize = 0;
    pub const IS_ACTIVE_BITS: usize = 1;
    #[doc(hidden)]
    const IS_ACTIVE_MASK: u32 = ((!0 as <u32 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u32 as ::bitcraft::BitLength>::BITS - Self::IS_ACTIVE_BITS)) as u32;
    #[allow(dead_code)]
    #[inline]
    pub fn is_active(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> bool {
        ((self.0.load(order) >> Self::IS_ACTIVE_OFFSET) & Self::IS_ACTIVE_MASK) != 0
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_is_active(
        &self,
        val: bool,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        let val_masked = val as u32;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::IS_ACTIVE_MASK << Self::IS_ACTIVE_OFFSET))
                            | (val_masked << Self::IS_ACTIVE_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_is_active(
        &self,
        val: bool,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_is_active(val, order);
        Ok(())
    }
    pub const ACTIVE_CONNECTIONS_OFFSET: usize = 0 + 1;
    pub const ACTIVE_CONNECTIONS_BITS: usize = 15;
    #[doc(hidden)]
    const ACTIVE_CONNECTIONS_MASK: u32 = ((!0
        as <u32 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u32 as ::bitcraft::BitLength>::BITS - Self::ACTIVE_CONNECTIONS_BITS))
        as u32;
    #[allow(dead_code)]
    #[inline]
    pub fn active_connections(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> u16 {
        ((self.0.load(order) >> Self::ACTIVE_CONNECTIONS_OFFSET)
            & Self::ACTIVE_CONNECTIONS_MASK) as u16
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_active_connections(
        &self,
        val: u16,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        if true {
            if !((val as u32) <= Self::ACTIVE_CONNECTIONS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 15),
                    );
                }
            }
        }
        let val_masked = (val as u32) & Self::ACTIVE_CONNECTIONS_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw
                            & !(Self::ACTIVE_CONNECTIONS_MASK
                                << Self::ACTIVE_CONNECTIONS_OFFSET))
                            | (val_masked << Self::ACTIVE_CONNECTIONS_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_active_connections(
        &self,
        val: u16,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u32) > Self::ACTIVE_CONNECTIONS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u32) as u128,
                allocated_bits: 15,
            });
        }
        let val_masked = (val as u32) & Self::ACTIVE_CONNECTIONS_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw
                            & !(Self::ACTIVE_CONNECTIONS_MASK
                                << Self::ACTIVE_CONNECTIONS_OFFSET))
                            | (val_masked << Self::ACTIVE_CONNECTIONS_OFFSET),
                    )
                },
            )
            .unwrap();
        Ok(())
    }
    pub const STATUS_OFFSET: usize = 0 + 1 + 15;
    pub const STATUS_BITS: usize = 2;
    #[doc(hidden)]
    const STATUS_MASK: u32 = ((!0 as <u32 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u32 as ::bitcraft::BitLength>::BITS - Self::STATUS_BITS)) as u32;
    #[allow(dead_code)]
    #[inline]
    pub fn status(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Status {
        Status::from_bits(
            ((self.0.load(order) >> Self::STATUS_OFFSET) & Self::STATUS_MASK) as _,
        )
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_status(
        &self,
        val: Status,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as u32) & Self::STATUS_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
                            | (val_masked << Self::STATUS_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_status(
        &self,
        val: Status,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_status(val, order);
        Ok(())
    }
}

#[repr(transparent)]
pub struct AtomicPoolTrackerValue(pub u32);

#[automatically_derived]
impl ::core::marker::Copy for AtomicPoolTrackerValue {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AtomicPoolTrackerValue {}

#[automatically_derived]
impl ::core::clone::Clone for AtomicPoolTrackerValue {
    #[inline]
    fn clone(&self) -> AtomicPoolTrackerValue {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AtomicPoolTrackerValue {}

#[automatically_derived]
impl ::core::cmp::PartialEq for AtomicPoolTrackerValue {
    #[inline]
    fn eq(&self, other: &AtomicPoolTrackerValue) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for AtomicPoolTrackerValue {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}

#[automatically_derived]
impl ::core::default::Default for AtomicPoolTrackerValue {
    #[inline]
    fn default() -> AtomicPoolTrackerValue {
        AtomicPoolTrackerValue(::core::default::Default::default())
    }
}

impl core::fmt::Debug for AtomicPoolTrackerValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AtomicPoolTrackerValue")
            .field("raw", &self.0)
            .field("is_active", &self.is_active())
            .field("active_connections", &self.active_connections())
            .field("status", &self.status())
            .finish()
    }
}

impl AtomicPoolTrackerValue {
    #[allow(dead_code)]
    pub const BITS: usize = <u32 as ::bitcraft::BitLength>::BITS;
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const IS_ACTIVE_OFFSET: usize = 0;
    /// The number of bits allocated for the `$field_name` property.
    pub const IS_ACTIVE_BITS: usize = 1;
    #[doc(hidden)]
    const IS_ACTIVE_MASK: u32 = ((!0 as <u32 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u32 as ::bitcraft::BitLength>::BITS - Self::IS_ACTIVE_BITS)) as u32;
    #[allow(dead_code)]
    #[inline]
    ///Returns the boolean value mapping to the `is_active` flag.
    pub const fn is_active(self) -> bool {
        ((self.0 >> Self::IS_ACTIVE_OFFSET) & Self::IS_ACTIVE_MASK) != 0
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to set the `is_active` flag.
    pub fn set_is_active(&mut self, val: bool) {
        let val_masked = val as u32;
        self.0 = (self.0 & !(Self::IS_ACTIVE_MASK << Self::IS_ACTIVE_OFFSET))
            | (val_masked << Self::IS_ACTIVE_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `is_active` flag specified.
    pub const fn with_is_active(self, val: bool) -> Self {
        let val_masked = val as u32;
        Self(
            (self.0 & !(Self::IS_ACTIVE_MASK << Self::IS_ACTIVE_OFFSET))
                | (val_masked << Self::IS_ACTIVE_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Inline mutation to set the `is_active` flag. Returns `Ok(())` since booleans cannot overflow.
    pub fn try_set_is_active(
        &mut self,
        val: bool,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_is_active(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `is_active` flag specified. Returns `Ok(Self)` since booleans cannot overflow.
    pub const fn try_with_is_active(
        self,
        val: bool,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_is_active(val))
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const ACTIVE_CONNECTIONS_OFFSET: usize = 0 + 1;
    /// The number of bits allocated for the `$field_name` property.
    pub const ACTIVE_CONNECTIONS_BITS: usize = 15;
    #[doc(hidden)]
    const ACTIVE_CONNECTIONS_MASK: u32 = ((!0
        as <u32 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u32 as ::bitcraft::BitLength>::BITS - Self::ACTIVE_CONNECTIONS_BITS))
        as u32;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `active_connections` property as a `u16`.
    pub const fn active_connections(self) -> u16 {
        ((self.0 >> Self::ACTIVE_CONNECTIONS_OFFSET) & Self::ACTIVE_CONNECTIONS_MASK)
            as u16
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `active_connections` property. Masks inputs over 15 bits.
    pub fn set_active_connections(&mut self, val: u16) {
        if true {
            if !((val as u32) <= Self::ACTIVE_CONNECTIONS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 15),
                    );
                }
            }
        }
        let val_masked = (val as u32) & Self::ACTIVE_CONNECTIONS_MASK;
        self.0 = (self.0
            & !(Self::ACTIVE_CONNECTIONS_MASK << Self::ACTIVE_CONNECTIONS_OFFSET))
            | (val_masked << Self::ACTIVE_CONNECTIONS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `active_connections` property mapped. Masks inputs over 15 bits.
    pub const fn with_active_connections(self, val: u16) -> Self {
        if true {
            if !((val as u32) <= Self::ACTIVE_CONNECTIONS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u32) & Self::ACTIVE_CONNECTIONS_MASK;
        Self(
            (self.0
                & !(Self::ACTIVE_CONNECTIONS_MASK << Self::ACTIVE_CONNECTIONS_OFFSET))
                | (val_masked << Self::ACTIVE_CONNECTIONS_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `active_connections` property. Returns a `BitstructError` if the value overflows 15 bits.
    pub fn try_set_active_connections(
        &mut self,
        val: u16,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u32) > Self::ACTIVE_CONNECTIONS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u32) as u128,
                allocated_bits: 15,
            });
        }
        let val_masked = (val as u32) & Self::ACTIVE_CONNECTIONS_MASK;
        self.0 = (self.0
            & !(Self::ACTIVE_CONNECTIONS_MASK << Self::ACTIVE_CONNECTIONS_OFFSET))
            | (val_masked << Self::ACTIVE_CONNECTIONS_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `active_connections` property. Returns a `BitstructError` if the value overflows 15 bits.
    pub const fn try_with_active_connections(
        self,
        val: u16,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u32) > Self::ACTIVE_CONNECTIONS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u32) as u128,
                allocated_bits: 15,
            });
        }
        let val_masked = (val as u32) & Self::ACTIVE_CONNECTIONS_MASK;
        Ok(
            Self(
                (self.0
                    & !(Self::ACTIVE_CONNECTIONS_MASK
                        << Self::ACTIVE_CONNECTIONS_OFFSET))
                    | (val_masked << Self::ACTIVE_CONNECTIONS_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const STATUS_OFFSET: usize = 0 + 1 + 15;
    /// The number of bits allocated for the `$field_name` property.
    pub const STATUS_BITS: usize = 2;
    #[doc(hidden)]
    const STATUS_MASK: u32 = ((!0 as <u32 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u32 as ::bitcraft::BitLength>::BITS - Self::STATUS_BITS)) as u32;
    #[allow(dead_code)]
    ///Returns the `status` variant strictly typed to the `Status` enumeration.
    pub const fn status(self) -> Status {
        Status::from_bits(((self.0 >> Self::STATUS_OFFSET) & Self::STATUS_MASK) as _)
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the bounded `Status` enumeration to the `status` property.
    pub fn set_status(&mut self, val: Status) {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as u32) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield bounded by the `Status` enumeration supplied to `status`.
    pub const fn with_status(self, val: Status) -> Self {
        const _: () = if !(<Status>::BITS <= 2) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Enum bit width exceeds allocated field width"),
                );
            }
        };
        let val_masked = (val.to_bits() as u32) & Self::STATUS_MASK;
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
        self.set_status(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the bounded `Status` enumeration to the `status` property. Returns a `BitstructError` if the value overflows 2 bits.
    pub const fn try_with_status(
        self,
        val: Status,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_status(val))
    }
    /// Returns the raw interior integer value.
    ///
    /// This is useful for serializing the struct or passing it to external APIs.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u32 {
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
    pub const fn from_bits(val: u32) -> Self {
        Self(val)
    }
}

impl AtomicPoolTracker {
    /// Returns a non-atomic snapshot of the current state as a `Value` struct.
    #[inline]
    pub fn get(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> AtomicPoolTrackerValue {
        AtomicPoolTrackerValue::from_bits(self.0.load(order))
    }
    /// Completely overwrites the entire atomic state with the given `Value`.
    /// This is a direct atomic `store` operation and does not perform a CAS loop.
    #[inline]
    pub fn set(
        &self,
        val: AtomicPoolTrackerValue,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        self.0.store(val.to_bits(), order);
    }
    /// Atomically updates multiple fields using a Compare-And-Swap (CAS) loop.
    ///
    /// The provided closure is called with a mutable `Value` representing the current state.
    /// Modify the value, and the changes will be applied atomically.
    ///
    /// Unlike `set`, this method guarantees that fields you do not modify within the closure
    /// will retain any concurrent updates made by other threads between the load and the store.
    #[inline]
    pub fn update<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> AtomicPoolTrackerValue
    where
        F: FnMut(&mut AtomicPoolTrackerValue),
    {
        let raw_prev = self
            .0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = AtomicPoolTrackerValue::from_bits(raw);
                    f(&mut snap);
                    Some(snap.to_bits())
                },
            )
            .unwrap();
        AtomicPoolTrackerValue::from_bits(raw_prev)
    }
    /// Conditionally updates multiple fields using a Compare-And-Swap (CAS) loop.
    ///
    /// The provided closure must return `Some(())` to commit the new state, or `None` to abort the loop.
    /// If `None` is returned, the CAS loop is aborted and `Err(Value)` containing the un-modified state is returned.
    #[inline]
    pub fn update_or_abort<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> Result<AtomicPoolTrackerValue, AtomicPoolTrackerValue>
    where
        F: FnMut(&mut AtomicPoolTrackerValue) -> Option<()>,
    {
        self.0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = AtomicPoolTrackerValue::from_bits(raw);
                    f(&mut snap).map(|_| snap.to_bits())
                },
            )
            .map(|raw| AtomicPoolTrackerValue::from_bits(raw))
            .map_err(|raw| AtomicPoolTrackerValue::from_bits(raw))
    }
}

#[repr(transparent)]
pub struct AtomicStatus(pub ::bitcraft::reexport::portable_atomic::AtomicU32);

impl core::fmt::Debug for AtomicStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let raw = self.0.load(::bitcraft::reexport::portable_atomic::Ordering::Relaxed);
        let val = AtomicStatusValue::from_bits(raw as _);
        f.debug_tuple("AtomicStatus").field(&val).finish()
    }
}

#[repr(transparent)]
pub struct AtomicStatusValue(pub <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim);

#[automatically_derived]
impl ::core::marker::Copy for AtomicStatusValue {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AtomicStatusValue {}

#[automatically_derived]
impl ::core::clone::Clone for AtomicStatusValue {
    #[inline]
    fn clone(&self) -> AtomicStatusValue {
        let _: ::core::clone::AssertParamIsClone<
            <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim,
        >;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AtomicStatusValue {}

#[automatically_derived]
impl ::core::cmp::PartialEq for AtomicStatusValue {
    #[inline]
    fn eq(&self, other: &AtomicStatusValue) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for AtomicStatusValue {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim,
        >;
    }
}

#[automatically_derived]
impl ::core::default::Default for AtomicStatusValue {
    #[inline]
    fn default() -> AtomicStatusValue {
        AtomicStatusValue(::core::default::Default::default())
    }
}

impl core::fmt::Debug for AtomicStatusValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self.0 {
            0 => "OFF",
            1 => "ON",
            2 => "FAULT",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{0}({1})::{2}", "AtomicStatusValue", self.0, s))
    }
}

impl AtomicStatusValue {
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
    #[allow(dead_code)]
    /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
    ///
    /// Useful for manually validating raw input before conversion.
    pub const MASK: <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim = {
        type Prim = <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim;
        #[allow(dead_code)]
        const TOTAL_BITS: usize = <Prim as ::bitcraft::BitLength>::BITS;
        (!0 as Prim) >> (TOTAL_BITS - 2)
    };
    /// Returns true if the raw value corresponds to a defined enumeration variant.
    ///
    /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
    #[inline(always)]
    #[allow(dead_code)]
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
                enum_name: "AtomicStatusValue",
            })
        }
    }
}

impl ::bitcraft::ValidField for AtomicStatusValue {
    const ASSERT_VALID: () = ();
}

impl Default for AtomicStatus {
    fn default() -> Self {
        Self::new(AtomicStatusValue::default())
    }
}

impl AtomicStatus {
    #[allow(dead_code)]
    pub const BITS: usize = 2;
    /// Creates a new atomic instance from an enum variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn new(val: AtomicStatusValue) -> Self {
        Self(<::bitcraft::reexport::portable_atomic::AtomicU32>::new(val.to_bits() as _))
    }
    /// Returns the current variant via `load`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn load(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> AtomicStatusValue {
        AtomicStatusValue::from_bits(self.0.load(order) as _)
    }
    /// Stores a new variant via `store`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn store(
        &self,
        val: AtomicStatusValue,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        self.0.store(val.to_bits() as _, order)
    }
    /// Atomically swaps the variant and returns the previous one.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn swap(
        &self,
        val: AtomicStatusValue,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> AtomicStatusValue {
        AtomicStatusValue::from_bits(self.0.swap(val.to_bits() as _, order) as _)
    }
    /// Compares and exchanges the variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn compare_exchange(
        &self,
        current: AtomicStatusValue,
        new: AtomicStatusValue,
        success: ::bitcraft::reexport::portable_atomic::Ordering,
        failure: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<AtomicStatusValue, AtomicStatusValue> {
        self.0
            .compare_exchange(
                current.to_bits() as _,
                new.to_bits() as _,
                success,
                failure,
            )
            .map(|raw| AtomicStatusValue::from_bits(raw as _))
            .map_err(|raw| AtomicStatusValue::from_bits(raw as _))
    }
    /// Weakly compares and exchanges the variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn compare_exchange_weak(
        &self,
        current: AtomicStatusValue,
        new: AtomicStatusValue,
        success: ::bitcraft::reexport::portable_atomic::Ordering,
        failure: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<AtomicStatusValue, AtomicStatusValue> {
        self.0
            .compare_exchange_weak(
                current.to_bits() as _,
                new.to_bits() as _,
                success,
                failure,
            )
            .map(|raw| AtomicStatusValue::from_bits(raw as _))
            .map_err(|raw| AtomicStatusValue::from_bits(raw as _))
    }
    /// Fetches and updates the variant via a CAS loop closure.
    /// The closure must return `Some(variant)` to commit, or `None` to abort.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn update_or_abort<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> Result<AtomicStatusValue, AtomicStatusValue>
    where
        F: FnMut(AtomicStatusValue) -> Option<AtomicStatusValue>,
    {
        self.0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let snap = AtomicStatusValue::from_bits(raw as _);
                    f(snap).map(|v| v.to_bits() as _)
                },
            )
            .map(|raw| AtomicStatusValue::from_bits(raw as _))
            .map_err(|raw| AtomicStatusValue::from_bits(raw as _))
    }
    /// Fetches and updates the variant via a CAS loop closure.
    /// The closure must return the new variant to commit.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn update<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> AtomicStatusValue
    where
        F: FnMut(AtomicStatusValue) -> AtomicStatusValue,
    {
        let raw_prev = self
            .0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let snap = AtomicStatusValue::from_bits(raw as _);
                    Some(f(snap).to_bits() as _)
                },
            )
            .unwrap();
        AtomicStatusValue::from_bits(raw_prev as _)
    }
}

#[repr(transparent)]
pub struct LargeAtomicTracker(pub ::bitcraft::reexport::portable_atomic::AtomicU128);

impl core::fmt::Debug for LargeAtomicTracker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LargeAtomicTracker")
            .field(
                "raw",
                &self.0.load(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .field(
                "is_active",
                &self.is_active(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .field(
                "user_id",
                &self.user_id(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .field(
                "session_id",
                &self
                    .session_id(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .field(
                "flags",
                &self.flags(::bitcraft::reexport::portable_atomic::Ordering::Relaxed),
            )
            .finish()
    }
}

impl Default for LargeAtomicTracker {
    fn default() -> Self {
        Self::new(0)
    }
}

impl LargeAtomicTracker {
    #[allow(dead_code)]
    pub const BITS: usize = <u128 as ::bitcraft::BitLength>::BITS;
    /// Creates a new instance from a raw integer value.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn new(val: u128) -> Self {
        Self(<::bitcraft::reexport::portable_atomic::AtomicU128>::new(val))
    }
    /// Returns the raw interior integer value via `load`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn load(&self, order: ::bitcraft::reexport::portable_atomic::Ordering) -> u128 {
        self.0.load(order)
    }
    /// Stores a raw integer value via `store`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn store(
        &self,
        val: u128,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        self.0.store(val, order)
    }
    pub const IS_ACTIVE_OFFSET: usize = 0;
    pub const IS_ACTIVE_BITS: usize = 1;
    #[doc(hidden)]
    const IS_ACTIVE_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::IS_ACTIVE_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    pub fn is_active(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> bool {
        ((self.0.load(order) >> Self::IS_ACTIVE_OFFSET) & Self::IS_ACTIVE_MASK) != 0
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_is_active(
        &self,
        val: bool,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        let val_masked = val as u128;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::IS_ACTIVE_MASK << Self::IS_ACTIVE_OFFSET))
                            | (val_masked << Self::IS_ACTIVE_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_is_active(
        &self,
        val: bool,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_is_active(val, order);
        Ok(())
    }
    pub const USER_ID_OFFSET: usize = 0 + 1;
    pub const USER_ID_BITS: usize = 64;
    #[doc(hidden)]
    const USER_ID_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::USER_ID_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    pub fn user_id(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> u64 {
        ((self.0.load(order) >> Self::USER_ID_OFFSET) & Self::USER_ID_MASK) as u64
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_user_id(
        &self,
        val: u64,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        if true {
            if !((val as u128) <= Self::USER_ID_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 64),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::USER_ID_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::USER_ID_MASK << Self::USER_ID_OFFSET))
                            | (val_masked << Self::USER_ID_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_user_id(
        &self,
        val: u64,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::USER_ID_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 64,
            });
        }
        let val_masked = (val as u128) & Self::USER_ID_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::USER_ID_MASK << Self::USER_ID_OFFSET))
                            | (val_masked << Self::USER_ID_OFFSET),
                    )
                },
            )
            .unwrap();
        Ok(())
    }
    pub const SESSION_ID_OFFSET: usize = 0 + 1 + 64;
    pub const SESSION_ID_BITS: usize = 32;
    #[doc(hidden)]
    const SESSION_ID_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::SESSION_ID_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    pub fn session_id(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> u32 {
        ((self.0.load(order) >> Self::SESSION_ID_OFFSET) & Self::SESSION_ID_MASK) as u32
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_session_id(
        &self,
        val: u32,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        if true {
            if !((val as u128) <= Self::SESSION_ID_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 32),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::SESSION_ID_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::SESSION_ID_MASK << Self::SESSION_ID_OFFSET))
                            | (val_masked << Self::SESSION_ID_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_session_id(
        &self,
        val: u32,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::SESSION_ID_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 32,
            });
        }
        let val_masked = (val as u128) & Self::SESSION_ID_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::SESSION_ID_MASK << Self::SESSION_ID_OFFSET))
                            | (val_masked << Self::SESSION_ID_OFFSET),
                    )
                },
            )
            .unwrap();
        Ok(())
    }
    pub const FLAGS_OFFSET: usize = 0 + 1 + 64 + 32;
    pub const FLAGS_BITS: usize = 31;
    #[doc(hidden)]
    const FLAGS_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::FLAGS_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    pub fn flags(&self, order: ::bitcraft::reexport::portable_atomic::Ordering) -> u32 {
        ((self.0.load(order) >> Self::FLAGS_OFFSET) & Self::FLAGS_MASK) as u32
    }
    #[allow(dead_code)]
    #[inline]
    pub fn set_flags(
        &self,
        val: u32,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 31),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::FLAGS_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::FLAGS_MASK << Self::FLAGS_OFFSET))
                            | (val_masked << Self::FLAGS_OFFSET),
                    )
                },
            )
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn try_set_flags(
        &self,
        val: u32,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 31,
            });
        }
        let val_masked = (val as u128) & Self::FLAGS_MASK;
        self.0
            .fetch_update(
                order,
                ::bitcraft::reexport::portable_atomic::Ordering::Relaxed,
                |raw| {
                    Some(
                        (raw & !(Self::FLAGS_MASK << Self::FLAGS_OFFSET))
                            | (val_masked << Self::FLAGS_OFFSET),
                    )
                },
            )
            .unwrap();
        Ok(())
    }
}

#[repr(transparent)]
pub struct LargeAtomicTrackerValue(pub u128);

#[automatically_derived]
impl ::core::marker::Copy for LargeAtomicTrackerValue {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for LargeAtomicTrackerValue {}

#[automatically_derived]
impl ::core::clone::Clone for LargeAtomicTrackerValue {
    #[inline]
    fn clone(&self) -> LargeAtomicTrackerValue {
        let _: ::core::clone::AssertParamIsClone<u128>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for LargeAtomicTrackerValue {}

#[automatically_derived]
impl ::core::cmp::PartialEq for LargeAtomicTrackerValue {
    #[inline]
    fn eq(&self, other: &LargeAtomicTrackerValue) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for LargeAtomicTrackerValue {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u128>;
    }
}

#[automatically_derived]
impl ::core::default::Default for LargeAtomicTrackerValue {
    #[inline]
    fn default() -> LargeAtomicTrackerValue {
        LargeAtomicTrackerValue(::core::default::Default::default())
    }
}

impl core::fmt::Debug for LargeAtomicTrackerValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LargeAtomicTrackerValue")
            .field("raw", &self.0)
            .field("is_active", &self.is_active())
            .field("user_id", &self.user_id())
            .field("session_id", &self.session_id())
            .field("flags", &self.flags())
            .finish()
    }
}

impl LargeAtomicTrackerValue {
    #[allow(dead_code)]
    pub const BITS: usize = <u128 as ::bitcraft::BitLength>::BITS;
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const IS_ACTIVE_OFFSET: usize = 0;
    /// The number of bits allocated for the `$field_name` property.
    pub const IS_ACTIVE_BITS: usize = 1;
    #[doc(hidden)]
    const IS_ACTIVE_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::IS_ACTIVE_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    ///Returns the boolean value mapping to the `is_active` flag.
    pub const fn is_active(self) -> bool {
        ((self.0 >> Self::IS_ACTIVE_OFFSET) & Self::IS_ACTIVE_MASK) != 0
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to set the `is_active` flag.
    pub fn set_is_active(&mut self, val: bool) {
        let val_masked = val as u128;
        self.0 = (self.0 & !(Self::IS_ACTIVE_MASK << Self::IS_ACTIVE_OFFSET))
            | (val_masked << Self::IS_ACTIVE_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `is_active` flag specified.
    pub const fn with_is_active(self, val: bool) -> Self {
        let val_masked = val as u128;
        Self(
            (self.0 & !(Self::IS_ACTIVE_MASK << Self::IS_ACTIVE_OFFSET))
                | (val_masked << Self::IS_ACTIVE_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Inline mutation to set the `is_active` flag. Returns `Ok(())` since booleans cannot overflow.
    pub fn try_set_is_active(
        &mut self,
        val: bool,
    ) -> Result<(), ::bitcraft::BitstructError> {
        self.set_is_active(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `is_active` flag specified. Returns `Ok(Self)` since booleans cannot overflow.
    pub const fn try_with_is_active(
        self,
        val: bool,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        Ok(self.with_is_active(val))
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const USER_ID_OFFSET: usize = 0 + 1;
    /// The number of bits allocated for the `$field_name` property.
    pub const USER_ID_BITS: usize = 64;
    #[doc(hidden)]
    const USER_ID_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::USER_ID_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `user_id` property as a `u64`.
    pub const fn user_id(self) -> u64 {
        ((self.0 >> Self::USER_ID_OFFSET) & Self::USER_ID_MASK) as u64
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `user_id` property. Masks inputs over 64 bits.
    pub fn set_user_id(&mut self, val: u64) {
        if true {
            if !((val as u128) <= Self::USER_ID_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 64),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::USER_ID_MASK;
        self.0 = (self.0 & !(Self::USER_ID_MASK << Self::USER_ID_OFFSET))
            | (val_masked << Self::USER_ID_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `user_id` property mapped. Masks inputs over 64 bits.
    pub const fn with_user_id(self, val: u64) -> Self {
        if true {
            if !((val as u128) <= Self::USER_ID_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::USER_ID_MASK;
        Self(
            (self.0 & !(Self::USER_ID_MASK << Self::USER_ID_OFFSET))
                | (val_masked << Self::USER_ID_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `user_id` property. Returns a `BitstructError` if the value overflows 64 bits.
    pub fn try_set_user_id(
        &mut self,
        val: u64,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::USER_ID_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 64,
            });
        }
        let val_masked = (val as u128) & Self::USER_ID_MASK;
        self.0 = (self.0 & !(Self::USER_ID_MASK << Self::USER_ID_OFFSET))
            | (val_masked << Self::USER_ID_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `user_id` property. Returns a `BitstructError` if the value overflows 64 bits.
    pub const fn try_with_user_id(
        self,
        val: u64,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::USER_ID_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 64,
            });
        }
        let val_masked = (val as u128) & Self::USER_ID_MASK;
        Ok(
            Self(
                (self.0 & !(Self::USER_ID_MASK << Self::USER_ID_OFFSET))
                    | (val_masked << Self::USER_ID_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const SESSION_ID_OFFSET: usize = 0 + 1 + 64;
    /// The number of bits allocated for the `$field_name` property.
    pub const SESSION_ID_BITS: usize = 32;
    #[doc(hidden)]
    const SESSION_ID_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::SESSION_ID_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `session_id` property as a `u32`.
    pub const fn session_id(self) -> u32 {
        ((self.0 >> Self::SESSION_ID_OFFSET) & Self::SESSION_ID_MASK) as u32
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `session_id` property. Masks inputs over 32 bits.
    pub fn set_session_id(&mut self, val: u32) {
        if true {
            if !((val as u128) <= Self::SESSION_ID_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 32),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::SESSION_ID_MASK;
        self.0 = (self.0 & !(Self::SESSION_ID_MASK << Self::SESSION_ID_OFFSET))
            | (val_masked << Self::SESSION_ID_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `session_id` property mapped. Masks inputs over 32 bits.
    pub const fn with_session_id(self, val: u32) -> Self {
        if true {
            if !((val as u128) <= Self::SESSION_ID_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::SESSION_ID_MASK;
        Self(
            (self.0 & !(Self::SESSION_ID_MASK << Self::SESSION_ID_OFFSET))
                | (val_masked << Self::SESSION_ID_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `session_id` property. Returns a `BitstructError` if the value overflows 32 bits.
    pub fn try_set_session_id(
        &mut self,
        val: u32,
    ) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::SESSION_ID_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 32,
            });
        }
        let val_masked = (val as u128) & Self::SESSION_ID_MASK;
        self.0 = (self.0 & !(Self::SESSION_ID_MASK << Self::SESSION_ID_OFFSET))
            | (val_masked << Self::SESSION_ID_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `session_id` property. Returns a `BitstructError` if the value overflows 32 bits.
    pub const fn try_with_session_id(
        self,
        val: u32,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::SESSION_ID_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 32,
            });
        }
        let val_masked = (val as u128) & Self::SESSION_ID_MASK;
        Ok(
            Self(
                (self.0 & !(Self::SESSION_ID_MASK << Self::SESSION_ID_OFFSET))
                    | (val_masked << Self::SESSION_ID_OFFSET),
            ),
        )
    }
    /// The bit-offset of the `$field_name` property within the underlying storage.
    pub const FLAGS_OFFSET: usize = 0 + 1 + 64 + 32;
    /// The number of bits allocated for the `$field_name` property.
    pub const FLAGS_BITS: usize = 31;
    #[doc(hidden)]
    const FLAGS_MASK: u128 = ((!0 as <u128 as ::bitcraft::IsValidBaseInt>::Unsigned)
        >> (<u128 as ::bitcraft::BitLength>::BITS - Self::FLAGS_BITS)) as u128;
    #[allow(dead_code)]
    #[inline]
    ///Returns the `flags` property as a `u32`.
    pub const fn flags(self) -> u32 {
        ((self.0 >> Self::FLAGS_OFFSET) & Self::FLAGS_MASK) as u32
    }
    #[allow(dead_code)]
    #[inline]
    ///Inline mutation to apply the `flags` property. Masks inputs over 31 bits.
    pub fn set_flags(&mut self, val: u32) {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 31),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::FLAGS_MASK;
        self.0 = (self.0 & !(Self::FLAGS_MASK << Self::FLAGS_OFFSET))
            | (val_masked << Self::FLAGS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield with the `flags` property mapped. Masks inputs over 31 bits.
    pub const fn with_flags(self, val: u32) -> Self {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        let val_masked = (val as u128) & Self::FLAGS_MASK;
        Self(
            (self.0 & !(Self::FLAGS_MASK << Self::FLAGS_OFFSET))
                | (val_masked << Self::FLAGS_OFFSET),
        )
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `flags` property. Returns a `BitstructError` if the value overflows 31 bits.
    pub fn try_set_flags(&mut self, val: u32) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 31,
            });
        }
        let val_masked = (val as u128) & Self::FLAGS_MASK;
        self.0 = (self.0 & !(Self::FLAGS_MASK << Self::FLAGS_OFFSET))
            | (val_masked << Self::FLAGS_OFFSET);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `flags` property. Returns a `BitstructError` if the value overflows 31 bits.
    pub const fn try_with_flags(
        self,
        val: u32,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: (val as u128) as u128,
                allocated_bits: 31,
            });
        }
        let val_masked = (val as u128) & Self::FLAGS_MASK;
        Ok(
            Self(
                (self.0 & !(Self::FLAGS_MASK << Self::FLAGS_OFFSET))
                    | (val_masked << Self::FLAGS_OFFSET),
            ),
        )
    }
    /// Returns the raw interior integer value.
    ///
    /// This is useful for serializing the struct or passing it to external APIs.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u128 {
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
    pub const fn from_bits(val: u128) -> Self {
        Self(val)
    }
}

impl LargeAtomicTracker {
    /// Returns a non-atomic snapshot of the current state as a `Value` struct.
    #[inline]
    pub fn get(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> LargeAtomicTrackerValue {
        LargeAtomicTrackerValue::from_bits(self.0.load(order))
    }
    /// Completely overwrites the entire atomic state with the given `Value`.
    /// This is a direct atomic `store` operation and does not perform a CAS loop.
    #[inline]
    pub fn set(
        &self,
        val: LargeAtomicTrackerValue,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        self.0.store(val.to_bits(), order);
    }
    /// Atomically updates multiple fields using a Compare-And-Swap (CAS) loop.
    ///
    /// The provided closure is called with a mutable `Value` representing the current state.
    /// Modify the value, and the changes will be applied atomically.
    ///
    /// Unlike `set`, this method guarantees that fields you do not modify within the closure
    /// will retain any concurrent updates made by other threads between the load and the store.
    #[inline]
    pub fn update<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> LargeAtomicTrackerValue
    where
        F: FnMut(&mut LargeAtomicTrackerValue),
    {
        let raw_prev = self
            .0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = LargeAtomicTrackerValue::from_bits(raw);
                    f(&mut snap);
                    Some(snap.to_bits())
                },
            )
            .unwrap();
        LargeAtomicTrackerValue::from_bits(raw_prev)
    }
    /// Conditionally updates multiple fields using a Compare-And-Swap (CAS) loop.
    ///
    /// The provided closure must return `Some(())` to commit the new state, or `None` to abort the loop.
    /// If `None` is returned, the CAS loop is aborted and `Err(Value)` containing the un-modified state is returned.
    #[inline]
    pub fn update_or_abort<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> Result<LargeAtomicTrackerValue, LargeAtomicTrackerValue>
    where
        F: FnMut(&mut LargeAtomicTrackerValue) -> Option<()>,
    {
        self.0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = LargeAtomicTrackerValue::from_bits(raw);
                    f(&mut snap).map(|_| snap.to_bits())
                },
            )
            .map(|raw| LargeAtomicTrackerValue::from_bits(raw))
            .map_err(|raw| LargeAtomicTrackerValue::from_bits(raw))
    }
}

#[repr(transparent)]
pub struct LargeAtomicState(pub ::bitcraft::reexport::portable_atomic::AtomicU128);

impl core::fmt::Debug for LargeAtomicState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let raw = self.0.load(::bitcraft::reexport::portable_atomic::Ordering::Relaxed);
        let val = LargeAtomicStateValue::from_bits(raw as _);
        f.debug_tuple("LargeAtomicState").field(&val).finish()
    }
}

#[repr(transparent)]
pub struct LargeAtomicStateValue(
    pub <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim,
);

#[automatically_derived]
impl ::core::marker::Copy for LargeAtomicStateValue {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for LargeAtomicStateValue {}

#[automatically_derived]
impl ::core::clone::Clone for LargeAtomicStateValue {
    #[inline]
    fn clone(&self) -> LargeAtomicStateValue {
        let _: ::core::clone::AssertParamIsClone<
            <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim,
        >;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for LargeAtomicStateValue {}

#[automatically_derived]
impl ::core::cmp::PartialEq for LargeAtomicStateValue {
    #[inline]
    fn eq(&self, other: &LargeAtomicStateValue) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for LargeAtomicStateValue {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim,
        >;
    }
}

#[automatically_derived]
impl ::core::default::Default for LargeAtomicStateValue {
    #[inline]
    fn default() -> LargeAtomicStateValue {
        LargeAtomicStateValue(::core::default::Default::default())
    }
}

impl core::fmt::Debug for LargeAtomicStateValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self.0 {
            0 => "INITIAL",
            1 => "READY",
            2 => "ACTIVE",
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF => "TERMINATED",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{0}({1})::{2}", "LargeAtomicStateValue", self.0, s))
    }
}

impl LargeAtomicStateValue {
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `INITIAL` with raw value `0`.
    pub const INITIAL: Self = Self(0);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `READY` with raw value `1`.
    pub const READY: Self = Self(1);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `ACTIVE` with raw value `2`.
    pub const ACTIVE: Self = Self(2);
    #[allow(non_upper_case_globals, dead_code)]
    ///Enumeration variant for `TERMINATED` with raw value `0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`.
    pub const TERMINATED: Self = Self(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    #[allow(dead_code)]
    /// The number of bits allocated for this enumeration in memory.
    pub const BITS: usize = 128;
    #[allow(dead_code)]
    /// The maximum value allowed for this enumeration variant based on the allocated $bits bits.
    ///
    /// Useful for manually validating raw input before conversion.
    pub const MASK: <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim = {
        type Prim = <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim;
        #[allow(dead_code)]
        const TOTAL_BITS: usize = <Prim as ::bitcraft::BitLength>::BITS;
        (!0 as Prim) >> (TOTAL_BITS - 128)
    };
    /// Returns true if the raw value corresponds to a defined enumeration variant.
    ///
    /// This is a zero-cost check that compiles to a simple comparison or a small jump table.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_defined(self) -> bool {
        match self.0 {
            0 => true,
            1 => true,
            2 => true,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF => true,
            _ => false,
        }
    }
    /// Returns the raw integer value of the enumeration variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(
        self,
    ) -> <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim {
        self.0
    }
    /// Creates an enumeration variant from a raw integer value.
    ///
    /// # Panics
    /// In debug mode, this will panic if the value exceeds the allocated bit width.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(
        val: <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim,
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
        val: <::bitcraft::Bits<128> as ::bitcraft::BitenumType>::Prim,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        let s = Self(val);
        if s.is_defined() {
            Ok(s)
        } else {
            Err(::bitcraft::BitstructError::InvalidVariant {
                value: val as u128,
                enum_name: "LargeAtomicStateValue",
            })
        }
    }
}

impl ::bitcraft::ValidField for LargeAtomicStateValue {
    const ASSERT_VALID: () = ();
}

impl Default for LargeAtomicState {
    fn default() -> Self {
        Self::new(LargeAtomicStateValue::default())
    }
}

impl LargeAtomicState {
    #[allow(dead_code)]
    pub const BITS: usize = 128;
    /// Creates a new atomic instance from an enum variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn new(val: LargeAtomicStateValue) -> Self {
        Self(
            <::bitcraft::reexport::portable_atomic::AtomicU128>::new(val.to_bits() as _),
        )
    }
    /// Returns the current variant via `load`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn load(
        &self,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> LargeAtomicStateValue {
        LargeAtomicStateValue::from_bits(self.0.load(order) as _)
    }
    /// Stores a new variant via `store`.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn store(
        &self,
        val: LargeAtomicStateValue,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) {
        self.0.store(val.to_bits() as _, order)
    }
    /// Atomically swaps the variant and returns the previous one.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn swap(
        &self,
        val: LargeAtomicStateValue,
        order: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> LargeAtomicStateValue {
        LargeAtomicStateValue::from_bits(self.0.swap(val.to_bits() as _, order) as _)
    }
    /// Compares and exchanges the variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn compare_exchange(
        &self,
        current: LargeAtomicStateValue,
        new: LargeAtomicStateValue,
        success: ::bitcraft::reexport::portable_atomic::Ordering,
        failure: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<LargeAtomicStateValue, LargeAtomicStateValue> {
        self.0
            .compare_exchange(
                current.to_bits() as _,
                new.to_bits() as _,
                success,
                failure,
            )
            .map(|raw| LargeAtomicStateValue::from_bits(raw as _))
            .map_err(|raw| LargeAtomicStateValue::from_bits(raw as _))
    }
    /// Weakly compares and exchanges the variant.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn compare_exchange_weak(
        &self,
        current: LargeAtomicStateValue,
        new: LargeAtomicStateValue,
        success: ::bitcraft::reexport::portable_atomic::Ordering,
        failure: ::bitcraft::reexport::portable_atomic::Ordering,
    ) -> Result<LargeAtomicStateValue, LargeAtomicStateValue> {
        self.0
            .compare_exchange_weak(
                current.to_bits() as _,
                new.to_bits() as _,
                success,
                failure,
            )
            .map(|raw| LargeAtomicStateValue::from_bits(raw as _))
            .map_err(|raw| LargeAtomicStateValue::from_bits(raw as _))
    }
    /// Fetches and updates the variant via a CAS loop closure.
    /// The closure must return `Some(variant)` to commit, or `None` to abort.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn update_or_abort<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> Result<LargeAtomicStateValue, LargeAtomicStateValue>
    where
        F: FnMut(LargeAtomicStateValue) -> Option<LargeAtomicStateValue>,
    {
        self.0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let snap = LargeAtomicStateValue::from_bits(raw as _);
                    f(snap).map(|v| v.to_bits() as _)
                },
            )
            .map(|raw| LargeAtomicStateValue::from_bits(raw as _))
            .map_err(|raw| LargeAtomicStateValue::from_bits(raw as _))
    }
    /// Fetches and updates the variant via a CAS loop closure.
    /// The closure must return the new variant to commit.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn update<F>(
        &self,
        set_order: ::bitcraft::reexport::portable_atomic::Ordering,
        fetch_order: ::bitcraft::reexport::portable_atomic::Ordering,
        mut f: F,
    ) -> LargeAtomicStateValue
    where
        F: FnMut(LargeAtomicStateValue) -> LargeAtomicStateValue,
    {
        let raw_prev = self
            .0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let snap = LargeAtomicStateValue::from_bits(raw as _);
                    Some(f(snap).to_bits() as _)
                },
            )
            .unwrap();
        LargeAtomicStateValue::from_bits(raw_prev as _)
    }
}

#[repr(transparent)]
pub struct NibblePalette(pub u32);

#[automatically_derived]
impl ::core::marker::Copy for NibblePalette {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for NibblePalette {}

#[automatically_derived]
impl ::core::clone::Clone for NibblePalette {
    #[inline]
    fn clone(&self) -> NibblePalette {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}

#[automatically_derived]
impl ::core::default::Default for NibblePalette {
    #[inline]
    fn default() -> NibblePalette {
        NibblePalette(::core::default::Default::default())
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for NibblePalette {}

#[automatically_derived]
impl ::core::cmp::PartialEq for NibblePalette {
    #[inline]
    fn eq(&self, other: &NibblePalette) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for NibblePalette {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for NibblePalette {
    #[inline]
    fn partial_cmp(
        &self,
        other: &NibblePalette,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for NibblePalette {
    #[inline]
    fn cmp(&self, other: &NibblePalette) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for NibblePalette {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl core::fmt::Debug for NibblePalette {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..8 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl NibblePalette {
    pub const ELEMENT_WIDTH: usize = 4;
    pub const ELEMENT_COUNT: usize = 8;
    pub const TOTAL_BITS: usize = 4 * 8;
    pub const MASK: u32 = if 4 >= <u32 as ::bitcraft::BitLength>::BITS {
        !0 as u32
    } else {
        ((1 as u32) << 4) - 1
    };
    #[inline(always)]
    pub const fn new(val: u32) -> Self {
        Self(val)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn get(&self, index: usize) -> u128 {
        if true {
            if !(index < 8) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 4;
        let raw = (self.0 >> shift) & Self::MASK;
        raw as u128
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: u128) {
        if true {
            if !(index < 8) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 4;
        let val_masked = value as u32 & Self::MASK;
        self.0 &= !(Self::MASK << shift);
        self.0 |= val_masked << shift;
    }
}

#[repr(transparent)]
pub struct SignedOffsets(pub u128);

#[automatically_derived]
impl ::core::marker::Copy for SignedOffsets {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for SignedOffsets {}

#[automatically_derived]
impl ::core::clone::Clone for SignedOffsets {
    #[inline]
    fn clone(&self) -> SignedOffsets {
        let _: ::core::clone::AssertParamIsClone<u128>;
        *self
    }
}

#[automatically_derived]
impl ::core::default::Default for SignedOffsets {
    #[inline]
    fn default() -> SignedOffsets {
        SignedOffsets(::core::default::Default::default())
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignedOffsets {}

#[automatically_derived]
impl ::core::cmp::PartialEq for SignedOffsets {
    #[inline]
    fn eq(&self, other: &SignedOffsets) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for SignedOffsets {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u128>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for SignedOffsets {
    #[inline]
    fn partial_cmp(
        &self,
        other: &SignedOffsets,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for SignedOffsets {
    #[inline]
    fn cmp(&self, other: &SignedOffsets) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for SignedOffsets {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl core::fmt::Debug for SignedOffsets {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..10 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl SignedOffsets {
    pub const ELEMENT_WIDTH: usize = 3;
    pub const ELEMENT_COUNT: usize = 10;
    pub const TOTAL_BITS: usize = 3 * 10;
    pub const MASK: u128 = if 3 >= <u128 as ::bitcraft::BitLength>::BITS {
        !0 as u128
    } else {
        ((1 as u128) << 3) - 1
    };
    #[inline(always)]
    pub const fn new(val: u128) -> Self {
        Self(val)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u128 {
        self.0
    }
    #[inline]
    pub fn get(&self, index: usize) -> i128 {
        if true {
            if !(index < 10) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 3;
        let raw = (self.0 >> shift) & Self::MASK;
        {
            let val = raw as i128;
            let shift = (core::mem::size_of::<i128>() * 8) - 3;
            (val << shift) >> shift
        }
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: i128) {
        if true {
            if !(index < 10) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 3;
        let val_masked = value as u128 & Self::MASK;
        self.0 &= !(Self::MASK << shift);
        self.0 |= val_masked << shift;
    }
}

#[repr(transparent)]
pub struct StatusFlags(pub u32);

#[automatically_derived]
impl ::core::marker::Copy for StatusFlags {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for StatusFlags {}

#[automatically_derived]
impl ::core::clone::Clone for StatusFlags {
    #[inline]
    fn clone(&self) -> StatusFlags {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}

#[automatically_derived]
impl ::core::default::Default for StatusFlags {
    #[inline]
    fn default() -> StatusFlags {
        StatusFlags(::core::default::Default::default())
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StatusFlags {}

#[automatically_derived]
impl ::core::cmp::PartialEq for StatusFlags {
    #[inline]
    fn eq(&self, other: &StatusFlags) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for StatusFlags {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for StatusFlags {
    #[inline]
    fn partial_cmp(
        &self,
        other: &StatusFlags,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for StatusFlags {
    #[inline]
    fn cmp(&self, other: &StatusFlags) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for StatusFlags {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl core::fmt::Debug for StatusFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..32 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl StatusFlags {
    pub const ELEMENT_WIDTH: usize = 1;
    pub const ELEMENT_COUNT: usize = 32;
    pub const TOTAL_BITS: usize = 1 * 32;
    pub const MASK: u32 = if 1 >= <u32 as ::bitcraft::BitLength>::BITS {
        !0 as u32
    } else {
        ((1 as u32) << 1) - 1
    };
    #[inline(always)]
    pub const fn new(val: u32) -> Self {
        Self(val)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        if true {
            if !(index < 32) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let raw = (self.0 >> shift) & Self::MASK;
        raw != 0
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        if true {
            if !(index < 32) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let val_masked = if value { 1 } else { 0 } & Self::MASK;
        self.0 &= !(Self::MASK << shift);
        self.0 |= val_masked << shift;
    }
}

#[repr(transparent)]
pub struct LargeBitFlags(pub u128);

#[automatically_derived]
impl ::core::marker::Copy for LargeBitFlags {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for LargeBitFlags {}

#[automatically_derived]
impl ::core::clone::Clone for LargeBitFlags {
    #[inline]
    fn clone(&self) -> LargeBitFlags {
        let _: ::core::clone::AssertParamIsClone<u128>;
        *self
    }
}

#[automatically_derived]
impl ::core::default::Default for LargeBitFlags {
    #[inline]
    fn default() -> LargeBitFlags {
        LargeBitFlags(::core::default::Default::default())
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for LargeBitFlags {}

#[automatically_derived]
impl ::core::cmp::PartialEq for LargeBitFlags {
    #[inline]
    fn eq(&self, other: &LargeBitFlags) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for LargeBitFlags {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u128>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for LargeBitFlags {
    #[inline]
    fn partial_cmp(
        &self,
        other: &LargeBitFlags,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for LargeBitFlags {
    #[inline]
    fn cmp(&self, other: &LargeBitFlags) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for LargeBitFlags {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl core::fmt::Debug for LargeBitFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..128 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl LargeBitFlags {
    pub const ELEMENT_WIDTH: usize = 1;
    pub const ELEMENT_COUNT: usize = 128;
    pub const TOTAL_BITS: usize = 1 * 128;
    pub const MASK: u128 = if 1 >= <u128 as ::bitcraft::BitLength>::BITS {
        !0 as u128
    } else {
        ((1 as u128) << 1) - 1
    };
    #[inline(always)]
    pub const fn new(val: u128) -> Self {
        Self(val)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u128 {
        self.0
    }
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let raw = (self.0 >> shift) & Self::MASK;
        raw != 0
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let val_masked = if value { 1 } else { 0 } & Self::MASK;
        self.0 &= !(Self::MASK << shift);
        self.0 |= val_masked << shift;
    }
}

#[repr(transparent)]
pub struct NibbleBuffer(pub [u8; (4 * 64 + 7) / 8]);

#[automatically_derived]
impl ::core::marker::Copy for NibbleBuffer {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for NibbleBuffer {}

#[automatically_derived]
impl ::core::clone::Clone for NibbleBuffer {
    #[inline]
    fn clone(&self) -> NibbleBuffer {
        let _: ::core::clone::AssertParamIsClone<[u8; (4 * 64 + 7) / 8]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for NibbleBuffer {}

#[automatically_derived]
impl ::core::cmp::PartialEq for NibbleBuffer {
    #[inline]
    fn eq(&self, other: &NibbleBuffer) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for NibbleBuffer {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; (4 * 64 + 7) / 8]>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for NibbleBuffer {
    #[inline]
    fn partial_cmp(
        &self,
        other: &NibbleBuffer,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for NibbleBuffer {
    #[inline]
    fn cmp(&self, other: &NibbleBuffer) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for NibbleBuffer {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl Default for NibbleBuffer {
    fn default() -> Self {
        Self([0u8; (4 * 64 + 7) / 8])
    }
}

impl core::fmt::Debug for NibbleBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..64 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl NibbleBuffer {
    pub const ELEMENT_WIDTH: usize = 4;
    pub const ELEMENT_COUNT: usize = 64;
    pub const TOTAL_BITS: usize = 4 * 64;
    pub const BYTES: usize = (4 * 64 + 7) / 8;
    const MASK: u128 = if 4 == 128 { !0u128 } else { (1u128 << 4) - 1 };
    #[inline]
    pub fn get(&self, index: usize) -> u128 {
        if true {
            if !(index < 64) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bytearray index out of bounds"),
                    );
                }
            }
        }
        let bit_offset = index * 4;
        let byte_idx = bit_offset / 8;
        let inner_bit_offset = bit_offset % 8;
        let bytes_to_read = (4 + inner_bit_offset + 7) / 8;
        let mut raw = 0u128;
        for i in 0..bytes_to_read {
            if byte_idx + i < Self::BYTES {
                raw |= (self.0[byte_idx + i] as u128) << (i * 8);
            }
        }
        let extracted = (raw >> inner_bit_offset) & Self::MASK;
        extracted as u128
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: u128) {
        if true {
            if !(index < 64) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bytearray index out of bounds"),
                    );
                }
            }
        }
        let bit_offset = index * 4;
        let byte_idx = bit_offset / 8;
        let inner_bit_offset = bit_offset % 8;
        let raw_val = value as u128 & Self::MASK;
        let val_shifted = raw_val << inner_bit_offset;
        let mask_shifted = Self::MASK << inner_bit_offset;
        let bytes_to_modify = (4 + inner_bit_offset + 7) / 8;
        for i in 0..bytes_to_modify {
            if byte_idx + i < Self::BYTES {
                let byte_mask = ((mask_shifted >> (i * 8)) & 0xFF) as u8;
                let byte_val = ((val_shifted >> (i * 8)) & 0xFF) as u8;
                self.0[byte_idx + i] &= !byte_mask;
                self.0[byte_idx + i] |= byte_val;
            }
        }
    }
}

#[repr(transparent)]
pub struct DeltaStream(pub [u8; (7 * 20 + 7) / 8]);

#[automatically_derived]
impl ::core::marker::Copy for DeltaStream {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for DeltaStream {}

#[automatically_derived]
impl ::core::clone::Clone for DeltaStream {
    #[inline]
    fn clone(&self) -> DeltaStream {
        let _: ::core::clone::AssertParamIsClone<[u8; (7 * 20 + 7) / 8]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for DeltaStream {}

#[automatically_derived]
impl ::core::cmp::PartialEq for DeltaStream {
    #[inline]
    fn eq(&self, other: &DeltaStream) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for DeltaStream {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; (7 * 20 + 7) / 8]>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for DeltaStream {
    #[inline]
    fn partial_cmp(
        &self,
        other: &DeltaStream,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for DeltaStream {
    #[inline]
    fn cmp(&self, other: &DeltaStream) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for DeltaStream {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl Default for DeltaStream {
    fn default() -> Self {
        Self([0u8; (7 * 20 + 7) / 8])
    }
}

impl core::fmt::Debug for DeltaStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..20 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl DeltaStream {
    pub const ELEMENT_WIDTH: usize = 7;
    pub const ELEMENT_COUNT: usize = 20;
    pub const TOTAL_BITS: usize = 7 * 20;
    pub const BYTES: usize = (7 * 20 + 7) / 8;
    const MASK: u128 = if 7 == 128 { !0u128 } else { (1u128 << 7) - 1 };
    #[inline]
    pub fn get(&self, index: usize) -> i128 {
        if true {
            if !(index < 20) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bytearray index out of bounds"),
                    );
                }
            }
        }
        let bit_offset = index * 7;
        let byte_idx = bit_offset / 8;
        let inner_bit_offset = bit_offset % 8;
        let bytes_to_read = (7 + inner_bit_offset + 7) / 8;
        let mut raw = 0u128;
        for i in 0..bytes_to_read {
            if byte_idx + i < Self::BYTES {
                raw |= (self.0[byte_idx + i] as u128) << (i * 8);
            }
        }
        let extracted = (raw >> inner_bit_offset) & Self::MASK;
        {
            let val = extracted as i128;
            let shift = (core::mem::size_of::<i128>() * 8) - 7;
            (val << shift) >> shift
        }
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: i128) {
        if true {
            if !(index < 20) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bytearray index out of bounds"),
                    );
                }
            }
        }
        let bit_offset = index * 7;
        let byte_idx = bit_offset / 8;
        let inner_bit_offset = bit_offset % 8;
        let raw_val = value as u128 & Self::MASK;
        let val_shifted = raw_val << inner_bit_offset;
        let mask_shifted = Self::MASK << inner_bit_offset;
        let bytes_to_modify = (7 + inner_bit_offset + 7) / 8;
        for i in 0..bytes_to_modify {
            if byte_idx + i < Self::BYTES {
                let byte_mask = ((mask_shifted >> (i * 8)) & 0xFF) as u8;
                let byte_val = ((val_shifted >> (i * 8)) & 0xFF) as u8;
                self.0[byte_idx + i] &= !byte_mask;
                self.0[byte_idx + i] |= byte_val;
            }
        }
    }
}

#[repr(transparent)]
pub struct ByteFlagArray(pub [u8; (1 * 128 + 7) / 8]);

#[automatically_derived]
impl ::core::marker::Copy for ByteFlagArray {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for ByteFlagArray {}

#[automatically_derived]
impl ::core::clone::Clone for ByteFlagArray {
    #[inline]
    fn clone(&self) -> ByteFlagArray {
        let _: ::core::clone::AssertParamIsClone<[u8; (1 * 128 + 7) / 8]>;
        *self
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ByteFlagArray {}

#[automatically_derived]
impl ::core::cmp::PartialEq for ByteFlagArray {
    #[inline]
    fn eq(&self, other: &ByteFlagArray) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for ByteFlagArray {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; (1 * 128 + 7) / 8]>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for ByteFlagArray {
    #[inline]
    fn partial_cmp(
        &self,
        other: &ByteFlagArray,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for ByteFlagArray {
    #[inline]
    fn cmp(&self, other: &ByteFlagArray) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for ByteFlagArray {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl Default for ByteFlagArray {
    fn default() -> Self {
        Self([0u8; (1 * 128 + 7) / 8])
    }
}

impl core::fmt::Debug for ByteFlagArray {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..128 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl ByteFlagArray {
    pub const ELEMENT_WIDTH: usize = 1;
    pub const ELEMENT_COUNT: usize = 128;
    pub const TOTAL_BITS: usize = 1 * 128;
    pub const BYTES: usize = (1 * 128 + 7) / 8;
    const MASK: u128 = if 1 == 128 { !0u128 } else { (1u128 << 1) - 1 };
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bytearray index out of bounds"),
                    );
                }
            }
        }
        let bit_offset = index * 1;
        let byte_idx = bit_offset / 8;
        let inner_bit_offset = bit_offset % 8;
        let bytes_to_read = (1 + inner_bit_offset + 7) / 8;
        let mut raw = 0u128;
        for i in 0..bytes_to_read {
            if byte_idx + i < Self::BYTES {
                raw |= (self.0[byte_idx + i] as u128) << (i * 8);
            }
        }
        let extracted = (raw >> inner_bit_offset) & Self::MASK;
        extracted != 0
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bytearray index out of bounds"),
                    );
                }
            }
        }
        let bit_offset = index * 1;
        let byte_idx = bit_offset / 8;
        let inner_bit_offset = bit_offset % 8;
        let raw_val = if value { 1 } else { 0 } & Self::MASK;
        let val_shifted = raw_val << inner_bit_offset;
        let mask_shifted = Self::MASK << inner_bit_offset;
        let bytes_to_modify = (1 + inner_bit_offset + 7) / 8;
        for i in 0..bytes_to_modify {
            if byte_idx + i < Self::BYTES {
                let byte_mask = ((mask_shifted >> (i * 8)) & 0xFF) as u8;
                let byte_val = ((val_shifted >> (i * 8)) & 0xFF) as u8;
                self.0[byte_idx + i] &= !byte_mask;
                self.0[byte_idx + i] |= byte_val;
            }
        }
    }
}

#[repr(transparent)]
pub struct AtomicNibbles(pub ::bitcraft::reexport::portable_atomic::AtomicU64);

impl core::fmt::Debug for AtomicNibbles {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..16 {
            list.entry(&self.get(i, ::bitcraft::Ordering::Relaxed));
        }
        list.finish()
    }
}

impl AtomicNibbles {
    pub const ELEMENT_WIDTH: usize = 4;
    pub const ELEMENT_COUNT: usize = 16;
    pub const TOTAL_BITS: usize = 4 * 16;
    pub const MASK: u64 = if 4 >= <u64 as ::bitcraft::BitLength>::BITS {
        !0 as u64
    } else {
        ((1 as u64) << 4) - 1
    };
    #[inline(always)]
    pub const fn new(val: u64) -> Self {
        Self(::bitcraft::reexport::portable_atomic::AtomicU64::new(val))
    }
    #[inline(always)]
    pub fn load(&self, order: ::bitcraft::Ordering) -> u64 {
        self.0.load(order)
    }
    #[inline(always)]
    pub fn store(&self, val: u64, order: ::bitcraft::Ordering) {
        self.0.store(val, order);
    }
    #[inline]
    pub fn get(&self, index: usize, order: ::bitcraft::Ordering) -> u128 {
        if true {
            if !(index < 16) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("atomic_bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 4;
        let raw = (self.0.load(order) >> shift) & Self::MASK;
        raw as u128
    }
    #[inline]
    pub fn set(&self, index: usize, value: u128, order: ::bitcraft::Ordering) {
        if true {
            if !(index < 16) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("atomic_bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 4;
        let val_masked = (value as u64 & Self::MASK) << shift;
        let mask = !(Self::MASK << shift);
        self.0
            .fetch_update(
                order,
                ::bitcraft::Ordering::Relaxed,
                |raw| { Some((raw & mask) | val_masked) },
            )
            .unwrap();
    }
}

#[repr(transparent)]
pub struct AtomicNibblesValue(pub u64);

#[automatically_derived]
impl ::core::marker::Copy for AtomicNibblesValue {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AtomicNibblesValue {}

#[automatically_derived]
impl ::core::clone::Clone for AtomicNibblesValue {
    #[inline]
    fn clone(&self) -> AtomicNibblesValue {
        let _: ::core::clone::AssertParamIsClone<u64>;
        *self
    }
}

#[automatically_derived]
impl ::core::default::Default for AtomicNibblesValue {
    #[inline]
    fn default() -> AtomicNibblesValue {
        AtomicNibblesValue(::core::default::Default::default())
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AtomicNibblesValue {}

#[automatically_derived]
impl ::core::cmp::PartialEq for AtomicNibblesValue {
    #[inline]
    fn eq(&self, other: &AtomicNibblesValue) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for AtomicNibblesValue {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u64>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for AtomicNibblesValue {
    #[inline]
    fn partial_cmp(
        &self,
        other: &AtomicNibblesValue,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for AtomicNibblesValue {
    #[inline]
    fn cmp(&self, other: &AtomicNibblesValue) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for AtomicNibblesValue {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl core::fmt::Debug for AtomicNibblesValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..16 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl AtomicNibblesValue {
    pub const ELEMENT_WIDTH: usize = 4;
    pub const ELEMENT_COUNT: usize = 16;
    pub const TOTAL_BITS: usize = 4 * 16;
    pub const MASK: u64 = if 4 >= <u64 as ::bitcraft::BitLength>::BITS {
        !0 as u64
    } else {
        ((1 as u64) << 4) - 1
    };
    #[inline(always)]
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u64 {
        self.0
    }
    #[inline]
    pub fn get(&self, index: usize) -> u128 {
        if true {
            if !(index < 16) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 4;
        let raw = (self.0 >> shift) & Self::MASK;
        raw as u128
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: u128) {
        if true {
            if !(index < 16) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 4;
        let val_masked = value as u64 & Self::MASK;
        self.0 &= !(Self::MASK << shift);
        self.0 |= val_masked << shift;
    }
}

impl AtomicNibbles {
    /// Returns a non-atomic snapshot of the current state.
    #[inline]
    pub fn get_snapshot(&self, order: ::bitcraft::Ordering) -> AtomicNibblesValue {
        AtomicNibblesValue::new(self.0.load(order))
    }
    /// Completely overwrites the entire atomic state with the given snapshot.
    #[inline]
    pub fn set_snapshot(&self, val: AtomicNibblesValue, order: ::bitcraft::Ordering) {
        self.0.store(val.to_bits(), order);
    }
    /// Atomically updates the array using a CAS loop.
    #[inline]
    pub fn update<F>(
        &self,
        set_order: ::bitcraft::Ordering,
        fetch_order: ::bitcraft::Ordering,
        mut f: F,
    ) -> AtomicNibblesValue
    where
        F: FnMut(&mut AtomicNibblesValue),
    {
        let raw_prev = self
            .0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = AtomicNibblesValue::new(raw);
                    f(&mut snap);
                    Some(snap.to_bits())
                },
            )
            .unwrap();
        AtomicNibblesValue::new(raw_prev)
    }
    /// Conditionally updates the array using a CAS loop.
    #[inline]
    pub fn update_or_abort<F>(
        &self,
        set_order: ::bitcraft::Ordering,
        fetch_order: ::bitcraft::Ordering,
        mut f: F,
    ) -> Result<AtomicNibblesValue, AtomicNibblesValue>
    where
        F: FnMut(&mut AtomicNibblesValue) -> Option<()>,
    {
        self.0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = AtomicNibblesValue::new(raw);
                    f(&mut snap).map(|_| snap.to_bits())
                },
            )
            .map(|raw| AtomicNibblesValue::new(raw))
            .map_err(|raw| AtomicNibblesValue::new(raw))
    }
}

#[repr(transparent)]
pub struct AtomicFlags128(pub ::bitcraft::reexport::portable_atomic::AtomicU128);

impl core::fmt::Debug for AtomicFlags128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..128 {
            list.entry(&self.get(i, ::bitcraft::Ordering::Relaxed));
        }
        list.finish()
    }
}

impl AtomicFlags128 {
    pub const ELEMENT_WIDTH: usize = 1;
    pub const ELEMENT_COUNT: usize = 128;
    pub const TOTAL_BITS: usize = 1 * 128;
    pub const MASK: u128 = if 1 >= <u128 as ::bitcraft::BitLength>::BITS {
        !0 as u128
    } else {
        ((1 as u128) << 1) - 1
    };
    #[inline(always)]
    pub const fn new(val: u128) -> Self {
        Self(::bitcraft::reexport::portable_atomic::AtomicU128::new(val))
    }
    #[inline(always)]
    pub fn load(&self, order: ::bitcraft::Ordering) -> u128 {
        self.0.load(order)
    }
    #[inline(always)]
    pub fn store(&self, val: u128, order: ::bitcraft::Ordering) {
        self.0.store(val, order);
    }
    #[inline]
    pub fn get(&self, index: usize, order: ::bitcraft::Ordering) -> bool {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("atomic_bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let raw = (self.0.load(order) >> shift) & Self::MASK;
        raw != 0
    }
    #[inline]
    pub fn set(&self, index: usize, value: bool, order: ::bitcraft::Ordering) {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("atomic_bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let val_masked = (if value { 1 } else { 0 } & Self::MASK) << shift;
        let mask = !(Self::MASK << shift);
        self.0
            .fetch_update(
                order,
                ::bitcraft::Ordering::Relaxed,
                |raw| { Some((raw & mask) | val_masked) },
            )
            .unwrap();
    }
}

#[repr(transparent)]
pub struct AtomicFlags128Value(pub u128);

#[automatically_derived]
impl ::core::marker::Copy for AtomicFlags128Value {}

#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AtomicFlags128Value {}

#[automatically_derived]
impl ::core::clone::Clone for AtomicFlags128Value {
    #[inline]
    fn clone(&self) -> AtomicFlags128Value {
        let _: ::core::clone::AssertParamIsClone<u128>;
        *self
    }
}

#[automatically_derived]
impl ::core::default::Default for AtomicFlags128Value {
    #[inline]
    fn default() -> AtomicFlags128Value {
        AtomicFlags128Value(::core::default::Default::default())
    }
}

#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AtomicFlags128Value {}

#[automatically_derived]
impl ::core::cmp::PartialEq for AtomicFlags128Value {
    #[inline]
    fn eq(&self, other: &AtomicFlags128Value) -> bool {
        self.0 == other.0
    }
}

#[automatically_derived]
impl ::core::cmp::Eq for AtomicFlags128Value {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u128>;
    }
}

#[automatically_derived]
impl ::core::cmp::PartialOrd for AtomicFlags128Value {
    #[inline]
    fn partial_cmp(
        &self,
        other: &AtomicFlags128Value,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::cmp::Ord for AtomicFlags128Value {
    #[inline]
    fn cmp(&self, other: &AtomicFlags128Value) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

#[automatically_derived]
impl ::core::hash::Hash for AtomicFlags128Value {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}

impl core::fmt::Debug for AtomicFlags128Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..128 {
            list.entry(&self.get(i));
        }
        list.finish()
    }
}

impl AtomicFlags128Value {
    pub const ELEMENT_WIDTH: usize = 1;
    pub const ELEMENT_COUNT: usize = 128;
    pub const TOTAL_BITS: usize = 1 * 128;
    pub const MASK: u128 = if 1 >= <u128 as ::bitcraft::BitLength>::BITS {
        !0 as u128
    } else {
        ((1 as u128) << 1) - 1
    };
    #[inline(always)]
    pub const fn new(val: u128) -> Self {
        Self(val)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u128 {
        self.0
    }
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let raw = (self.0 >> shift) & Self::MASK;
        raw != 0
    }
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        if true {
            if !(index < 128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("bitarray index out of bounds"),
                    );
                }
            }
        }
        let shift = index * 1;
        let val_masked = if value { 1 } else { 0 } & Self::MASK;
        self.0 &= !(Self::MASK << shift);
        self.0 |= val_masked << shift;
    }
}

impl AtomicFlags128 {
    /// Returns a non-atomic snapshot of the current state.
    #[inline]
    pub fn get_snapshot(&self, order: ::bitcraft::Ordering) -> AtomicFlags128Value {
        AtomicFlags128Value::new(self.0.load(order))
    }
    /// Completely overwrites the entire atomic state with the given snapshot.
    #[inline]
    pub fn set_snapshot(&self, val: AtomicFlags128Value, order: ::bitcraft::Ordering) {
        self.0.store(val.to_bits(), order);
    }
    /// Atomically updates the array using a CAS loop.
    #[inline]
    pub fn update<F>(
        &self,
        set_order: ::bitcraft::Ordering,
        fetch_order: ::bitcraft::Ordering,
        mut f: F,
    ) -> AtomicFlags128Value
    where
        F: FnMut(&mut AtomicFlags128Value),
    {
        let raw_prev = self
            .0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = AtomicFlags128Value::new(raw);
                    f(&mut snap);
                    Some(snap.to_bits())
                },
            )
            .unwrap();
        AtomicFlags128Value::new(raw_prev)
    }
    /// Conditionally updates the array using a CAS loop.
    #[inline]
    pub fn update_or_abort<F>(
        &self,
        set_order: ::bitcraft::Ordering,
        fetch_order: ::bitcraft::Ordering,
        mut f: F,
    ) -> Result<AtomicFlags128Value, AtomicFlags128Value>
    where
        F: FnMut(&mut AtomicFlags128Value) -> Option<()>,
    {
        self.0
            .fetch_update(
                set_order,
                fetch_order,
                |raw| {
                    let mut snap = AtomicFlags128Value::new(raw);
                    f(&mut snap).map(|_| snap.to_bits())
                },
            )
            .map(|raw| AtomicFlags128Value::new(raw))
            .map_err(|raw| AtomicFlags128Value::new(raw))
    }
}

