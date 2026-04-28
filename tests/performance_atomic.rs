use bitcraft::{atomic_bitenum, atomic_bitstruct};
use core::sync::atomic::Ordering;
use std::hint::black_box;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Default, Copy, Clone)]
struct StandardState {
    pub is_active: bool,
    pub connections: u16,
    pub status: u8,
}

atomic_bitstruct! {
    struct AtomicState(AtomicU32) {
        pub is_active: bool = 1,
        pub connections: u16 = 16,
        pub status: u8 = 8,
    }
}

atomic_bitenum! {
    enum AtomicStatus(AtomicU8, 2) {
        OFF = 0,
        ON = 1,
        FAULT = 2,
    }
}

#[cfg(debug_assertions)]
const THREADS: usize = 4;
#[cfg(debug_assertions)]
const ITERATIONS_PER_THREAD: usize = 100_000;

#[cfg(not(debug_assertions))]
const THREADS: usize = 8;
#[cfg(not(debug_assertions))]
const ITERATIONS_PER_THREAD: usize = 1_000_000;

#[test]
fn test_atomic_performance_vs_mutex() {
    println!(
        "\n=== Atomic Performance vs Mutex ({} threads, {} iterations each) ===",
        THREADS, ITERATIONS_PER_THREAD
    );

    // ----------------------------------------------------------------------
    // PART A: Struct-Level Contention (atomic_bitstruct vs Mutex<Struct>)
    // ----------------------------------------------------------------------
    println!("\n--- Part A: Struct-Level Contention (atomic_bitstruct vs Mutex<Struct>) ---");

    // 1. Mutex<StandardState>
    let mutex_state = Arc::new(Mutex::new(StandardState::default()));
    let start_mutex = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let state = Arc::clone(&mutex_state);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                let mut guard = state.lock().unwrap();
                guard.is_active = i % 2 == 0;
                guard.connections = (i % 65535) as u16;
                guard.status = (i % 255) as u8;
                black_box(*guard);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let dur_mutex = start_mutex.elapsed();
    println!("Mutex<StandardState> Time:     {:?}", dur_mutex);

    // 2. AtomicState (Lock-Free)
    let atomic_state = Arc::new(AtomicState::new(0));
    let start_atomic = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let state = Arc::clone(&atomic_state);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                // Using update() to simulate a transactional batch update of multiple fields
                state.update(Ordering::Release, Ordering::Relaxed, |v| {
                    v.set_is_active(i % 2 == 0);
                    v.set_connections((i % 65535) as u16);
                    v.set_status((i % 255) as u8);
                });
                black_box(state.get(Ordering::Relaxed));
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let dur_atomic = start_atomic.elapsed();
    println!("atomic_bitstruct! Time:        {:?}", dur_atomic);

    println!(
        "  -> Atomic Speedup:           {:.2}x",
        dur_mutex.as_nanos() as f64 / dur_atomic.as_nanos() as f64
    );

    // ----------------------------------------------------------------------
    // PART B: Enum-Level Contention (atomic_bitenum vs Mutex<Enum>)
    // ----------------------------------------------------------------------
    println!("\n--- Part B: Enum-Level Contention (atomic_bitenum vs Mutex<u8>) ---");

    // 1. Mutex<u8>
    let mutex_enum = Arc::new(Mutex::new(0u8));
    let start_mutex_enum = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let e = Arc::clone(&mutex_enum);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                let mut guard = e.lock().unwrap();
                *guard = (i % 3) as u8;
                black_box(*guard);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let dur_mutex_enum = start_mutex_enum.elapsed();
    println!("Mutex<u8> Enum Time:           {:?}", dur_mutex_enum);

    // 2. AtomicStatus (Lock-Free)
    let atomic_status = Arc::new(AtomicStatus::new(AtomicStatusValue::OFF));
    let start_atomic_enum = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let e = Arc::clone(&atomic_status);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                let next = match i % 3 {
                    0 => AtomicStatusValue::OFF,
                    1 => AtomicStatusValue::ON,
                    _ => AtomicStatusValue::FAULT,
                };
                e.store(next, Ordering::Release);
                black_box(e.load(Ordering::Relaxed));
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let dur_atomic_enum = start_atomic_enum.elapsed();
    println!("atomic_bitenum! Time:          {:?}", dur_atomic_enum);

    println!(
        "  -> Atomic Speedup:           {:.2}x",
        dur_mutex_enum.as_nanos() as f64 / dur_atomic_enum.as_nanos() as f64
    );
}
