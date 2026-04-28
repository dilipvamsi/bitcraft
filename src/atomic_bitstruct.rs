/// A declarative macro for generating atomic bitfields.
///
/// This macro generates a `#[repr(transparent)]` struct wrapping an atomic integer
/// (`AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`). It automatically generates
/// getters and setters that take memory orderings, allowing lock-free concurrent mutation
/// of individual bitfields safely via `fetch_update`.
///
/// # Example
///
/// ```rust
/// use bitcraft::atomic_bitstruct;
/// use core::sync::atomic::Ordering;
///
/// atomic_bitstruct! {
///     pub struct ConcurrentFlags(AtomicU32) {
///         pub is_ready: bool = 1,
///         pub status: u8 = 3,
///         pub retries: i16 = 12, // Native signed extraction
///     }
/// }
///
/// let flags = ConcurrentFlags::new(0);
/// flags.set_is_ready(true, Ordering::Release);
/// flags.set_retries(-500, Ordering::Release);
///
/// assert!(flags.is_ready(Ordering::Acquire));
/// assert_eq!(flags.retries(Ordering::Acquire), -500);
/// ```
#[macro_export]
macro_rules! atomic_bitstruct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicU8) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicU8, u8, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicU16) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicU16, u16, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicU32) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicU32, u32, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicU64) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicU64, u64, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicI8) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicI8, i8, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicI16) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicI16, i16, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicI32) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicI32, i32, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident (AtomicI64) {
            $(
                $field_vis:vis $field_name:ident: $field_type:tt = $bits:tt
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitstruct!(@impl $(#[$meta])* $vis $struct_name core::sync::atomic::AtomicI64, i64, { $( $field_vis $field_name: $field_type = $bits ),* }, $($field_vis $field_name $field_type $bits)*);
    };

    (@impl $(#[$meta:meta])* $vis:vis $struct_name:ident $atomic_ty:ty, $base_type:ty, { $($field_vis_struct:vis $field_name_struct:ident: $field_type_struct:tt = $bits_struct:tt),* }, $($field_vis:vis $field_name:ident $field_type:tt $bits:tt)*) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $struct_name(pub $atomic_ty);

        impl core::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("raw", &self.0.load(core::sync::atomic::Ordering::Relaxed))
                    $(
                        .field(stringify!($field_name), &self.$field_name(core::sync::atomic::Ordering::Relaxed))
                    )*
                    .finish()
            }
        }

        const _: () = {
            let _ = <$base_type as $crate::IsValidBaseInt>::ASSERT_VALID;
            $crate::bitstruct!(@check_fields $($field_type)*);

            #[allow(dead_code)]
            const TOTAL_BITS: usize = 0 $( + $bits )*;
            assert!(TOTAL_BITS <= <$base_type as $crate::IsValidBaseInt>::MAX_BITS, "Sum of field bits exceeds base type max bits");
        };

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new(0)
            }
        }

        impl $struct_name {
            #[allow(dead_code)]
            pub const BITS: usize = <$base_type as $crate::BitLength>::BITS;

            /// Creates a new instance from a raw integer value.
            #[inline(always)]
            #[allow(dead_code)]
            pub const fn new(val: $base_type) -> Self {
                Self(<$atomic_ty>::new(val))
            }

            /// Returns the raw interior integer value via `load`.
            #[inline(always)]
            #[allow(dead_code)]
            pub fn load(&self, order: core::sync::atomic::Ordering) -> $base_type {
                self.0.load(order)
            }

            /// Stores a raw integer value via `store`.
            #[inline(always)]
            #[allow(dead_code)]
            pub fn store(&self, val: $base_type, order: core::sync::atomic::Ordering) {
                self.0.store(val, order)
            }

            $crate::atomic_bitstruct!(@impl_getters_setters $base_type, 0, $($field_vis $field_name $field_type $bits)*);
        }

        $crate::paste::paste! {
            $crate::bitstruct! {
                #[doc = concat!("A non-atomic value snapshot of `", stringify!($struct_name), "` used for batch updates.")]
                $vis struct [<$struct_name Value>]($base_type) {
                    $(
                        $field_vis_struct $field_name_struct: $field_type_struct = $bits_struct,
                    )*
                }
            }

            impl $struct_name {
                /// Returns a non-atomic snapshot of the current state as a `Value` struct.
                #[inline]
                pub fn get(&self, order: core::sync::atomic::Ordering) -> [<$struct_name Value>] {
                    [<$struct_name Value>]::from_bits(self.0.load(order))
                }
                /// Completely overwrites the entire atomic state with the given `Value`.
                /// This is a direct atomic `store` operation and does not perform a CAS loop.
                #[inline]
                pub fn set(&self, val: [<$struct_name Value>], order: core::sync::atomic::Ordering) {
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
                pub fn update<F>(&self, set_order: core::sync::atomic::Ordering, fetch_order: core::sync::atomic::Ordering, mut f: F) -> [<$struct_name Value>]
                where
                    F: FnMut(&mut [<$struct_name Value>])
                {
                    let raw_prev = self.0.fetch_update(set_order, fetch_order, |raw| {
                        let mut snap = [<$struct_name Value>]::from_bits(raw);
                        f(&mut snap);
                        Some(snap.to_bits())
                    }).unwrap();
                    [<$struct_name Value>]::from_bits(raw_prev)
                }

                /// Conditionally updates multiple fields using a Compare-And-Swap (CAS) loop.
                ///
                /// The provided closure must return `Some(())` to commit the new state, or `None` to abort the loop.
                /// If `None` is returned, the CAS loop is aborted and `Err(Value)` containing the un-modified state is returned.
                #[inline]
                pub fn update_or_abort<F>(&self, set_order: core::sync::atomic::Ordering, fetch_order: core::sync::atomic::Ordering, mut f: F) -> Result<[<$struct_name Value>], [<$struct_name Value>]>
                where
                    F: FnMut(&mut [<$struct_name Value>]) -> Option<()>
                {
                    self.0.fetch_update(set_order, fetch_order, |raw| {
                        let mut snap = [<$struct_name Value>]::from_bits(raw);
                        f(&mut snap).map(|_| snap.to_bits())
                    }).map(|raw| [<$struct_name Value>]::from_bits(raw))
                    .map_err(|raw| [<$struct_name Value>]::from_bits(raw))
                }
            }
        }
    };

    (@impl_getters_setters $base_type:ty, $shift:expr, ) => {};

    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident bool $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            pub const [<$field_name:upper _BITS>]: usize = $bits;
            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[allow(dead_code)]
            #[inline]
            $field_vis fn $field_name(&self, order: core::sync::atomic::Ordering) -> bool {
                ((self.0.load(order) >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) != 0
            }

            #[allow(dead_code)]
            #[inline]
            $field_vis fn [<set_ $field_name>](&self, val: bool, order: core::sync::atomic::Ordering) {
                let val_masked = val as $base_type;
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
            }

            #[allow(dead_code)]
            $field_vis fn [<try_set_ $field_name>](&self, val: bool, order: core::sync::atomic::Ordering) -> Result<(), $crate::BitstructError> {
                self.[<set_ $field_name>](val, order);
                Ok(())
            }
        }
        $crate::atomic_bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };

    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u8 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u8 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u16 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u16 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u32 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u32 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u64 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u64 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident u128 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_int $base_type, $shift, $field_vis $field_name u128 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i8 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i8 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i16 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i16 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i32 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i32 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i64 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i64 $bits $($rest)*); };
    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident i128 $bits:tt $($rest:tt)*) => { $crate::atomic_bitstruct!(@impl_signed_int $base_type, $shift, $field_vis $field_name i128 $bits $($rest)*); };

    (@impl_int $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            pub const [<$field_name:upper _BITS>]: usize = $bits;
            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[allow(dead_code)]
            #[inline]
            $field_vis fn $field_name(&self, order: core::sync::atomic::Ordering) -> $field_type {
                ((self.0.load(order) >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as $field_type
            }

            #[allow(dead_code)]
            #[inline]
            $field_vis fn [<set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) {
                debug_assert!((val as $base_type) <= Self::[<$field_name:upper _MASK>], "Value {} overflows allocated {} bits", val, $bits);
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
            }

            #[allow(dead_code)]
            $field_vis fn [<try_set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) -> Result<(), $crate::BitstructError> {
                if (val as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: (val as $base_type) as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
                Ok(())
            }
        }
        $crate::atomic_bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };

    (@impl_signed_int $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            pub const [<$field_name:upper _BITS>]: usize = $bits;
            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[doc(hidden)]
            pub const [<$field_name:upper _MIN>]: $field_type = (!0 as $field_type) << (Self::[<$field_name:upper _BITS>] - 1);
            #[doc(hidden)]
            pub const [<$field_name:upper _MAX>]: $field_type = !Self::[<$field_name:upper _MIN>];
            #[doc(hidden)]
            const [<$field_name:upper _SHIFT_UP>]: usize = <$field_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>];

            #[allow(dead_code)]
            #[inline]
            $field_vis fn $field_name(&self, order: core::sync::atomic::Ordering) -> $field_type {
                let raw = ((self.0.load(order) >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as $field_type;
                (raw << Self::[<$field_name:upper _SHIFT_UP>]) >> Self::[<$field_name:upper _SHIFT_UP>]
            }

            #[allow(dead_code)]
            #[inline]
            $field_vis fn [<set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) {
                debug_assert!(val >= Self::[<$field_name:upper _MIN>] && val <= Self::[<$field_name:upper _MAX>], "Value {} out of bounds for {} bits", val, $bits);
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
            }

            #[allow(dead_code)]
            $field_vis fn [<try_set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) -> Result<(), $crate::BitstructError> {
                if val < Self::[<$field_name:upper _MIN>] || val > Self::[<$field_name:upper _MAX>] {
                    return Err($crate::BitstructError::Overflow { value: val as i128 as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
                Ok(())
            }
        }
        $crate::atomic_bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };

    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident ($field_type:ty) $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            pub const [<$field_name:upper _BITS>]: usize = $bits;
            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[allow(dead_code)]
            #[inline]
            $field_vis fn $field_name(&self, order: core::sync::atomic::Ordering) -> $field_type {
                ((self.0.load(order) >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as $field_type
            }

            #[allow(dead_code)]
            #[inline]
            $field_vis fn [<set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) {
                debug_assert!((val as $base_type) <= Self::[<$field_name:upper _MASK>], "Value {} overflows allocated {} bits", val, $bits);
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
            }

            #[allow(dead_code)]
            $field_vis fn [<try_set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) -> Result<(), $crate::BitstructError> {
                if (val as $base_type) > Self::[<$field_name:upper _MASK>] {
                    return Err($crate::BitstructError::Overflow { value: (val as $base_type) as u128, allocated_bits: $bits });
                }
                let val_masked = (val as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
                Ok(())
            }
        }
        $crate::atomic_bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };

    (@impl_getters_setters $base_type:ty, $shift:expr, $field_vis:vis $field_name:ident $field_type:tt $bits:tt $($rest:tt)*) => {
        $crate::paste::paste! {
            pub const [<$field_name:upper _OFFSET>]: usize = $shift;
            pub const [<$field_name:upper _BITS>]: usize = $bits;
            #[doc(hidden)]
            const [<$field_name:upper _MASK>]: $base_type = ((!0 as <$base_type as $crate::IsValidBaseInt>::Unsigned) >> (<$base_type as $crate::BitLength>::BITS - Self::[<$field_name:upper _BITS>])) as $base_type;

            #[allow(dead_code)]
            #[inline]
            $field_vis fn $field_name(&self, order: core::sync::atomic::Ordering) -> $field_type {
                $field_type::from_bits(((self.0.load(order) >> Self::[<$field_name:upper _OFFSET>]) & Self::[<$field_name:upper _MASK>]) as _)
            }

            #[allow(dead_code)]
            #[inline]
            $field_vis fn [<set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) {
                const _: () = assert!(<$field_type>::BITS <= $bits, "Enum bit width exceeds allocated field width");
                let val_masked = (val.to_bits() as $base_type) & Self::[<$field_name:upper _MASK>];
                self.0.fetch_update(order, core::sync::atomic::Ordering::Relaxed, |raw| {
                    Some((raw & !(Self::[<$field_name:upper _MASK>] << Self::[<$field_name:upper _OFFSET>])) | (val_masked << Self::[<$field_name:upper _OFFSET>]))
                }).unwrap();
            }

            #[allow(dead_code)]
            $field_vis fn [<try_set_ $field_name>](&self, val: $field_type, order: core::sync::atomic::Ordering) -> Result<(), $crate::BitstructError> {
                self.[<set_ $field_name>](val, order);
                Ok(())
            }
        }
        $crate::atomic_bitstruct!(@impl_getters_setters $base_type, $shift + $bits, $($rest)*);
    };
}
