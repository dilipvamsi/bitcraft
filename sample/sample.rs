use bitcraft::{
    atomic_bitarray, atomic_bitenum, atomic_bitstruct, bitarray, bitenum, bitstruct, bytearray,
    bytestruct, byteval,
};
use bitcraft::Ordering;

bitenum! {
    /// A sample enumeration for status tracking.
    pub enum Status(2) {
        OFF = 0,
        ON = 1,
        FAULT = 2,
    }
}

bitenum! {
    /// A sample signed enumeration using the (i bits) syntax.
    pub enum SignedStatus(i 2) {
        OFF = 0,
        ON = 1,
        FAULT = -1,
        ERROR = -2,
    }
}

bitstruct! {
    /// A sample 16-bit packed configuration.
    pub struct Config(u16) {
        pub enabled: bool = 1,
        pub mode: u8 = 3,
        pub status: Status = 2,
        pub data: u16 = 10,
    }
}

bitstruct! {
    /// A sample packed configuration using a signed integer base type.
    pub struct SignedConfig(i16) {
        pub enabled: bool = 1,
        pub mode: u8 = 3,
        pub status: Status = 2,
        pub data: u16 = 9,
    }
}

bytestruct! {
    /// A sample 5-byte dense coordinate.
    pub struct Coordinate(5) {
        pub x: u16 = 16,
        pub y: u16 = 16,
        pub flags: u8 = 8,
    }
}

bytestruct! {
    /// A more complex 7-byte telemetry packet.
    pub struct Telemetry(7) {
        pub sensor_a: u128 = 20,
        pub sensor_b: u128 = 20,
        pub alert: bool = 1,
        pub status: Status = 2,
        pub counter: u16 = 13,
    }
}

bytestruct! {
    /// A packet using u16 units (48 bits total).
    pub struct U16Packet([u16; 3]) {
        pub field_a: u32 = 24,
        pub field_b: u32 = 24,
    }
}

bytestruct! {
    /// A packet using u32 units (64 bits total).
    pub struct U32Packet([u32; 2]) {
        pub header: u8 = 8,
        pub body: u64 = 56,
    }
}

bitstruct! {
    /// A sample configuration with signed fields.
    pub struct SignedFieldsConfig(u64) {
        pub temperature: i8 = 8, // -128 to 127
        pub altitude: i16 = 16,  // -32768 to 32767
        pub offset_x: i32 = 20,  // -524288 to 524287
        pub status: SignedStatus = 2,
    }
}

bytestruct! {
    /// A sample bytestruct with signed fields.
    pub struct SignedCoordinate(4) {
        pub x: i16 = 16,
        pub y: i16 = 16,
    }
}

byteval! {
    /// A small 8-bit identifier (u8).
    pub struct SmallId(1);
}

byteval! {
    /// A 16-bit identifier (u16).
    pub struct Id16(2);
}

byteval! {
    /// A sample 24-bit identifier (default u8 unit).
    pub struct PackedId(3);
}

byteval! {
    /// A 40-bit identifier (u8 unit).
    pub struct Id40(5, u8);
}

byteval! {
    /// A sample 48-bit identifier (u16 unit).
    pub struct Id48(3, u16);
}

byteval! {
    /// A 24-bit signed identifier (default u8 unit).
    pub struct SignedId24(i 3);
}

byteval! {
    /// A sample 48-bit signed identifier (u16 unit).
    pub struct SignedId48(i 3, u16);
}

byteval! {
    /// A 64-bit identifier (u64).
    pub struct Id64(8);
}

byteval! {
    /// A sample 96-bit identifier with manual alignment.
    #[repr(align(4))]
    pub struct AlignedId96(3, u32);
}

byteval! {
    /// A huge 128-bit identifier (u128 unit).
    pub struct Id128(1, u128);
}

byteval! {
    /// A 128-bit identifier backed by two u64s.
    pub struct DualId128(2, u64);
}

atomic_bitstruct! {
    /// A lock-free atomic connection pool tracker.
    pub struct AtomicPoolTracker(AtomicU32) {
        pub is_active: bool = 1,
        pub active_connections: u16 = 15,
        pub status: Status = 2,
    }
}

atomic_bitenum! {
    /// A lock-free atomic status tracker.
    pub enum AtomicStatus(AtomicU32, 2) {
        OFF = 0,
        ON = 1,
        FAULT = 2,
    }
}

atomic_bitstruct! {
    /// A large 128-bit atomic metadata tracker.
    pub struct LargeAtomicTracker(AtomicU128) {
        pub is_active: bool = 1,
        pub user_id: u64 = 64,
        pub session_id: u32 = 32,
        pub flags: u32 = 31,
    }
}

atomic_bitenum! {
    /// A huge 128-bit atomic state.
    pub enum LargeAtomicState(AtomicU128, 128) {
        INITIAL = 0,
        READY = 1,
        ACTIVE = 2,
        TERMINATED = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    }
}

// ---------------------------------------------------------------------------
// bitarray! — register-backed packed arrays
// ---------------------------------------------------------------------------
bitarray! { pub struct NibblePalette(u 4, 8); } // 8 nibbles → u32
bitarray! { pub struct SignedOffsets(i 3, 10); } // 10 × 3-bit signed → u128
bitarray! { pub struct StatusFlags(bool, 32); } // 32 flags → u32
bitarray! { pub struct LargeBitFlags(bool, 128); } // 128 flags → u128

// ---------------------------------------------------------------------------
// bytearray! — byte-array-backed packed arrays
// ---------------------------------------------------------------------------
bytearray! { pub struct NibbleBuffer(u 4, 64); } // 64 nibbles → [u8; 32]
bytearray! { pub struct DeltaStream(i 7, 20); } // 20 × 7-bit signed → [u8; 18]
bytearray! { pub struct ByteFlagArray(bool, 128); } // 128 flags → [u8; 16]

// ---------------------------------------------------------------------------
// atomic_bitarray! — thread-safe packed arrays
// ---------------------------------------------------------------------------
atomic_bitarray! { pub struct AtomicNibbles(u 4, 16); } // 16 nibbles → AtomicU64
atomic_bitarray! { pub struct AtomicFlags128(bool, 128); } // 128 flags → AtomicU128

fn main() {
    let config = Config::default()
        .with_enabled(true)
        .with_mode(2)
        .with_status(Status::ON)
        .with_data(0x123);

    println!("Config: {:?}", config);
    println!("Enabled: {}", config.enabled());
    println!("Mode: {}", config.mode());
    println!("Status: {:?}", config.status());
    println!("Data: 0x{:X}", config.data());

    let coord = Coordinate::default()
        .with_x(100)
        .with_y(200)
        .with_flags(0xFF);

    println!("\nCoordinate: {:?}", coord);
    println!(
        "X: {}, Y: {}, Flags: 0x{:X}",
        coord.x(),
        coord.y(),
        coord.flags()
    );

    let telemetry = Telemetry::default()
        .with_sensor_a(0xABCDE)
        .with_sensor_b(0x12345)
        .with_alert(true)
        .with_status(Status::FAULT)
        .with_counter(999);

    println!("\nTelemetry: {:?}", telemetry);
    println!("Sensor A: 0x{:X}", telemetry.sensor_a());
    println!("Sensor B: 0x{:X}", telemetry.sensor_b());
    println!("Alert: {}", telemetry.alert());
    println!("Status: {:?}", telemetry.status());
    println!("Counter: {}", telemetry.counter());

    let u16_packet = U16Packet::default()
        .with_field_a(0x123456)
        .with_field_b(0x654321);

    println!("\nU16Packet: {:?}", u16_packet);
    println!("Field A: 0x{:X}", u16_packet.field_a());
    println!("Field B: 0x{:X}", u16_packet.field_b());

    let u32_packet = U32Packet::default()
        .with_header(0xAB)
        .with_body(0x00234567890ABCDE); // 56-bit max: 0x00FFFFFFFFFFFFFF

    println!("\nU32Packet: {:?}", u32_packet);
    println!("Header: 0x{:X}", u32_packet.header());
    println!("Body: 0x{:X}", u32_packet.body());

    let sid = SmallId::from_u8(0x7F);
    println!("\nSmallId: {:?} (Bits: {})", sid, SmallId::BITS);

    let id16 = Id16::from_u16(0xABCD);
    println!("Id16: {:?} (Bits: {})", id16, Id16::BITS);

    // 2. Demonstration of safe setters for byteval
    println!("\nTesting safe setters for PackedId (24-bit)...");
    let mut id = PackedId::from_u32(0x123456);
    if id.try_set_value(0x654321).is_ok() {
        println!("  Successfully set value to 0x654321");
    }
    if let Err(e) = id.try_set_value(0x1000000) {
        println!("  Correctly failed to set value 0x1000000: {:?}", e);
    }
    let id2 = id.try_with_value(0xABCDEF).unwrap();
    println!("  id2 value: 0x{:X}", id2.value());

    let id = PackedId::from_u32(0xABCDEF);
    println!("PackedId: {:?} (Bits: {})", id, PackedId::BITS);
    println!("Value: 0x{:X}", id.value());

    let id40 = Id40::from_u64(0x123456789A);
    println!("Id40: {:?} (Bits: {})", id40, Id40::BITS);
    println!("Value: 0x{:X}", id40.value());

    let id48 = Id48::from_u64(0x112233445566);
    println!("Id48: {:?} (Bits: {})", id48, Id48::BITS);
    println!("Value: 0x{:X}", id48.value());

    let id64 = Id64::from_u64(0xDEADBEEFCAFEBABE);
    println!("Id64: {:?} (Bits: {})", id64, Id64::BITS);

    let al_id96 = AlignedId96::from_bits(0xDEADBEEFCAFEBABE11223344);
    println!("\nAlignedId96: {:?} (Bits: {})", al_id96, AlignedId96::BITS);
    println!("Value: 0x{:X}", al_id96.value());
    println!("Alignment: {}", std::mem::align_of::<AlignedId96>());
    println!("Size: {}", std::mem::size_of::<AlignedId96>());

    let id128 = Id128::from_u128(0x0123456789ABCDEF0123456789ABCDEF);
    println!("\nId128: {:?} (Bits: {})", id128, Id128::BITS);
    println!("Value: 0x{:X}", id128.value());

    let dual_id128 = DualId128::from_u128(0xDEADBEEFCAFEBABEDEADBEEFCAFEBABE);
    println!("\nDualId128: {:?} (Bits: {})", dual_id128, DualId128::BITS);
    println!("Value: 0x{:X}", dual_id128.value());

    // 3. Demonstration of signed fields
    let signed_cfg = SignedFieldsConfig::default()
        .with_temperature(-40)
        .with_altitude(12000)
        .with_offset_x(-250000);

    println!("\nSignedFieldsConfig: {:?}", signed_cfg);
    println!("Temperature: {}", signed_cfg.temperature());
    println!("Altitude: {}", signed_cfg.altitude());
    println!("Offset X: {}", signed_cfg.offset_x());

    let signed_coord = SignedCoordinate::default().with_x(-1234).with_y(5678);

    println!("\nSignedCoordinate: {:?}", signed_coord);
    println!("X: {}, Y: {}", signed_coord.x(), signed_coord.y());

    // 4. Demonstration of atomic_bitstruct
    println!("\nTesting atomic_bitstruct lock-free transactions...");
    let atomic_pool = AtomicPoolTracker::new(0);
    atomic_pool.set_is_active(true, Ordering::Release);

    // Simulate taking a connection conditionally
    let res = atomic_pool.update_or_abort(Ordering::SeqCst, Ordering::Relaxed, |v| {
        let current = v.active_connections();
        if current >= 50 {
            return None; // Abort if pool is heavily loaded!
        }
        v.set_active_connections(current + 1);
        v.set_status(Status::ON);
        Some(())
    });

    if res.is_ok() {
        println!(
            "  Successfully took connection! Total active: {}",
            atomic_pool.active_connections(Ordering::Acquire)
        );
    }

    // Fetch snapshot
    let snapshot = atomic_pool.get(Ordering::Acquire);
    println!(
        "  Atomic Snapshot -> Active: {}, Status: {:?}",
        snapshot.is_active(),
        snapshot.status()
    );

    // 5. Demonstration of atomic_bitenum
    println!("\nTesting atomic_bitenum...");
    let atomic_status = AtomicStatus::new(AtomicStatusValue::OFF);
    atomic_status.store(AtomicStatusValue::ON, Ordering::SeqCst);
    println!(
        "  Status after store: {:?}",
        atomic_status.load(Ordering::SeqCst)
    );

    atomic_status.update(Ordering::SeqCst, Ordering::Relaxed, |v| {
        if v == AtomicStatusValue::ON {
            AtomicStatusValue::FAULT
        } else {
            v
        }
    });
    println!(
        "  Status after update: {:?}",
        atomic_status.load(Ordering::SeqCst)
    );

    // 6. Demonstration of 128-bit atomics
    println!("\nTesting 128-bit atomics...");
    let large_tracker = LargeAtomicTracker::new(0);
    large_tracker.set_user_id(0xDEADBEEFCAFEBABE, Ordering::SeqCst);
    large_tracker.set_session_id(0x12345678, Ordering::SeqCst);
    large_tracker.set_is_active(true, Ordering::SeqCst);

    println!(
        "  LargeAtomicTracker -> UserID: 0x{:X}, SessionID: 0x{:X}, Active: {}",
        large_tracker.user_id(Ordering::Acquire),
        large_tracker.session_id(Ordering::Acquire),
        large_tracker.is_active(Ordering::Acquire)
    );

    let large_state = LargeAtomicState::new(LargeAtomicStateValue::INITIAL);
    large_state.store(LargeAtomicStateValue::TERMINATED, Ordering::SeqCst);
    println!(
        "  LargeAtomicState: {:?}",
        large_state.load(Ordering::SeqCst)
    );

    // 7. Demonstration of bitarray!
    println!("\n--- bitarray! ---");

    let mut palette = NibblePalette::default();
    palette.set(0, 0xA);
    palette.set(7, 0xF);
    println!("NibblePalette: {:?}", palette);
    println!("  palette[0] = 0x{:X}", palette.get(0));
    println!("  palette[7] = 0x{:X}", palette.get(7));
    println!(
        "  size_of    = {} bytes",
        core::mem::size_of::<NibblePalette>()
    );

    let mut offsets = SignedOffsets::default();
    offsets.set(0, -3);
    offsets.set(1, 2);
    offsets.set(9, -4);
    println!("SignedOffsets: {:?}", offsets);
    println!("  offsets[0] = {}", offsets.get(0));
    println!("  offsets[1] = {}", offsets.get(1));
    println!("  offsets[9] = {}", offsets.get(9));

    let mut flags = StatusFlags::default();
    flags.set(0, true);
    flags.set(15, true);
    flags.set(31, true);
    println!("StatusFlags bits: 0b{:032b}", flags.to_bits());
    println!("  flags[0]  = {}", flags.get(0));
    println!("  flags[15] = {}", flags.get(15));
    println!("  flags[31] = {}", flags.get(31));
    println!(
        "  size_of   = {} bytes",
        core::mem::size_of::<StatusFlags>()
    );

    // bytemuck: cast raw u32 → StatusFlags
    let raw_flags: u32 = 0b1000_0000_0000_0001;
    let cast_flags: StatusFlags = bytemuck::cast(raw_flags);
    println!(
        "  cast from 0x{raw_flags:08X}: bit[0]={} bit[15]={}",
        cast_flags.get(0),
        cast_flags.get(15)
    );

    // 7. Demonstration of bytearray!
    println!("\n--- bytearray! ---");

    let mut buf = NibbleBuffer::default();
    buf.set(0, 0xD);
    buf.set(63, 0xE);
    println!("NibbleBuffer: {:?}", buf);
    println!("  buf[0]   = 0x{:X}", buf.get(0));
    println!("  buf[63]  = 0x{:X}", buf.get(63));
    println!("  BYTES    = {}", NibbleBuffer::BYTES);
    println!(
        "  size_of  = {} bytes",
        core::mem::size_of::<NibbleBuffer>()
    );

    let mut stream = DeltaStream::default();
    stream.set(0, -64);
    stream.set(1, 63);
    stream.set(19, -1);
    println!("DeltaStream: {:?}", stream);
    println!("  stream[0]  = {}", stream.get(0));
    println!("  stream[1]  = {}", stream.get(1));
    println!("  stream[19] = {}", stream.get(19));
    println!("  BYTES      = {}", DeltaStream::BYTES);

    let mut byte_flags = ByteFlagArray::default();
    byte_flags.set(0, true);
    byte_flags.set(63, true);
    byte_flags.set(127, true);
    println!(
        "ByteFlagArray: bit[0]={} bit[63]={} bit[127]={}",
        byte_flags.get(0),
        byte_flags.get(63),
        byte_flags.get(127)
    );
    println!("  BYTES   = {}", ByteFlagArray::BYTES);
    println!(
        "  size_of = {} bytes",
        core::mem::size_of::<ByteFlagArray>()
    );

    // bytemuck: cast [u8; 16] → ByteFlagArray and LargeBitFlags
    let raw_bytes = [0xFFu8; 16]; // all bits set
    let bf: ByteFlagArray = bytemuck::cast(raw_bytes);
    let lbf: LargeBitFlags = bytemuck::cast(raw_bytes);
    println!("  After casting 0xFF×16:");
    println!("  ByteFlagArray bit[100]  = {}", bf.get(100));
    println!("  LargeBitFlags bit[100]  = {}", lbf.get(100));

    // 8. Demonstration of atomic_bitarray!
    println!("\n--- atomic_bitarray! ---");
    let atomic_nibbles = AtomicNibbles::new(0);
    atomic_nibbles.set(0, 0xA, Ordering::SeqCst);
    atomic_nibbles.set(15, 0xF, Ordering::SeqCst);
    println!("AtomicNibbles:");
    println!("  [0] = 0x{:X}", atomic_nibbles.get(0, Ordering::Relaxed));
    println!("  [15] = 0x{:X}", atomic_nibbles.get(15, Ordering::Relaxed));

    atomic_nibbles.update(Ordering::SeqCst, Ordering::Relaxed, |snap| {
        let val: u128 = snap.get(0);
        snap.set(0, val + 1);
    });
    println!(
        "  [0] after update = 0x{:X}",
        atomic_nibbles.get(0, Ordering::Relaxed)
    );

    let atomic_flags = AtomicFlags128::new(0);
    atomic_flags.set(0, true, Ordering::SeqCst);
    atomic_flags.set(127, true, Ordering::SeqCst);
    println!("AtomicFlags128:");
    println!("  [0] = {}", atomic_flags.get(0, Ordering::Relaxed));
    println!("  [127] = {}", atomic_flags.get(127, Ordering::Relaxed));
}
