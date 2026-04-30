use bitcraft::{atomic_bitenum, atomic_bitstruct, bitenum};
use bitcraft::Ordering;

bitenum! {
    pub enum State(2) {
        A = 0,
        B = 1,
        C = 2,
    }
}

atomic_bitstruct! {
    pub struct ConcurrentTest(AtomicU32) {
        pub flag: bool = 1,
        pub value: u8 = 4,
        pub signed_val: i16 = 12,
        pub state: State = 2,
    }
}

#[test]
fn test_atomic_bitstruct_api() {
    let f = ConcurrentTest::new(0);

    // Initial State
    assert_eq!(f.flag(Ordering::Relaxed), false);
    assert_eq!(f.value(Ordering::Relaxed), 0);
    assert_eq!(f.signed_val(Ordering::Relaxed), 0);
    assert_eq!(f.state(Ordering::Relaxed), State::A);

    // Setters
    f.set_flag(true, Ordering::SeqCst);
    assert_eq!(f.flag(Ordering::SeqCst), true);

    f.set_value(15, Ordering::SeqCst);
    assert_eq!(f.value(Ordering::SeqCst), 15);

    f.set_signed_val(-2000, Ordering::SeqCst);
    assert_eq!(f.signed_val(Ordering::SeqCst), -2000);

    f.set_state(State::C, Ordering::SeqCst);
    assert_eq!(f.state(Ordering::SeqCst), State::C);

    // Try Setters
    assert!(f.try_set_flag(false, Ordering::SeqCst).is_ok());
    assert_eq!(f.flag(Ordering::SeqCst), false);

    assert!(f.try_set_value(20, Ordering::SeqCst).is_err());
    assert!(f.try_set_signed_val(2048, Ordering::SeqCst).is_err());

    assert!(f.try_set_state(State::B, Ordering::SeqCst).is_ok());
    assert_eq!(f.state(Ordering::SeqCst), State::B);

    // Validate underlying load
    assert!(f.load(Ordering::SeqCst) > 0);
}

atomic_bitstruct! {
    pub struct SignedConcurrentTest(AtomicI32) {
        pub flag: bool = 1,
        pub value: i16 = 12,
    }
}

#[test]
fn test_signed_atomic_bitstruct() {
    let f = SignedConcurrentTest::new(-1);
    assert_eq!(f.flag(Ordering::Relaxed), true);
    assert_eq!(f.value(Ordering::Relaxed), -1);

    f.set_value(-2000, Ordering::SeqCst);
    assert_eq!(f.value(Ordering::SeqCst), -2000);

    // Test get()
    let current_val = f.get(Ordering::SeqCst);
    assert_eq!(current_val.flag(), true);
    assert_eq!(current_val.value(), -2000);

    let final_val = f.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
        v.set_flag(false);
        v.set_value(500);
    });

    assert_eq!(final_val.flag(), true);
    assert_eq!(final_val.value(), -2000);

    assert_eq!(f.flag(Ordering::Relaxed), false);
    assert_eq!(f.value(Ordering::Relaxed), 500);
    // Test update_or_abort aborting
    let try_abort_result = f.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
        if v.flag() == false {
            return None; // Abort!
        }
        v.set_flag(true);
        Some(())
    });

    // It should have aborted and returned the unmodified state
    assert!(try_abort_result.is_err());
    assert_eq!(try_abort_result.unwrap_err().value(), 500);
    assert_eq!(f.value(Ordering::SeqCst), 500);

    // Test update_or_abort succeeding
    let try_success_result = f.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
        if v.flag() == false {
            v.set_value(999);
            return Some(()); // Commit!
        }
        None
    });

    assert!(try_success_result.is_ok());
    assert_eq!(f.value(Ordering::SeqCst), 999);

    // Test try_setters inside the closure
    let try_inner = f.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
        // flag is boolean, so try_set always succeeds
        let _ = v.try_set_flag(false);

        if v.try_set_value(30000).is_err() {
            // It correctly blocked the overflow
            // let's do a valid try_set and return
            let _ = v.try_set_value(1234);
            return Some(());
        }
        None
    });

    assert!(try_inner.is_ok());
    assert_eq!(f.value(Ordering::SeqCst), 1234);
    assert_eq!(f.flag(Ordering::SeqCst), false);
}

#[test]
fn test_threaded_concurrent_updates() {
    use std::sync::Arc;
    use std::thread;

    let shared = Arc::new(ConcurrentTest::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let f = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                // Safely update the value atomically using the closure batch updater
                f.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
                    let current = v.value();
                    v.set_value(current.wrapping_add(1) % 16);
                });
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Since we wrapped by 16 and did 1000 increments, 1000 % 16 = 8.
    // 0 + 1000 = 1000. 1000 % 16 = 8. Wait! 1000 increments each adding 1 and wrapping at 16.
    // So the final value should be (1000 % 16) = 8.
    assert_eq!(shared.value(Ordering::SeqCst), 8);
}

atomic_bitenum! {
    pub enum ConcurrentMode(AtomicU8, 2) {
        STANDBY = 0,
        ACTIVE = 1,
        ERROR = 2,
    }
}

#[test]
fn test_atomic_bitenum() {
    let mode = ConcurrentMode::new(ConcurrentModeValue::STANDBY);
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::STANDBY);

    mode.store(ConcurrentModeValue::ACTIVE, Ordering::SeqCst);
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::ACTIVE);

    let prev = mode.swap(ConcurrentModeValue::ERROR, Ordering::SeqCst);
    assert_eq!(prev, ConcurrentModeValue::ACTIVE);
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::ERROR);

    let res = mode.compare_exchange(
        ConcurrentModeValue::ERROR,
        ConcurrentModeValue::STANDBY,
        Ordering::SeqCst,
        Ordering::Relaxed,
    );
    assert!(res.is_ok());
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::STANDBY);

    let bad_res = mode.compare_exchange(
        ConcurrentModeValue::ERROR,
        ConcurrentModeValue::ACTIVE,
        Ordering::SeqCst,
        Ordering::Relaxed,
    );
    assert!(bad_res.is_err());

    let fetch_res = mode.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
        if v == ConcurrentModeValue::STANDBY {
            Some(ConcurrentModeValue::ACTIVE)
        } else {
            None
        }
    });
    assert!(fetch_res.is_ok());
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::ACTIVE);

    mode.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
        assert_eq!(v, ConcurrentModeValue::ACTIVE);
        ConcurrentModeValue::STANDBY
    });
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::STANDBY);

    let mut current = mode.load(Ordering::SeqCst);
    loop {
        match mode.compare_exchange_weak(
            current,
            ConcurrentModeValue::ERROR,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(actual) => current = actual,
        }
    }
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentModeValue::ERROR);

    assert_eq!(ConcurrentMode::BITS, 2);

    let def_mode = ConcurrentMode::default();
    assert_eq!(def_mode.load(Ordering::SeqCst), ConcurrentModeValue::STANDBY);

    let debug_str = format!("{:?}", mode);
    assert!(debug_str.contains("ConcurrentMode(ConcurrentModeValue(2)::ERROR)"));
}

atomic_bitstruct! {
    pub struct ConcurrentTest128(AtomicU128) {
        pub flag: bool = 1,
        pub value: u64 = 64,
        pub status: State = 2,
        pub extra: u32 = 32,
    }
}

#[test]
fn test_atomic_u128_bitstruct() {
    let f = ConcurrentTest128::new(0);
    f.set_flag(true, Ordering::SeqCst);
    f.set_value(0xDEADBEEFCAFEBABE, Ordering::SeqCst);
    f.set_status(State::C, Ordering::SeqCst);
    f.set_extra(0x12345678, Ordering::SeqCst);

    assert_eq!(f.flag(Ordering::SeqCst), true);
    assert_eq!(f.value(Ordering::SeqCst), 0xDEADBEEFCAFEBABE);
    assert_eq!(f.status(Ordering::SeqCst), State::C);
    assert_eq!(f.extra(Ordering::SeqCst), 0x12345678);

    f.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
        v.set_value(0x1122334455667788);
    });
    assert_eq!(f.value(Ordering::SeqCst), 0x1122334455667788);
}

atomic_bitstruct! {
    pub struct SignedConcurrentTest128(AtomicI128) {
        pub flag: bool = 1,
        pub value: i64 = 64,
        pub extra: i32 = 32,
    }
}

#[test]
fn test_atomic_i128_bitstruct() {
    let f = SignedConcurrentTest128::new(0);
    f.set_value(-1234567890123456789, Ordering::SeqCst);
    f.set_extra(-999999, Ordering::SeqCst);

    assert_eq!(f.value(Ordering::SeqCst), -1234567890123456789);
    assert_eq!(f.extra(Ordering::SeqCst), -999999);

    f.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
        let current = v.value();
        v.set_value(current + 1);
    });
    assert_eq!(f.value(Ordering::SeqCst), -1234567890123456788);
}

atomic_bitenum! {
    pub enum ConcurrentMode128(AtomicU128, 128) {
        MIN = 0,
        MAX = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    }
}

#[test]
fn test_atomic_u128_bitenum() {
    let mode = ConcurrentMode128::new(ConcurrentMode128Value::MIN);
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentMode128Value::MIN);

    mode.store(ConcurrentMode128Value::MAX, Ordering::SeqCst);
    assert_eq!(mode.load(Ordering::SeqCst), ConcurrentMode128Value::MAX);
}
