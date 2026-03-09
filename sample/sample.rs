use bitstruct::{bitenum, bitstruct, bytestruct, byteval};

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

bytestruct! {
    /// A sample 5-byte dense coordinate.
    pub struct Coordinate(5) {
        pub x: u16 = 16,
        pub y: u16 = 16,
        pub flags: u8 = 8,
    }
}

byteval! {
    /// A sample 24-bit identifier.
    pub struct PackedId(3);
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

    let id = PackedId::from_u32(0xABCDEF);
    println!("\nPackedId: {:?}", id);
    println!("Value: 0x{:X}", id.value());
}
