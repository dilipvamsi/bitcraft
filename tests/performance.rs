use bitstruct::{bitenum, bitstruct, bytestruct, byteval};
use std::hint::black_box;
use std::time::Instant;

// --- DUMMY ENUMS FOR BITENUM TESTING ---
bitenum! {
    pub enum PerfState(2) {
        IDLE = 0,
        RUNNING = 1,
        BLOCKED = 2,
    }
}

// --- 1. COMPLEX DESCRIPTOR TEST (bitstruct vs bytestruct vs std) ---
#[derive(Default)]
struct StandardDescriptor {
    pub is_privileged: bool,
    pub priority: u8,
    pub memory_pages: u16,
    pub state: u8, // std enum handling
}

bitstruct! {
    pub struct PackedDescriptor(u16) {
        pub is_privileged: bool = 1,
        pub priority: u8 = 3,
        pub memory_pages: u16 = 10,
        pub state: PerfState = 2,
    }
}

bytestruct! {
    pub struct ArrayDescriptor(2) {
        pub is_privileged: bool = 1,      // 1 bit
        pub priority: u8 = 3,             // 3 bits
        pub memory_pages: u16 = 10,       // 10 bits
        pub state: PerfState = 2,         // 2 bits
    }
}

// --- 2. ODD-WIDTH ID TEST (byteval vs std manual) ---
byteval! {
    pub struct PackedId24(3);
}

#[derive(Default)]
struct StandardId24([u8; 3]);

impl StandardId24 {
    #[inline(always)]
    fn set(&mut self, val: u32) {
        let bytes = val.to_le_bytes();
        self.0[0] = bytes[0];
        self.0[1] = bytes[1];
        self.0[2] = bytes[2];
    }

    #[inline(always)]
    fn get(&self) -> u32 {
        u32::from_le_bytes([self.0[0], self.0[1], self.0[2], 0])
    }
}

#[cfg(debug_assertions)]
const ITERATIONS: u64 = 1_000_000; // 1M for fast debug checks
#[cfg(not(debug_assertions))]
const ITERATIONS: u64 = 1_000_000_000; // 1B for release benchmarks

#[test]
fn test_comprehensive_performance() {
    println!("\n=== Running Comprehensive Performance Comparison ({} iterations) ===", ITERATIONS);

    // ----------------------------------------------------------------------
    // PART A: Complex Descriptor Comparisons
    // ----------------------------------------------------------------------
    println!("\n--- Part A: Complex Descriptors (4 fields, 16 bits total) ---");

    // 1. Standard Struct
    let mut standard = StandardDescriptor::default();
    let start_std = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 1024) as u16;
        standard.is_privileged = val % 2 == 0;
        standard.priority = (val % 8) as u8;
        standard.memory_pages = val;
        standard.state = (val % 3) as u8;

        black_box(&mut standard);
        black_box(standard.is_privileged);
        black_box(standard.priority);
        black_box(standard.memory_pages);
        black_box(standard.state);
    }
    let dur_std = start_std.elapsed();
    println!("Standard Struct Time:          {:?}", dur_std);

    // 2. bitstruct! (Register backed)
    let mut packed = PackedDescriptor::default();
    let start_packed = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 1024) as u16;
        packed.set_is_privileged(val % 2 == 0);
        packed.set_priority((val % 8) as u8);
        packed.set_memory_pages(val);
        packed.set_state(PerfState::from_bits((val % 3) as u8));

        black_box(&mut packed);
        black_box(packed.is_privileged());
        black_box(packed.priority());
        black_box(packed.memory_pages());
        black_box(packed.state());
    }
    let dur_packed = start_packed.elapsed();
    println!("bitstruct! (u16) Time:         {:?}", dur_packed);

    // 3. bytestruct! (Array backed)
    let mut array = ArrayDescriptor::default();
    let start_array = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 1024) as u16;
        array.set_is_privileged(val % 2 == 0);
        array.set_priority((val % 8) as u8);
        array.set_memory_pages(val);
        array.set_state(PerfState::from_bits((val % 3) as u8));

        black_box(&mut array);
        black_box(array.is_privileged());
        black_box(array.priority());
        black_box(array.memory_pages());
        black_box(array.state());
    }
    let dur_array = start_array.elapsed();
    println!("bytestruct! ([u8; 2]) Time:    {:?}", dur_array);

    println!("  -> bitstruct! Overhead:  {:.2}x", dur_packed.as_nanos() as f64 / dur_std.as_nanos() as f64);
    println!("  -> bytestruct! Overhead: {:.2}x", dur_array.as_nanos() as f64 / dur_std.as_nanos() as f64);

    // ----------------------------------------------------------------------
    // PART B: Odd-Width Type Comparisons (24-bit wrappers)
    // ----------------------------------------------------------------------
    println!("\n--- Part B: 24-bit Odd-Width IDs ([u8; 3]) ---");

    // 1. Standard Manual ID
    let mut std_id = StandardId24::default();
    let start_std_id = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 0xFFFFFF) as u32;
        std_id.set(val);
        black_box(&mut std_id);
        black_box(std_id.get());
    }
    let dur_std_id = start_std_id.elapsed();
    println!("Standard Manual byte-ops Time: {:?}", dur_std_id);

    // 2. byteval! Macro ID
    let mut byteval_id;
    let start_byteval_id = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 0xFFFFFF) as u32;
        byteval_id = PackedId24::from_u32(val);
        black_box(&mut byteval_id);
        black_box(byteval_id.to_u32());
    }
    let dur_byteval_id = start_byteval_id.elapsed();
    println!("byteval! Time:                 {:?}", dur_byteval_id);

    println!("  -> byteval! Overhead:    {:.2}x", dur_byteval_id.as_nanos() as f64 / dur_std_id.as_nanos() as f64);

    // ----------------------------------------------------------------------
    // PART C: Enum Comparisons (bitenum vs std enum)
    // ----------------------------------------------------------------------
    println!("\n--- Part C: Enum Comparisons (bitenum vs std enum) ---");

    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    enum StdEnum { A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7 }

    impl From<u8> for StdEnum {
        #[inline(always)]
        fn from(v: u8) -> Self {
            match v % 8 {
                0 => StdEnum::A, 1 => StdEnum::B, 2 => StdEnum::C, 3 => StdEnum::D,
                4 => StdEnum::E, 5 => StdEnum::F, 6 => StdEnum::G, _ => StdEnum::H,
            }
        }
    }

    bitenum! {
        pub enum BitEnum(3) { A=0, B=1, C=2, D=3, E=4, F=5, G=6, H=7 }
    }

    // 1. Standard Enum
    let start_std_enum = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 8) as u8;
        let e = StdEnum::from(val);
        let back = match e {
            StdEnum::A => 0, StdEnum::B => 1, StdEnum::C => 2, StdEnum::D => 3,
            StdEnum::E => 4, StdEnum::F => 5, StdEnum::G => 6, StdEnum::H => 7,
        };
        black_box(e);
        black_box(back);
    }
    let dur_std_enum = start_std_enum.elapsed();
    println!("Standard Enum Time:            {:?}", dur_std_enum);

    // 2. BitEnum
    let start_bit_enum = Instant::now();
    for i in 0..ITERATIONS {
        let val = (i % 8) as u8;
        let e = BitEnum::from_bits(val);
        let back = match e.to_bits() {
            0 => 0, 1 => 1, 2 => 2, 3 => 3,
            4 => 4, 5 => 5, 6 => 6, 7 => 7,
            _ => 0,
        };
        black_box(e);
        black_box(back);
    }
    let dur_bit_enum = start_bit_enum.elapsed();
    println!("bitenum! Time:                 {:?}", dur_bit_enum);

    println!("  -> bitenum! Overhead:    {:.2}x", dur_bit_enum.as_nanos() as f64 / dur_std_enum.as_nanos() as f64);
}
