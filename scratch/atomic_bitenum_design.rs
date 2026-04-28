#[macro_export]
macro_rules! atomic_bitenum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident (AtomicU8, u $bits:expr) {
            $( $variant:ident = $val:expr ),* $(,)?
        }
    ) => {
        $crate::atomic_bitenum!(@impl $(#[$meta])* $vis $enum_name core::sync::atomic::AtomicU8, u8, u $bits, { $($variant = $val),* });
    };
    // ...
}
