use bitcraft::{bitenum, bitstruct};

bitenum! {
    pub enum Status(i 3) {
        OFF = 0,
        ON = 1,
        FAULT = -1,
        ERROR = -2,
    }
}

bitstruct! {
    pub struct StatusConfig(u8) {
        pub mode: Status = 3,
        pub enabled: bool = 1,
    }
}

#[test]
fn test_signed_enum() {
    let mut config = StatusConfig::default();

    // Set to positive
    config.set_mode(Status::ON);
    assert_eq!(config.mode(), Status::ON);
    assert_eq!(config.mode().to_bits(), 1);

    // Set to negative
    config.set_mode(Status::FAULT);
    assert_eq!(config.mode(), Status::FAULT);
    assert_eq!(config.mode().to_bits(), -1);

    // Check struct extraction mapping
    assert_eq!(config.0, 0b00000111); // FAULT is -1 in 3 bits (111)
}

#[test]
fn test_signed_enum_from_bits() {
    // Native bounds for i3: -4 to 3
    let s = Status::try_from_bits(-1).unwrap();
    assert_eq!(s, Status::FAULT);

    let s = Status::try_from_bits(-2).unwrap();
    assert_eq!(s, Status::ERROR);
}
