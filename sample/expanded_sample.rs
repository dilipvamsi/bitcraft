pub struct Status(pub <::bitcraft::Bits<2> as ::bitcraft::BitenumType>::Prim);

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

pub struct Config(pub u16);

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

pub struct Coordinate(pub [u8; 5]);

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
    /// Returns the underlying raw byte array.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_array(self) -> [u8; 5] {
        self.0
    }
    /// Creates a new instance from a raw byte array.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_array(arr: [u8; 5]) -> Self {
        Self(arr)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_u64(self) -> u64 {
        0 | (self.0[0] as u64) << (0 << 3) | (self.0[1] as u64) << (1 << 3)
            | (self.0[2] as u64) << (2 << 3) | (self.0[3] as u64) << (3 << 3)
            | (self.0[4] as u64) << (4 << 3)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_u64(val: u64) -> Self {
        Self([
            (val >> (0 << 3)) as u8,
            (val >> (1 << 3)) as u8,
            (val >> (2 << 3)) as u8,
            (val >> (3 << 3)) as u8,
            (val >> (4 << 3)) as u8,
        ])
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u64 {
        self.to_u64()
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(val: u64) -> Self {
        Self::from_u64(val)
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const X_OFFSET: usize = 0;
    #[doc(hidden)]
    const X_MASK: u128 = (!0u128) >> (128 - 16);
    #[allow(dead_code)]
    #[inline]
    ///Returns the `x` property as a `u16`.
    pub const fn x(self) -> u16 {
        let val = {
            const BYTE_INDEX: usize = 0 >> 3;
            const BIT_OFFSET: usize = 0 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::read_le_bits::<
                { 0 },
                { 16 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&self.0)
        };
        val as u16
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the `x` property. Masks inputs over 16 bits.
    pub fn set_x(&mut self, val: u16) {
        if true {
            if !((val as u128) <= Self::X_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 16),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 >> 3;
            const BIT_OFFSET: usize = 0 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 },
                { 16 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bytestruct with the `x` property mapped. Masks inputs over 16 bits.
    pub const fn with_x(mut self, val: u16) -> Self {
        if true {
            if !((val as u128) <= Self::X_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 >> 3;
            const BIT_OFFSET: usize = 0 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 },
                { 16 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
        self
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `x` property. Returns a `BitstructError` if the value overflows 16 bits.
    pub fn try_set_x(&mut self, val: u16) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::X_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        self.set_x(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `x` property. Returns a `BitstructError` if the value overflows 16 bits.
    pub const fn try_with_x(self, val: u16) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::X_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        Ok(self.with_x(val))
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const Y_OFFSET: usize = 0 + 16;
    #[doc(hidden)]
    const Y_MASK: u128 = (!0u128) >> (128 - 16);
    #[allow(dead_code)]
    #[inline]
    ///Returns the `y` property as a `u16`.
    pub const fn y(self) -> u16 {
        let val = {
            const BYTE_INDEX: usize = 0 + 16 >> 3;
            const BIT_OFFSET: usize = 0 + 16 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 16 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::read_le_bits::<
                { 0 + 16 },
                { 16 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&self.0)
        };
        val as u16
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the `y` property. Masks inputs over 16 bits.
    pub fn set_y(&mut self, val: u16) {
        if true {
            if !((val as u128) <= Self::Y_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 16),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 + 16 >> 3;
            const BIT_OFFSET: usize = 0 + 16 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 16 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 + 16 },
                { 16 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bytestruct with the `y` property mapped. Masks inputs over 16 bits.
    pub const fn with_y(mut self, val: u16) -> Self {
        if true {
            if !((val as u128) <= Self::Y_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 + 16 >> 3;
            const BIT_OFFSET: usize = 0 + 16 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 16 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 + 16 },
                { 16 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
        self
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `y` property. Returns a `BitstructError` if the value overflows 16 bits.
    pub fn try_set_y(&mut self, val: u16) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::Y_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        self.set_y(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `y` property. Returns a `BitstructError` if the value overflows 16 bits.
    pub const fn try_with_y(self, val: u16) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::Y_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 16,
            });
        }
        Ok(self.with_y(val))
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const FLAGS_OFFSET: usize = 0 + 16 + 16;
    #[doc(hidden)]
    const FLAGS_MASK: u128 = (!0u128) >> (128 - 8);
    #[allow(dead_code)]
    #[inline]
    ///Returns the `flags` property as a `u8`.
    pub const fn flags(self) -> u8 {
        let val = {
            const BYTE_INDEX: usize = 0 + 16 + 16 >> 3;
            const BIT_OFFSET: usize = 0 + 16 + 16 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 16 + 8 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::read_le_bits::<
                { 0 + 16 + 16 },
                { 8 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&self.0)
        };
        val as u8
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the `flags` property. Masks inputs over 8 bits.
    pub fn set_flags(&mut self, val: u8) {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 8),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 + 16 + 16 >> 3;
            const BIT_OFFSET: usize = 0 + 16 + 16 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 16 + 8 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 + 16 + 16 },
                { 8 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bytestruct with the `flags` property mapped. Masks inputs over 8 bits.
    pub const fn with_flags(mut self, val: u8) -> Self {
        if true {
            if !((val as u128) <= Self::FLAGS_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 + 16 + 16 >> 3;
            const BIT_OFFSET: usize = 0 + 16 + 16 & 7;
            const BYTE_COUNT: usize = ((0 + 16 + 16 + 8 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 + 16 + 16 },
                { 8 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
        self
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `flags` property. Returns a `BitstructError` if the value overflows 8 bits.
    pub fn try_set_flags(&mut self, val: u8) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 8,
            });
        }
        self.set_flags(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `flags` property. Returns a `BitstructError` if the value overflows 8 bits.
    pub const fn try_with_flags(
        self,
        val: u8,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::FLAGS_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 8,
            });
        }
        Ok(self.with_flags(val))
    }
}

pub struct PackedId(pub [u8; 3]);

impl core::fmt::Debug for PackedId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PackedId")
            .field("raw", &self.0)
            .field("value", &self.value())
            .finish()
    }
}

impl PackedId {
    /// Returns the underlying raw byte array.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_array(self) -> [u8; 3] {
        self.0
    }
    /// Creates a new instance from a raw byte array.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_array(arr: [u8; 3]) -> Self {
        Self(arr)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_u32(self) -> u32 {
        0 | (self.0[0] as u32) << (0 << 3) | (self.0[1] as u32) << (1 << 3)
            | (self.0[2] as u32) << (2 << 3)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_u32(val: u32) -> Self {
        Self([(val >> (0 << 3)) as u8, (val >> (1 << 3)) as u8, (val >> (2 << 3)) as u8])
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn to_bits(self) -> u32 {
        self.to_u32()
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn from_bits(val: u32) -> Self {
        Self::from_u32(val)
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    const VALUE_OFFSET: usize = 0;
    #[doc(hidden)]
    const VALUE_MASK: u128 = (!0u128) >> (128 - 24);
    #[allow(dead_code)]
    #[inline]
    ///Returns the `value` property as a `u32`.
    pub const fn value(self) -> u32 {
        let val = {
            const BYTE_INDEX: usize = 0 >> 3;
            const BIT_OFFSET: usize = 0 & 7;
            const BYTE_COUNT: usize = ((0 + 24 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::read_le_bits::<
                { 0 },
                { 24 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&self.0)
        };
        val as u32
    }
    #[allow(dead_code)]
    ///Inline mutation to apply the `value` property. Masks inputs over 24 bits.
    pub fn set_value(&mut self, val: u32) {
        if true {
            if !((val as u128) <= Self::VALUE_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value {0} overflows allocated {1} bits", val, 24),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 >> 3;
            const BIT_OFFSET: usize = 0 & 7;
            const BYTE_COUNT: usize = ((0 + 24 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 },
                { 24 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
    }
    #[allow(dead_code)]
    ///Returns a cloned copy of the bytestruct with the `value` property mapped. Masks inputs over 24 bits.
    pub const fn with_value(mut self, val: u32) -> Self {
        if true {
            if !((val as u128) <= Self::VALUE_MASK) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Value overflows allocated bits"),
                    );
                }
            }
        }
        {
            const BYTE_INDEX: usize = 0 >> 3;
            const BIT_OFFSET: usize = 0 & 7;
            const BYTE_COUNT: usize = ((0 + 24 + 7) >> 3) - BYTE_INDEX;
            ::bitcraft::write_le_bits::<
                { 0 },
                { 24 },
                _,
                BYTE_INDEX,
                BIT_OFFSET,
                BYTE_COUNT,
            >(&mut self.0, val as u128)
        };
        self
    }
    #[allow(dead_code)]
    ///Strict inline mutation to apply the `value` property. Returns a `BitstructError` if the value overflows 24 bits.
    pub fn try_set_value(&mut self, val: u32) -> Result<(), ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 24,
            });
        }
        self.set_value(val);
        Ok(())
    }
    #[allow(dead_code)]
    ///Strict cloned evaluation to apply the `value` property. Returns a `BitstructError` if the value overflows 24 bits.
    pub const fn try_with_value(
        self,
        val: u32,
    ) -> Result<Self, ::bitcraft::BitstructError> {
        if (val as u128) > Self::VALUE_MASK {
            return Err(::bitcraft::BitstructError::Overflow {
                value: val as u128,
                allocated_bits: 24,
            });
        }
        Ok(self.with_value(val))
    }
}

