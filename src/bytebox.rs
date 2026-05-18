/// A declarative macro for generating packed arrays backed by a heap-allocated `Box<[u8]>`.
///
/// Requires the `alloc` feature to be enabled.
/// This macro generates a strictly fixed-size array on the heap.
/// Example: `pub struct Name(u 4, 128);`
///
/// Under the hood, this macro automatically calls `byteslice!` to generate
/// the zero-copy companion types `NameSlice<'a>` and `NameSliceMut<'a>`.
#[macro_export]
macro_rules! bytebox {
    ($vis:vis struct $name:ident (bool);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](bool); } }
        $crate::__bytebox_runtime_core!($vis, $name, bool, 1, unsigned, bool_type);
    };
    ($vis:vis struct $name:ident (u $width:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](u $width); } }
        $crate::__bytebox_runtime_core!($vis, $name, u128, $width, unsigned, int_type);
    };
    ($vis:vis struct $name:ident (i $width:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](i $width); } }
        $crate::__bytebox_runtime_core!($vis, $name, i128, $width, signed, int_type);
    };
    ($vis:vis struct $name:ident (bool, $count:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](bool); } }
        $crate::__bytebox_core!($vis, $name, bool, 1, unsigned, bool_type, $count);
    };
    ($vis:vis struct $name:ident (u $width:literal, $count:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](u $width); } }
        $crate::__bytebox_core!($vis, $name, u128, $width, unsigned, int_type, $count);
    };
    ($vis:vis struct $name:ident (i $width:literal, $count:literal);) => {
        $crate::paste::paste! { $crate::byteslice! { $vis struct [< $name Slice >](i $width); } }
        $crate::__bytebox_core!($vis, $name, i128, $width, signed, int_type, $count);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bytebox_runtime_core {
    ($vis:vis, $name:ident, $elem_ty:ty, $width:literal, $signed_tag:ident, $type_tag:ident) => {

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name {
            pub data: $crate::alloc::boxed::Box<[u8]>,
            pub len: usize,
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
                pub fn new(len: usize) -> Self {
                    let bytes = (len * $width + 7) / 8;
                    let vec = $crate::alloc::vec![0; bytes];
                    Self {
                        data: vec.into_boxed_slice(),
                        len,
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

                /// Returns a zero-copy read-only view of this box.
                #[inline]
                pub fn as_slice(&self) -> [< $name Slice >]<'_> {
                    [< $name Slice >]::new(&self.data, self.len)
                }

                /// Returns a zero-copy mutable view of this box.
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

                /// Returns an iterator over the logical elements of this box.
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

#[macro_export]
#[doc(hidden)]
macro_rules! __bytebox_core {
    ($vis:vis, $name:ident, $elem_ty:ty, $width:literal, $signed_tag:ident, $type_tag:ident, $count:literal) => {

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name {
            pub data: $crate::alloc::boxed::Box<[u8]>,
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
                for i in 0..Self::ELEMENTS { list.entry(&self.get(i)); }
                list.finish()
            }
        }

        $crate::paste::paste! {
            impl $name {
                pub const ELEMENT_WIDTH: usize = $width;
                pub const ELEMENTS: usize = $count;
                pub const BYTES: usize = ($count * $width + 7) / 8;

                #[inline]
                pub fn new() -> Self {
                    let vec = $crate::alloc::vec![0; Self::BYTES];
                    Self {
                        data: vec.into_boxed_slice(),
                    }
                }

                #[inline]
                pub fn len(&self) -> usize {
                    Self::ELEMENTS
                }

                #[inline]
                pub fn is_empty(&self) -> bool {
                    Self::ELEMENTS == 0
                }

                /// Returns a zero-copy read-only view of this box.
                #[inline]
                pub fn as_slice(&self) -> [< $name Slice >]<'_> {
                    [< $name Slice >]::new(&self.data, Self::ELEMENTS)
                }

                /// Returns a zero-copy mutable view of this box.
                #[inline]
                pub fn as_mut_slice(&mut self) -> [< $name SliceMut >]<'_> {
                    [< $name SliceMut >]::new(&mut self.data, Self::ELEMENTS)
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

                /// Returns an iterator over the logical elements of this box.
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
