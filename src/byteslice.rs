/// A declarative macro for generating packed arrays backed by borrowed slices (`&[u8]`).
///
/// This does not require the `alloc` feature and is fully `no_std` compatible.
/// It is ideal for zero-copy memory overlays, such as mapping packed data directly from
/// disk (`mmap`) or network buffers.
///
/// Generates two structs:
/// 1. `Name<'a>` — A read-only slice.
/// 2. `NameMut<'a>` — A mutable slice.
///
/// # Supported element types
/// - `u N` — unsigned N-bit integer
/// - `i N` — signed N-bit integer (sign-extended on `get`)
/// - `bool` — single bit, returned as `bool`
#[macro_export]
macro_rules! byteslice {
    ($vis:vis struct $name:ident (bool);) => {
        $crate::__byteslice_core!($vis, $name, bool, 1, unsigned, bool_type);
    };
    ($vis:vis struct $name:ident (u $width:literal);) => {
        $crate::__byteslice_core!($vis, $name, u128, $width, unsigned, int_type);
    };
    ($vis:vis struct $name:ident (i $width:literal);) => {
        $crate::__byteslice_core!($vis, $name, i128, $width, signed, int_type);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __byteslice_core {
    ($vis:vis, $name:ident, $elem_ty:ty, $width:literal, $signed_tag:ident, $type_tag:ident) => {
        // ----------------------------------------------------------------
        // 1. Read-Only Slice Type
        // ----------------------------------------------------------------
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        $vis struct $name<'a> {
            pub data: &'a [u8],
            pub len: usize,
        }

        impl<'a> core::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut list = f.debug_list();
                for i in 0..self.len { list.entry(&self.get(i)); }
                list.finish()
            }
        }

        impl<'a> $name<'a> {
            pub const ELEMENT_WIDTH: usize = $width;
            const MASK: u128 = if $width == 128 { !0u128 } else { (1u128 << $width) - 1 };

            #[inline]
            pub fn new(data: &'a [u8], len: usize) -> Self {
                let required_bytes = (len * $width + 7) / 8;
                debug_assert!(data.len() >= required_bytes, "byteslice data is too small for the specified logical length");
                Self { data, len }
            }

            #[inline]
            pub fn len(&self) -> usize { self.len }

            #[inline]
            pub fn is_empty(&self) -> bool { self.len == 0 }

            #[inline]
            pub fn get(&self, index: usize) -> $elem_ty {
                $crate::__bytearray_get!(&self.data, self.data.len(), index, self.len, $elem_ty, $width, Self::MASK, $signed_tag, $type_tag)
            }

            #[inline]
            pub fn as_bytes(&self) -> &'a [u8] {
                self.data
            }
        }

        $crate::paste::paste! {
            pub struct [< $name Iter >]<'a> {
                slice: $name<'a>,
                index: usize,
            }

            impl<'a> Iterator for [< $name Iter >]<'a> {
                type Item = $elem_ty;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    if self.index < self.slice.len {
                        let val = self.slice.get(self.index);
                        self.index += 1;
                        Some(val)
                    } else {
                        None
                    }
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let remaining = self.slice.len - self.index;
                    (remaining, Some(remaining))
                }
            }

            impl<'a> ExactSizeIterator for [< $name Iter >]<'a> {}

            impl<'a> IntoIterator for $name<'a> {
                type Item = $elem_ty;
                type IntoIter = [< $name Iter >]<'a>;

                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    [< $name Iter >] {
                        slice: self,
                        index: 0,
                    }
                }
            }

            impl<'a> $name<'a> {
                #[inline]
                pub fn iter(&self) -> [< $name Iter >]<'a> {
                    self.into_iter()
                }
            }
        }

        // ----------------------------------------------------------------
        // 2. Mutable Slice Type
        // ----------------------------------------------------------------
        $crate::paste::paste! {
            #[derive(PartialEq, Eq, Hash)]
            $vis struct [< $name Mut >]<'a> {
                pub data: &'a mut [u8],
                pub len: usize,
            }

            impl<'a> core::fmt::Debug for [< $name Mut >]<'a> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    let mut list = f.debug_list();
                    for i in 0..self.len { list.entry(&self.get(i)); }
                    list.finish()
                }
            }

            impl<'a> [< $name Mut >]<'a> {
                pub const ELEMENT_WIDTH: usize = $width;
                const MASK: u128 = if $width == 128 { !0u128 } else { (1u128 << $width) - 1 };

                #[inline]
                pub fn new(data: &'a mut [u8], len: usize) -> Self {
                    let required_bytes = (len * $width + 7) / 8;
                    debug_assert!(data.len() >= required_bytes, "byteslice data is too small for the specified logical length");
                    Self { data, len }
                }

                #[inline]
                pub fn len(&self) -> usize { self.len }

                #[inline]
                pub fn is_empty(&self) -> bool { self.len == 0 }

                #[inline]
            pub fn get(&self, index: usize) -> $elem_ty {
                $crate::__bytearray_get!(&self.data, self.data.len(), index, self.len, $elem_ty, $width, Self::MASK, $signed_tag, $type_tag)
            }

            #[inline]
            pub fn set(&mut self, index: usize, value: $elem_ty) {
                $crate::__bytearray_set!(&mut self.data, self.data.len(), index, self.len, value, $elem_ty, $width, Self::MASK, $type_tag)
            }

                #[inline]
                pub fn as_bytes(&self) -> &[u8] {
                    self.data
                }

                #[inline]
                pub fn as_mut_bytes(&mut self) -> &mut [u8] {
                    self.data
                }
            }
        }
    };
}
