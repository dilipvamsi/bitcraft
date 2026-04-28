/// A declarative macro for generating packed arrays backed by a single unsigned integer.
///
/// Automatically selects the **smallest** integer base (`u8`→`u128`) for the given
/// element width and count. Pattern matching happens at the top-level call site where
/// all tokens are still fresh literals — this is the only way to do automatic type
/// selection in `macro_rules!` without a proc-macro.
///
/// # Supported element types
/// - `u N` — unsigned N-bit integer
/// - `i N` — signed N-bit integer (sign-extended on `get`)
/// - `bool` — single bit, returned as `bool`
#[macro_export]
macro_rules! bitarray {
    // =========================================================================
    // bool (1 bit) → choose smallest uX that holds $count bits
    // =========================================================================
    ($vis:vis struct $name:ident (bool,  1);) => { $crate::__bitarray_final!($vis,$name,u8,  bool,1, 1,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool,  2);) => { $crate::__bitarray_final!($vis,$name,u8,  bool,1, 2,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool,  4);) => { $crate::__bitarray_final!($vis,$name,u8,  bool,1, 4,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool,  8);) => { $crate::__bitarray_final!($vis,$name,u8,  bool,1, 8,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool, 16);) => { $crate::__bitarray_final!($vis,$name,u16, bool,1,16,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool, 32);) => { $crate::__bitarray_final!($vis,$name,u32, bool,1,32,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool, 64);) => { $crate::__bitarray_final!($vis,$name,u64, bool,1,64,unsigned,bool_type); };
    ($vis:vis struct $name:ident (bool,128);) => { $crate::__bitarray_final!($vis,$name,u128,bool,1,128,unsigned,bool_type); };
    // fallback bool
    ($vis:vis struct $name:ident (bool, $count:literal);) => {
        const _: () = { assert!($count <= 128, "bitarray bool: count exceeds 128"); };
        $crate::__bitarray_final!($vis,$name,u128,bool,1,$count,unsigned,bool_type);
    };

    // =========================================================================
    // Width 1 (individual bits as u/i)
    // =========================================================================
    ($vis:vis struct $name:ident (u 1,  1);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,1, 1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1,  2);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,1, 2,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1,  4);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,1, 4,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1,  8);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,1, 8,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1, 16);) => { $crate::__bitarray_final!($vis,$name,u16, u128,1,16,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1, 32);) => { $crate::__bitarray_final!($vis,$name,u32, u128,1,32,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1, 64);) => { $crate::__bitarray_final!($vis,$name,u64, u128,1,64,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 1,128);) => { $crate::__bitarray_final!($vis,$name,u128,u128,1,128,unsigned,int_type); };

    // =========================================================================
    // Width 2
    // =========================================================================
    ($vis:vis struct $name:ident (u 2, 1);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,2, 1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 2, 2);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,2, 2,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 2, 4);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,2, 4,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 2, 8);) => { $crate::__bitarray_final!($vis,$name,u16, u128,2, 8,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 2,16);) => { $crate::__bitarray_final!($vis,$name,u32, u128,2,16,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 2,32);) => { $crate::__bitarray_final!($vis,$name,u64, u128,2,32,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 2,64);) => { $crate::__bitarray_final!($vis,$name,u128,u128,2,64,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 2, 1);) => { $crate::__bitarray_final!($vis,$name,u8,  i128,2, 1,signed,int_type); };
    ($vis:vis struct $name:ident (i 2, 2);) => { $crate::__bitarray_final!($vis,$name,u8,  i128,2, 2,signed,int_type); };
    ($vis:vis struct $name:ident (i 2, 4);) => { $crate::__bitarray_final!($vis,$name,u8,  i128,2, 4,signed,int_type); };
    ($vis:vis struct $name:ident (i 2, 8);) => { $crate::__bitarray_final!($vis,$name,u16, i128,2, 8,signed,int_type); };
    ($vis:vis struct $name:ident (i 2,16);) => { $crate::__bitarray_final!($vis,$name,u32, i128,2,16,signed,int_type); };
    ($vis:vis struct $name:ident (i 2,32);) => { $crate::__bitarray_final!($vis,$name,u64, i128,2,32,signed,int_type); };
    ($vis:vis struct $name:ident (i 2,64);) => { $crate::__bitarray_final!($vis,$name,u128,i128,2,64,signed,int_type); };

    // =========================================================================
    // Width 4 (nibbles)
    // =========================================================================
    ($vis:vis struct $name:ident (u 4, 1);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,4, 1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 4, 2);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,4, 2,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 4, 4);) => { $crate::__bitarray_final!($vis,$name,u16, u128,4, 4,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 4, 8);) => { $crate::__bitarray_final!($vis,$name,u32, u128,4, 8,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 4,16);) => { $crate::__bitarray_final!($vis,$name,u64, u128,4,16,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 4,32);) => { $crate::__bitarray_final!($vis,$name,u128,u128,4,32,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 4, 1);) => { $crate::__bitarray_final!($vis,$name,u8,  i128,4, 1,signed,int_type); };
    ($vis:vis struct $name:ident (i 4, 2);) => { $crate::__bitarray_final!($vis,$name,u8,  i128,4, 2,signed,int_type); };
    ($vis:vis struct $name:ident (i 4, 4);) => { $crate::__bitarray_final!($vis,$name,u16, i128,4, 4,signed,int_type); };
    ($vis:vis struct $name:ident (i 4, 8);) => { $crate::__bitarray_final!($vis,$name,u32, i128,4, 8,signed,int_type); };
    ($vis:vis struct $name:ident (i 4,16);) => { $crate::__bitarray_final!($vis,$name,u64, i128,4,16,signed,int_type); };
    ($vis:vis struct $name:ident (i 4,32);) => { $crate::__bitarray_final!($vis,$name,u128,i128,4,32,signed,int_type); };

    // =========================================================================
    // Width 8 (bytes)
    // =========================================================================
    ($vis:vis struct $name:ident (u 8, 1);) => { $crate::__bitarray_final!($vis,$name,u8,  u128,8, 1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 8, 2);) => { $crate::__bitarray_final!($vis,$name,u16, u128,8, 2,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 8, 4);) => { $crate::__bitarray_final!($vis,$name,u32, u128,8, 4,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 8, 8);) => { $crate::__bitarray_final!($vis,$name,u64, u128,8, 8,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 8,16);) => { $crate::__bitarray_final!($vis,$name,u128,u128,8,16,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 8, 1);) => { $crate::__bitarray_final!($vis,$name,u8,  i128,8, 1,signed,int_type); };
    ($vis:vis struct $name:ident (i 8, 2);) => { $crate::__bitarray_final!($vis,$name,u16, i128,8, 2,signed,int_type); };
    ($vis:vis struct $name:ident (i 8, 4);) => { $crate::__bitarray_final!($vis,$name,u32, i128,8, 4,signed,int_type); };
    ($vis:vis struct $name:ident (i 8, 8);) => { $crate::__bitarray_final!($vis,$name,u64, i128,8, 8,signed,int_type); };
    ($vis:vis struct $name:ident (i 8,16);) => { $crate::__bitarray_final!($vis,$name,u128,i128,8,16,signed,int_type); };

    // =========================================================================
    // Width 16
    // =========================================================================
    ($vis:vis struct $name:ident (u 16, 1);) => { $crate::__bitarray_final!($vis,$name,u16, u128,16, 1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 16, 2);) => { $crate::__bitarray_final!($vis,$name,u32, u128,16, 2,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 16, 4);) => { $crate::__bitarray_final!($vis,$name,u64, u128,16, 4,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 16, 8);) => { $crate::__bitarray_final!($vis,$name,u128,u128,16, 8,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 16, 1);) => { $crate::__bitarray_final!($vis,$name,u16, i128,16, 1,signed,int_type); };
    ($vis:vis struct $name:ident (i 16, 2);) => { $crate::__bitarray_final!($vis,$name,u32, i128,16, 2,signed,int_type); };
    ($vis:vis struct $name:ident (i 16, 4);) => { $crate::__bitarray_final!($vis,$name,u64, i128,16, 4,signed,int_type); };
    ($vis:vis struct $name:ident (i 16, 8);) => { $crate::__bitarray_final!($vis,$name,u128,i128,16, 8,signed,int_type); };

    // =========================================================================
    // Width 32
    // =========================================================================
    ($vis:vis struct $name:ident (u 32, 1);) => { $crate::__bitarray_final!($vis,$name,u32, u128,32,1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 32, 2);) => { $crate::__bitarray_final!($vis,$name,u64, u128,32,2,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 32, 4);) => { $crate::__bitarray_final!($vis,$name,u128,u128,32,4,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 32, 1);) => { $crate::__bitarray_final!($vis,$name,u32, i128,32,1,signed,int_type); };
    ($vis:vis struct $name:ident (i 32, 2);) => { $crate::__bitarray_final!($vis,$name,u64, i128,32,2,signed,int_type); };
    ($vis:vis struct $name:ident (i 32, 4);) => { $crate::__bitarray_final!($vis,$name,u128,i128,32,4,signed,int_type); };

    // =========================================================================
    // Width 64
    // =========================================================================
    ($vis:vis struct $name:ident (u 64, 1);) => { $crate::__bitarray_final!($vis,$name,u64, u128,64,1,unsigned,int_type); };
    ($vis:vis struct $name:ident (u 64, 2);) => { $crate::__bitarray_final!($vis,$name,u128,u128,64,2,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 64, 1);) => { $crate::__bitarray_final!($vis,$name,u64, i128,64,1,signed,int_type); };
    ($vis:vis struct $name:ident (i 64, 2);) => { $crate::__bitarray_final!($vis,$name,u128,i128,64,2,signed,int_type); };

    // =========================================================================
    // Width 128
    // =========================================================================
    ($vis:vis struct $name:ident (u 128, 1);) => { $crate::__bitarray_final!($vis,$name,u128,u128,128,1,unsigned,int_type); };
    ($vis:vis struct $name:ident (i 128, 1);) => { $crate::__bitarray_final!($vis,$name,u128,i128,128,1,signed,int_type); };

    // =========================================================================
    // General fallback — u128 for any other combination (assert guards ≤128 bits)
    // =========================================================================
    ($vis:vis struct $name:ident (u $width:literal, $count:literal);) => {
        const _: () = { assert!($width * $count <= 128, "bitarray: total bits exceed 128-bit limit"); };
        $crate::__bitarray_final!($vis,$name,u128,u128,$width,$count,unsigned,int_type);
    };
    ($vis:vis struct $name:ident (i $width:literal, $count:literal);) => {
        const _: () = { assert!($width * $count <= 128, "bitarray: total bits exceed 128-bit limit"); };
        $crate::__bitarray_final!($vis,$name,u128,i128,$width,$count,signed,int_type);
    };
}

// ---------------------------------------------------------------------------
// Core struct + impl generator
// ---------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __bitarray_final {
    ($vis:vis, $name:ident, $base_ty:ty, $elem_ty:ty, $width:literal, $count:literal, $signed_tag:ident, $type_tag:ident) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
                 bytemuck::Pod, bytemuck::Zeroable)]
        #[repr(transparent)]
        $vis struct $name(pub $base_ty);

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
            pub const MASK: $base_ty = if $width == <$base_ty as $crate::BitLength>::BITS {
                !0 as $base_ty
            } else {
                ((1 as $base_ty) << $width) - 1
            };

            #[inline(always)] pub const fn new(val: $base_ty) -> Self { Self(val) }
            #[inline(always)] pub const fn to_bits(self) -> $base_ty { self.0 }

            #[inline]
            pub fn get(&self, index: usize) -> $elem_ty {
                debug_assert!(index < $count, "bitarray index out of bounds");
                let shift = index * $width;
                let raw   = (self.0 >> shift) & Self::MASK;
                $crate::__bitarray_cast_get!(raw, $elem_ty, $width, $signed_tag, $type_tag)
            }

            #[inline]
            pub fn set(&mut self, index: usize, value: $elem_ty) {
                debug_assert!(index < $count, "bitarray index out of bounds");
                let shift      = index * $width;
                let val_masked = $crate::__bitarray_cast_set!(value, $base_ty, $type_tag) & Self::MASK;
                self.0 &= !(Self::MASK << shift);
                self.0 |=  val_masked << shift;
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Casting helpers — use ident tags, not literal booleans
// ---------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __bitarray_cast_get {
    // bool: compare to zero
    ($raw:expr, $elem_ty:ty, $width:literal, $signed_tag:ident, bool_type) => {
        $raw != 0
    };
    // signed int: arithmetic sign extension
    ($raw:expr, $elem_ty:ty, $width:literal, signed, int_type) => {{
        let val = $raw as $elem_ty;
        let shift = (core::mem::size_of::<$elem_ty>() * 8) - $width;
        (val << shift) >> shift
    }};
    // unsigned int: zero-extend (just cast)
    ($raw:expr, $elem_ty:ty, $width:literal, unsigned, int_type) => {
        $raw as $elem_ty
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bitarray_cast_set {
    ($value:expr, $base_ty:ty, bool_type) => {
        if $value { 1 } else { 0 }
    };
    ($value:expr, $base_ty:ty, int_type) => {
        $value as $base_ty
    };
}
