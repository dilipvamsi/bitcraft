use bitstruct::{bitstruct, bytestruct, byteval, bitenum};

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
