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
const THREADS: usize = 32;
#[cfg(not(debug_assertions))]
const ITERATIONS_PER_THREAD: usize = 1_000_000;

#[test]
fn test_atomic_performance_vs_mutex() {
    println!(
        "\n=== Atomic Performance vs Mutex ({} threads, {} iterations each) ===",
        THREADS, ITERATIONS_PER_THREAD
    );

    // ----------------------------------------------------------------------
    // PART A: Heavy Contention (All threads hitting ONE shared word)
    // ----------------------------------------------------------------------
    println!("\n--- Part A: Heavy Contention (32 threads hitting ONE shared word) ---");

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
    for h in handles { h.join().unwrap(); }
    let dur_mutex = start_mutex.elapsed();
    println!("Mutex<StandardState> Time:     {:?}", dur_mutex);

    // 2. AtomicState (Lock-Free CAS Loop)
    let atomic_state = Arc::new(AtomicState::new(0));
    let start_atomic = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let state = Arc::clone(&atomic_state);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                state.update(Ordering::Release, Ordering::Relaxed, |v| {
                    v.set_is_active(i % 2 == 0);
                    v.set_connections((i % 65535) as u16);
                    v.set_status((i % 255) as u8);
                });
                black_box(state.get(Ordering::Relaxed));
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    let dur_atomic = start_atomic.elapsed();
    println!("atomic_bitstruct! Time:        {:?}", dur_atomic);
    println!("  -> Atomic Speedup:           {:.2}x", dur_mutex.as_nanos() as f64 / dur_atomic.as_nanos() as f64);

    // ----------------------------------------------------------------------
    // PART B: Enum Transitions (Direct Store vs Mutex)
    // ----------------------------------------------------------------------
    println!("\n--- Part B: Enum Transitions (Direct Store vs Mutex) ---");

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
    for h in handles { h.join().unwrap(); }
    let dur_mutex_enum = start_mutex_enum.elapsed();
    println!("Mutex<u8> Enum Time:           {:?}", dur_mutex_enum);

    // 2. AtomicStatus (Direct Store)
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
    for h in handles { h.join().unwrap(); }
    let dur_atomic_enum = start_atomic_enum.elapsed();
    println!("atomic_bitenum! Time:          {:?}", dur_atomic_enum);
    println!("  -> Atomic Speedup:           {:.2}x", dur_mutex_enum.as_nanos() as f64 / dur_atomic_enum.as_nanos() as f64);

    // ----------------------------------------------------------------------
    // PART C: Parallel Throughput (NO Contention - 32 distinct words)
    // ----------------------------------------------------------------------
    println!("\n--- Part C: Parallel Throughput (NO Contention - 32 distinct words) ---");

    // 1. Mutex Array
    let mutexes: Vec<Arc<Mutex<StandardState>>> = (0..THREADS).map(|_| Arc::new(Mutex::new(StandardState::default()))).collect();
    let start_mutex_p = Instant::now();
    let mut handles = Vec::new();
    for t in 0..THREADS {
        let m = Arc::clone(&mutexes[t]);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                let mut guard = m.lock().unwrap();
                guard.is_active = i % 2 == 0;
                black_box(*guard);
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    let dur_mutex_p = start_mutex_p.elapsed();
    println!("32x Mutex Time:                {:?}", dur_mutex_p);

    // 2. Atomic Array
    let atomics: Vec<Arc<AtomicState>> = (0..THREADS).map(|_| Arc::new(AtomicState::new(0))).collect();
    let start_atomic_p = Instant::now();
    let mut handles = Vec::new();
    for t in 0..THREADS {
        let a = Arc::clone(&atomics[t]);
        handles.push(thread::spawn(move || {
            for i in 0..ITERATIONS_PER_THREAD {
                // Using Relaxed because there is zero contention between threads
                a.set_is_active(i % 2 == 0, Ordering::Relaxed);
                black_box(a.is_active(Ordering::Relaxed));
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    let dur_atomic_p = start_atomic_p.elapsed();
    println!("32x Atomic Time:               {:?}", dur_atomic_p);
    println!("  -> Parallel Speedup:         {:.2}x", dur_mutex_p.as_nanos() as f64 / dur_atomic_p.as_nanos() as f64);

    // ----------------------------------------------------------------------
    // PART D: Conditional Transitions (compare_exchange vs Mutex-check)
    // ----------------------------------------------------------------------
    println!("\n--- Part D: Conditional Transitions (compare_exchange vs Mutex-check) ---");

    // 1. Mutex Check
    let mutex_cond = Arc::new(Mutex::new(0u8)); // 0 = OFF, 1 = ON
    let start_mutex_c = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let m = Arc::clone(&mutex_cond);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS_PER_THREAD {
                let mut guard = m.lock().unwrap();
                if *guard == 0 {
                    *guard = 1;
                } else {
                    *guard = 0;
                }
                black_box(*guard);
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    let dur_mutex_c = start_mutex_c.elapsed();
    println!("Mutex-Check Time:              {:?}", dur_mutex_c);

    // 2. Atomic compare_exchange
    let atomic_cond = Arc::new(AtomicStatus::new(AtomicStatusValue::OFF));
    let start_atomic_c = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let a = Arc::clone(&atomic_cond);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS_PER_THREAD {
                let current = a.load(Ordering::Relaxed);
                let next = if current == AtomicStatusValue::OFF {
                    AtomicStatusValue::ON
                } else {
                    AtomicStatusValue::OFF
                };
                // Attempt transition
                let _ = a.compare_exchange(current, next, Ordering::Release, Ordering::Relaxed);
                black_box(a.load(Ordering::Relaxed));
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    let dur_atomic_c = start_atomic_c.elapsed();
    println!("Atomic CAS Time:               {:?}", dur_atomic_c);
    println!("  -> CAS Speedup:              {:.2}x", dur_mutex_c.as_nanos() as f64 / dur_atomic_c.as_nanos() as f64);
}
