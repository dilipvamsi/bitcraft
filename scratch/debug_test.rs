use bitcraft::{atomic_bitenum, bitenum};
use core::sync::atomic::{AtomicU8, Ordering};

atomic_bitenum! {
    pub enum ConcurrentMode(AtomicU8, 2) {
        STANDBY = 0,
        ACTIVE = 1,
        ERROR = 2,
    }
}

fn main() {
    let mode = ConcurrentMode::new(ConcurrentModeValue::ERROR);
    println!("{:?}", mode);
}
