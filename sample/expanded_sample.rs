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
        if true {
            if !((val.to_bits() as i16) <= Self::STATUS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Enum variant overflows allocated {0} bits", 2),
                    );
                }
            }
        }
        let val_masked = (val.to_bits() as i16) & Self::STATUS_MASK;
        self.0 = (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
            | (val_masked << Self::STATUS_OFFSET);
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bitfield bounded by the `Status` enumeration supplied to `status`.
    pub const fn with_status(self, val: Status) -> Self {
        if true {
            if !((val.to_bits() as i16) <= Self::STATUS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Enum variant overflows allocated bits"),
                    );
                }
            }
        }
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
        if (val.to_bits() as i16) > Self::STATUS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val.to_bits() as u128,
                allocated_bits: 2,
            });
        }
        let val_masked = (val.to_bits() as i16) & Self::STATUS_MASK;
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
        if (val.to_bits() as i16) > Self::STATUS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val.to_bits() as u128,
                allocated_bits: 2,
            });
        }
        let val_masked = (val.to_bits() as i16) & Self::STATUS_MASK;
        Ok(
            Self(
                (self.0 & !(Self::STATUS_MASK << Self::STATUS_OFFSET))
                    | (val_masked << Self::STATUS_OFFSET),
            ),
        )
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
        if (val.to_bits() as u128) > Self::STATUS_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val.to_bits() as u128,
                allocated_bits: 2,
            });
        }
        self.set_status(val);
        Ok(())
    }
    #[allow(dead_code)]
    pub const fn try_with_status(
        self,
        val: Status,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val.to_bits() as u128) > Self::STATUS_MASK as u128 {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val.to_bits() as u128,
                allocated_bits: 2,
            });
        }
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

