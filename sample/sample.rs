use bitcraft::{bitenum, bitstruct, bytestruct, byteval};

bitenum! {
    /// A sample enumeration for status tracking.
    pub enum Status(2) {
        OFF = 0,
        ON = 1,
        FAULT = 2,
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
        .with_counter(0x1FFF);

    println!("\nTelemetry (56-bit): {:?}", telemetry);
    println!("Sensor A: 0x{:X}", telemetry.sensor_a());
    println!("Sensor B: 0x{:X}", telemetry.sensor_b());
    println!("Alert: {}", telemetry.alert());
    println!("Status: {:?}", telemetry.status());
    println!("Counter: 0x{:X}", telemetry.counter());
    println!("Bits: {}", Telemetry::BITS);

    let u16_packet = U16Packet::from_u64(0x112233445566);
    println!("\nU16Packet (48-bit, u16 units): {:?}", u16_packet);
    println!("Field A: 0x{:X}", u16_packet.field_a());
    println!("Field B: 0x{:X}", u16_packet.field_b());

    let u32_packet = U32Packet::from_u64(0xDEADBEEFCAFEBABE);
    println!("\nU32Packet (64-bit, u32 units): {:?}", u32_packet);
    println!("Header: 0x{:X}", u32_packet.header());
    println!("Body: 0x{:X}", u32_packet.body());

    let sid = SmallId::from_u8(0x7F);
    println!("\nSmallId: {:?} (Bits: {})", sid, SmallId::BITS);

    let id16 = Id16::from_u16(0xABCD);
    println!("Id16: {:?} (Bits: {})", id16, Id16::BITS);

    // 2. Demonstration of safe setters for byteval (New Feature)
    println!("\nTesting safe setters for PackedId (24-bit)...");
    let mut id = PackedId::from_u32(0x123456);
    // This should succeed
    if id.try_set_value(0x654321).is_ok() {
        println!("  Successfully set value to 0x654321");
    }
    // This should fail (24 bits = max 0xFFFFFF)
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

    let signed_coord = SignedCoordinate::default()
        .with_x(-1234)
        .with_y(5678);

    println!("\nSignedCoordinate: {:?}", signed_coord);
    println!("X: {}, Y: {}", signed_coord.x(), signed_coord.y());
}

