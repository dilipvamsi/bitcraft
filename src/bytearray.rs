/// A declarative macro for generating packed arrays backed by a fixed-size byte array.
///
/// Storage is automatically sized as `[u8; (width * count + 7) / 8]` — a const expression
/// evaluated at compile time to a concrete fixed size. No heap allocation.
///
/// # Supported element types
/// - `u N` — unsigned N-bit integer
/// - `i N` — signed N-bit integer (sign-extended on `get`)
/// - `bool` — single bit, returned as `bool`
///
/// # Examples
/// ```rust
/// use bitcraft::bytearray;
///
/// bytearray! { pub struct NibbleBuf(u 4, 64); }  // → [u8; 32]
/// bytearray! { pub struct Flags(bool, 128); }     // → [u8; 16]
/// ```
#[macro_export]
macro_rules! bytearray {
    // --- bool: width is always 1 ---
    ($vis:vis struct $name:ident (bool, $count:literal);) => {
        $crate::__bytearray_core!($vis, $name, bool, 1, $count, unsigned, bool_type);
    };
    // --- unsigned ---
    ($vis:vis struct $name:ident (u $width:literal, $count:literal);) => {
        $crate::__bytearray_core!($vis, $name, u128, $width, $count, unsigned, int_type);
    };
    // --- signed ---
    ($vis:vis struct $name:ident (i $width:literal, $count:literal);) => {
        $crate::__bytearray_core!($vis, $name, i128, $width, $count, signed, int_type);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bytearray_core {
    ($vis:vis, $name:ident, $elem_ty:ty, $width:literal, $count:literal, $signed_tag:ident, $type_tag:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
                 bytemuck::Pod, bytemuck::Zeroable)]
        #[repr(transparent)]
        $vis struct $name(pub [u8; ($width * $count + 7) / 8]);

        impl Default for $name {
            fn default() -> Self {
                Self([0u8; ($width * $count + 7) / 8])
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut list = f.debug_list();
                for i in 0..$count { list.entry(&self.get(i)); }
                list.finish()
            }
        }

        impl $name {
            pub const ELEMENT_WIDTH: usize = $width;
            pub const ELEMENT_COUNT: usize = $count;
            pub const TOTAL_BITS:    usize = $width * $count;
            pub const BYTES:         usize = ($width * $count + 7) / 8;

            const MASK: u128 = if $width == 128 { !0u128 } else { (1u128 << $width) - 1 };

            #[inline]
            pub fn get(&self, index: usize) -> $elem_ty {
                debug_assert!(index < $count, "bytearray index out of bounds");
                let bit_offset       = index * $width;
                let byte_idx         = bit_offset / 8;
                let inner_bit_offset = bit_offset % 8;
                let bytes_to_read    = ($width + inner_bit_offset + 7) / 8;

                let mut raw = 0u128;
                for i in 0..bytes_to_read {
                    if byte_idx + i < Self::BYTES {
                        raw |= (self.0[byte_idx + i] as u128) << (i * 8);
                    }
                }

                let extracted = (raw >> inner_bit_offset) & Self::MASK;
                $crate::__bitarray_cast_get!(extracted, $elem_ty, $width, $signed_tag, $type_tag)
            }

            #[inline]
            pub fn set(&mut self, index: usize, value: $elem_ty) {
                debug_assert!(index < $count, "bytearray index out of bounds");
                let bit_offset       = index * $width;
                let byte_idx         = bit_offset / 8;
                let inner_bit_offset = bit_offset % 8;

                let raw_val      = $crate::__bitarray_cast_set!(value, u128, $type_tag) & Self::MASK;
                let val_shifted  = raw_val   << inner_bit_offset;
                let mask_shifted = Self::MASK << inner_bit_offset;
                let bytes_to_modify = ($width + inner_bit_offset + 7) / 8;

                for i in 0..bytes_to_modify {
                    if byte_idx + i < Self::BYTES {
                        let byte_mask = ((mask_shifted >> (i * 8)) & 0xFF) as u8;
                        let byte_val  = ((val_shifted  >> (i * 8)) & 0xFF) as u8;
                        self.0[byte_idx + i] &= !byte_mask;
                        self.0[byte_idx + i] |=  byte_val;
                    }
                }
            }
        }
    };
}
