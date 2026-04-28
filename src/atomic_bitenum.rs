/// A declarative macro for generating atomic bitenums.
///
/// This macro generates a `#[repr(transparent)]` struct wrapping an atomic integer
/// (`AtomicU8`, `AtomicU16`, etc.). It automatically generates atomic methods like
/// `load`, `store`, `swap`, `compare_exchange`, `update`, and `update_or_abort` that operate
/// strictly on the defined enumeration variants.
///
/// # Example
///
/// ```rust
/// use bitcraft::atomic_bitenum;
/// use core::sync::atomic::Ordering;
///
/// atomic_bitenum! {
///     pub enum AtomicConnectionState(AtomicU8, 2) {
///         DISCONNECTED = 0,
///         CONNECTING = 1,
///         CONNECTED = 2,
///     }
/// }
///
/// let state = AtomicConnectionState::new(AtomicConnectionStateValue::DISCONNECTED);
/// state.store(AtomicConnectionStateValue::CONNECTING, Ordering::Release);
/// assert_eq!(state.load(Ordering::Acquire), AtomicConnectionStateValue::CONNECTING);
/// ```
#[macro_export]
macro_rules! atomic_bitenum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU8, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU8, u8, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU8, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU8, u8, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU8, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU8, u8, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU16, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU16, u16, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU16, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU16, u16, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU16, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU16, u16, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU32, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU32, u32, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU32, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU32, u32, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU32, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU32, u32, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU64, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU64, u64, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU64, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU64, u64, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU64, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU64, u64, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI8, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI8, i8, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI8, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI8, i8, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI8, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI8, i8, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI16, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI16, i16, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI16, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI16, i16, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI16, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI16, i16, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI32, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI32, i32, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI32, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI32, i32, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI32, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI32, i32, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI64, u $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI64, i64, u $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI64, i $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI64, i64, i $bits, { $($variant = $val),* });
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicI64, $bits:expr) {
            $(
                $variant:ident = $val:expr
            ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicI64, i64, u $bits, { $($variant = $val),* });
    };

    (@impl $(#[$meta:meta])* $vis:vis $enum_name:ident $atomic_ty:ty, $base_type:ty, $modifier:ident $bits:expr, { $($variant:ident = $val:expr),* }) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $enum_name(pub $atomic_ty);

        impl core::fmt::Debug for $enum_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let raw = self.0.load(core::sync::atomic::Ordering::Relaxed);
                $crate::paste::paste! {
                    let val = [<$enum_name Value>]::from_bits(raw as _);
                    f.debug_tuple(stringify!($enum_name)).field(&val).finish()
                }
            }
        }

        $crate::paste::paste! {
            $crate::bitenum! {
                #[doc = concat!("A non-atomic value snapshot of `", stringify!($enum_name), "` used for atomic transitions.")]
                $vis enum [<$enum_name Value>]($modifier $bits) {
                    $(
                        $variant = $val,
                    )*
                }
            }

            impl Default for $enum_name {
                fn default() -> Self {
                    Self::new([<$enum_name Value>]::default())
                }
            }

            impl $enum_name {
                #[allow(dead_code)]
                pub const BITS: usize = $bits;

                /// Creates a new atomic instance from an enum variant.
                #[inline(always)]
                #[allow(dead_code)]
                pub const fn new(val: [<$enum_name Value>]) -> Self {
                    Self(<$atomic_ty>::new(val.to_bits() as _))
                }

                /// Returns the current variant via `load`.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn load(&self, order: core::sync::atomic::Ordering) -> [<$enum_name Value>] {
                    [<$enum_name Value>]::from_bits(self.0.load(order) as _)
                }

                /// Stores a new variant via `store`.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn store(&self, val: [<$enum_name Value>], order: core::sync::atomic::Ordering) {
                    self.0.store(val.to_bits() as _, order)
                }

                /// Atomically swaps the variant and returns the previous one.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn swap(&self, val: [<$enum_name Value>], order: core::sync::atomic::Ordering) -> [<$enum_name Value>] {
                    [<$enum_name Value>]::from_bits(self.0.swap(val.to_bits() as _, order) as _)
                }

                /// Compares and exchanges the variant.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn compare_exchange(
                    &self,
                    current: [<$enum_name Value>],
                    new: [<$enum_name Value>],
                    success: core::sync::atomic::Ordering,
                    failure: core::sync::atomic::Ordering,
                ) -> Result<[<$enum_name Value>], [<$enum_name Value>]> {
                    self.0.compare_exchange(current.to_bits() as _, new.to_bits() as _, success, failure)
                        .map(|raw| [<$enum_name Value>]::from_bits(raw as _))
                        .map_err(|raw| [<$enum_name Value>]::from_bits(raw as _))
                }

                /// Weakly compares and exchanges the variant.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn compare_exchange_weak(
                    &self,
                    current: [<$enum_name Value>],
                    new: [<$enum_name Value>],
                    success: core::sync::atomic::Ordering,
                    failure: core::sync::atomic::Ordering,
                ) -> Result<[<$enum_name Value>], [<$enum_name Value>]> {
                    self.0.compare_exchange_weak(current.to_bits() as _, new.to_bits() as _, success, failure)
                        .map(|raw| [<$enum_name Value>]::from_bits(raw as _))
                        .map_err(|raw| [<$enum_name Value>]::from_bits(raw as _))
                }

                /// Fetches and updates the variant via a CAS loop closure.
                /// The closure must return `Some(variant)` to commit, or `None` to abort.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn update_or_abort<F>(
                    &self,
                    set_order: core::sync::atomic::Ordering,
                    fetch_order: core::sync::atomic::Ordering,
                    mut f: F,
                ) -> Result<[<$enum_name Value>], [<$enum_name Value>]>
                where
                    F: FnMut([<$enum_name Value>]) -> Option<[<$enum_name Value>]>
                {
                    self.0.fetch_update(set_order, fetch_order, |raw| {
                        let snap = [<$enum_name Value>]::from_bits(raw as _);
                        f(snap).map(|v| v.to_bits() as _)
                    }).map(|raw| [<$enum_name Value>]::from_bits(raw as _))
                    .map_err(|raw| [<$enum_name Value>]::from_bits(raw as _))
                }

                /// Fetches and updates the variant via a CAS loop closure.
                /// The closure must return the new variant to commit.
                #[inline(always)]
                #[allow(dead_code)]
                pub fn update<F>(
                    &self,
                    set_order: core::sync::atomic::Ordering,
                    fetch_order: core::sync::atomic::Ordering,
                    mut f: F,
                ) -> [<$enum_name Value>]
                where
                    F: FnMut([<$enum_name Value>]) -> [<$enum_name Value>]
                {
                    let raw_prev = self.0.fetch_update(set_order, fetch_order, |raw| {
                        let snap = [<$enum_name Value>]::from_bits(raw as _);
                        Some(f(snap).to_bits() as _)
                    }).unwrap();
                    [<$enum_name Value>]::from_bits(raw_prev as _)
                }
            }
        }
    };
}

