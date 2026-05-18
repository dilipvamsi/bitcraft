/// A declarative macro for generating packed arrays backed by a heap-allocated `Vec<u8>`.
///
/// Requires the `alloc` feature to be enabled.
/// This macro provides two forms of initialization:
/// 1. `pub struct Name(u 4);` — A dynamically sized vector that starts empty.
/// 2. `pub struct Name(u 4, 128);` — A dynamically sized vector where `new()` pre-allocates capacity for 128 elements.
///
/// Under the hood, this macro automatically calls `byteslice!` to generate
/// the zero-copy companion types `NameSlice<'a>` and `NameSliceMut<'a>`.
#[macro_export]
macro_rules! bytevec {
    // Variable size, no default capacity
    ($vis:vis struct $name:ident (bool);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](bool); } }
        $crate::__bytevec_core!($vis, $name, bool, 1, unsigned, bool_type, 0);
    };
    ($vis:vis struct $name:ident (u $width:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](u $width); } }
        $crate::__bytevec_core!($vis, $name, u128, $width, unsigned, int_type, 0);
    };
    ($vis:vis struct $name:ident (i $width:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](i $width); } }
        $crate::__bytevec_core!($vis, $name, i128, $width, signed, int_type, 0);
    };

    // Variable size, with default capacity
    ($vis:vis struct $name:ident (bool, $cap:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](bool); } }
        $crate::__bytevec_core!($vis, $name, bool, 1, unsigned, bool_type, $cap);
    };
    ($vis:vis struct $name:ident (u $width:literal, $cap:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](u $width); } }
        $crate::__bytevec_core!($vis, $name, u128, $width, unsigned, int_type, $cap);
    };
    ($vis:vis struct $name:ident (i $width:literal, $cap:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](i $width); } }
        $crate::__bytevec_core!($vis, $name, i128, $width, signed, int_type, $cap);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bytevec_core {
    ($vis:vis, $name:ident, $elem_ty:ty, $width:literal, $signed_tag:ident, $type_tag:ident, $default_cap:literal) => {

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name {
            pub data: $crate::alloc::vec::Vec<u8>,
            pub len: usize,
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut list = f.debug_list();
                for i in 0..self.len { list.entry(&self.get(i)); }
                list.finish()
            }
        }

        $crate::paste::paste! {
            impl $name {
                pub const ELEMENT_WIDTH: usize = $width;

                #[inline]
                pub fn new() -> Self {
                    if $default_cap > 0 {
                        Self::with_capacity($default_cap)
                    } else {
                        Self {
                            data: $crate::alloc::vec::Vec::new(),
                            len: 0,
                        }
                    }
                }

                #[inline]
                pub fn with_capacity(capacity: usize) -> Self {
                    let byte_cap = (capacity * $width + 7) / 8;
                    Self {
                        data: $crate::alloc::vec::Vec::with_capacity(byte_cap),
                        len: 0,
                    }
                }

                #[inline]
                pub fn len(&self) -> usize {
                    self.len
                }

                #[inline]
                pub fn is_empty(&self) -> bool {
                    self.len == 0
                }

                #[inline]
                pub fn clear(&mut self) {
                    self.data.clear();
                    self.len = 0;
                }

                #[inline]
                pub fn capacity(&self) -> usize {
                    (self.data.capacity() * 8) / $width
                }

                #[inline]
                pub fn push(&mut self, value: $elem_ty) {
                    let bit_offset = self.len * $width;
                    let required_bytes = (bit_offset + $width + 7) / 8;

                    while self.data.len() < required_bytes {
                        self.data.push(0);
                    }

                    self.len += 1;
                    self.set(self.len - 1, value);
                }

                #[inline]
                pub fn pop(&mut self) -> Option<$elem_ty> {
                    if self.len == 0 {
                        return None;
                    }

                    let value = self.get(self.len - 1);

                    self.len -= 1;
                    let required_bytes = (self.len * $width + 7) / 8;
                    self.data.truncate(required_bytes);

                    Some(value)
                }

                /// Returns a zero-copy read-only view of this vector.
                #[inline]
                pub fn as_slice(&self) -> [< $name Slice >]<'_> {
                    [< $name Slice >]::new(&self.data, self.len)
                }

                /// Returns a zero-copy mutable view of this vector.
                #[inline]
                pub fn as_mut_slice(&mut self) -> [< $name SliceMut >]<'_> {
                    [< $name SliceMut >]::new(&mut self.data, self.len)
                }

                #[inline]
                pub fn get(&self, index: usize) -> $elem_ty {
                    self.as_slice().get(index)
                }

                #[inline]
                pub fn set(&mut self, index: usize, value: $elem_ty) {
                    self.as_mut_slice().set(index, value)
                }

                #[inline]
                pub fn as_bytes(&self) -> &[u8] {
                    &self.data
                }

                #[inline]
                pub fn as_mut_bytes(&mut self) -> &mut [u8] {
                    &mut self.data
                }

                /// Returns an iterator over the logical elements of this vector.
                #[inline]
                pub fn iter(&self) -> [< $name Slice Iter >]<'_> {
                    self.into_iter()
                }
            }

            impl<'a> IntoIterator for &'a $name {
                type Item = $elem_ty;
                type IntoIter = [< $name Slice Iter >]<'a>;

                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    self.as_slice().into_iter()
                }
            }
        }
    };
}
