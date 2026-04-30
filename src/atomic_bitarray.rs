/// A declarative macro for generating atomic packed arrays backed by a single atomic integer.
///
/// Automatically selects the **smallest** atomic integer base (`AtomicU8`→`AtomicU128`)
/// for the given element width and count.
///
/// # Supported element types
/// - `bool` — single bit, returned as `bool`
/// - `u N` — unsigned N-bit integer
/// - `i N` — signed N-bit integer (sign-extended on `get`)
#[macro_export]
macro_rules! atomic_bitarray {
    // =========================================================================
    // bool (1 bit) → choose smallest AtomicUX that holds $count bits
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool,  1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  bool,1, 1,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool,  2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  bool,1, 2,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool,  4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  bool,1, 4,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool,  8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  bool,1, 8,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool, 16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, bool,1,16,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool, 32);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, bool,1,32,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool, 64);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, bool,1,64,unsigned,bool_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool,128);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,bool,1,128,unsigned,bool_type); };
    // fallback bool
    ($(#[$meta:meta])* $vis:vis struct $name:ident (bool, $count:literal);) => {
        const _: () = { assert!($count <= 128, "atomic_bitarray bool: count exceeds 128"); };
        $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,bool,1,$count,unsigned,bool_type);
    };

    // =========================================================================
    // Width 1 (individual bits as u/i)
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1,  1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,1, 1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1,  2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,1, 2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1,  4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,1, 4,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1,  8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,1, 8,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1, 16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, u128,1,16,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1, 32);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, u128,1,32,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1, 64);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,1,64,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 1,128);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,1,128,unsigned,int_type); };

    // =========================================================================
    // Width 2
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,2, 1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,2, 2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,2, 4,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, u128,2, 8,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2,16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, u128,2,16,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2,32);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,2,32,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 2,64);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,2,64,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  i128,2, 1,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  i128,2, 2,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  i128,2, 4,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, i128,2, 8,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2,16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, i128,2,16,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2,32);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, i128,2,32,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 2,64);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,2,64,signed,int_type); };

    // =========================================================================
    // Width 4 (nibbles)
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 4, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,4, 1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 4, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,4, 2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 4, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, u128,4, 4,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 4, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, u128,4, 8,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 4,16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,4,16,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 4,32);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,4,32,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  i128,4, 1,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  i128,4, 2,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, i128,4, 4,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, i128,4, 8,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4,16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, i128,4,16,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 4,32);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,4,32,signed,int_type); };

    // =========================================================================
    // Width 8 (bytes)
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 8, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  u128,8, 1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 8, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, u128,8, 2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 8, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, u128,8, 4,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 8, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,8, 8,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 8,16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,8,16,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 8, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u8,  AtomicU8,  i128,8, 1,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 8, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, i128,8, 2,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 8, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, i128,8, 4,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 8, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, i128,8, 8,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 8,16);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,8,16,signed,int_type); };

    // =========================================================================
    // Width 16
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 16, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, u128,16, 1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 16, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, u128,16, 2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 16, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,16, 4,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 16, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,16, 8,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 16, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u16, AtomicU16, i128,16, 1,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 16, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, i128,16, 2,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 16, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, i128,16, 4,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 16, 8);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,16, 8,signed,int_type); };

    // =========================================================================
    // Width 32
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 32, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, u128,32,1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 32, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,32,2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 32, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,32,4,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 32, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u32, AtomicU32, i128,32,1,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 32, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, i128,32,2,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 32, 4);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,32,4,signed,int_type); };

    // =========================================================================
    // Width 64
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 64, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, u128,64,1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 64, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,64,2,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 64, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u64, AtomicU64, i128,64,1,signed,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 64, 2);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,64,2,signed,int_type); };

    // =========================================================================
    // Width 128
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u 128, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,128,1,unsigned,int_type); };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i 128, 1);) => { $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,128,1,signed,int_type); };

    // =========================================================================
    // General fallback
    // =========================================================================
    ($(#[$meta:meta])* $vis:vis struct $name:ident (u $width:literal, $count:literal);) => {
        const _: () = { assert!($width * $count <= 128, "atomic_bitarray: total bits exceed 128-bit limit"); };
        $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,u128,$width,$count,unsigned,int_type);
    };
    ($(#[$meta:meta])* $vis:vis struct $name:ident (i $width:literal, $count:literal);) => {
        const _: () = { assert!($width * $count <= 128, "atomic_bitarray: total bits exceed 128-bit limit"); };
        $crate::__atomic_bitarray_final!($(#[$meta])* $vis,$name,u128,AtomicU128,i128,$width,$count,signed,int_type);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __atomic_bitarray_final {
    ($(#[$meta:meta])* $vis:vis, $name:ident, $base_ty:ty, $atomic_ty:ident, $elem_ty:ty, $width:literal, $count:literal, $signed_tag:ident, $type_tag:ident) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $name(pub $crate::reexport::portable_atomic::$atomic_ty);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut list = f.debug_list();
                for i in 0..$count {
                    list.entry(&self.get(i, $crate::Ordering::Relaxed));
                }
                list.finish()
            }
        }

        impl $name {
            pub const ELEMENT_WIDTH: usize = $width;
            pub const ELEMENT_COUNT: usize = $count;
            pub const TOTAL_BITS:    usize = $width * $count;
            pub const MASK: $base_ty = if $width >= <$base_ty as $crate::BitLength>::BITS {
                !0 as $base_ty
            } else {
                ((1 as $base_ty) << $width) - 1
            };

            #[inline(always)]
            pub const fn new(val: $base_ty) -> Self {
                Self($crate::reexport::portable_atomic::$atomic_ty::new(val))
            }

            #[inline(always)]
            pub fn load(&self, order: $crate::Ordering) -> $base_ty {
                self.0.load(order)
            }

            #[inline(always)]
            pub fn store(&self, val: $base_ty, order: $crate::Ordering) {
                self.0.store(val, order);
            }

            #[inline]
            pub fn get(&self, index: usize, order: $crate::Ordering) -> $elem_ty {
                debug_assert!(index < $count, "atomic_bitarray index out of bounds");
                let shift = index * $width;
                let raw   = (self.0.load(order) >> shift) & Self::MASK;
                $crate::__bitarray_cast_get!(raw, $elem_ty, $width, $signed_tag, $type_tag)
            }

            #[inline]
            pub fn set(&self, index: usize, value: $elem_ty, order: $crate::Ordering) {
                debug_assert!(index < $count, "atomic_bitarray index out of bounds");
                let shift      = index * $width;
                let val_masked = ($crate::__bitarray_cast_set!(value, $base_ty, $type_tag) & Self::MASK) << shift;
                let mask       = !(Self::MASK << shift);

                self.0.fetch_update(order, $crate::Ordering::Relaxed, |raw| {
                    Some((raw & mask) | val_masked)
                }).unwrap();
            }
        }

        $crate::__atomic_bitarray_snapshot!($vis, $name, $width, $count, $signed_tag, $type_tag);

        $crate::paste::paste! {
            impl $name {
                /// Returns a non-atomic snapshot of the current state.
                #[inline]
                pub fn get_snapshot(&self, order: $crate::Ordering) -> [<$name Value>] {
                    [<$name Value>]::new(self.0.load(order))
                }

                /// Completely overwrites the entire atomic state with the given snapshot.
                #[inline]
                pub fn set_snapshot(&self, val: [<$name Value>], order: $crate::Ordering) {
                    self.0.store(val.to_bits(), order);
                }

                /// Atomically updates the array using a CAS loop.
                #[inline]
                pub fn update<F>(&self, set_order: $crate::Ordering, fetch_order: $crate::Ordering, mut f: F) -> [<$name Value>]
                where
                    F: FnMut(&mut [<$name Value>])
                {
                    let raw_prev = self.0.fetch_update(set_order, fetch_order, |raw| {
                        let mut snap = [<$name Value>]::new(raw);
                        f(&mut snap);
                        Some(snap.to_bits())
                    }).unwrap();
                    [<$name Value>]::new(raw_prev)
                }

                /// Conditionally updates the array using a CAS loop.
                #[inline]
                pub fn update_or_abort<F>(&self, set_order: $crate::Ordering, fetch_order: $crate::Ordering, mut f: F) -> Result<[<$name Value>], [<$name Value>]>
                where
                    F: FnMut(&mut [<$name Value>]) -> Option<()>
                {
                    self.0.fetch_update(set_order, fetch_order, |raw| {
                        let mut snap = [<$name Value>]::new(raw);
                        f(&mut snap).map(|_| snap.to_bits())
                    }).map(|raw| [<$name Value>]::new(raw))
                    .map_err(|raw| [<$name Value>]::new(raw))
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __atomic_bitarray_snapshot {
    ($vis:vis, $name:ident, $width:literal, $count:literal, unsigned, int_type) => {
        $crate::paste::paste! {
            $crate::bitarray! {
                #[doc = concat!("A non-atomic value snapshot of `", stringify!($name), "` used for batch updates.")]
                $vis struct [<$name Value>](u $width, $count);
            }
        }
    };
    ($vis:vis, $name:ident, $width:literal, $count:literal, signed, int_type) => {
        $crate::paste::paste! {
            $crate::bitarray! {
                #[doc = concat!("A non-atomic value snapshot of `", stringify!($name), "` used for batch updates.")]
                $vis struct [<$name Value>](i $width, $count);
            }
        }
    };
    ($vis:vis, $name:ident, $width:literal, $count:literal, unsigned, bool_type) => {
        $crate::paste::paste! {
            $crate::bitarray! {
                #[doc = concat!("A non-atomic value snapshot of `", stringify!($name), "` used for batch updates.")]
                $vis struct [<$name Value>](bool, $count);
            }
        }
    };
}
