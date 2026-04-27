use bitcraft::bitstruct;

bitstruct! {
    struct I8Struct(i8) {
        a: bool = 1,
        b: u8 = 6, // 1 + 6 = 7 bits (max for i8)
    }
}

bitstruct! {
    struct I16Struct(i16) {
        x: u8 = 4,
        y: u16 = 11, // 4 + 11 = 15 bits (max for i16)
    }
}

bitstruct! {
    struct I64Struct(i64) {
        a: u32 = 32,
        b: u32 = 31, // 63 bits (max for i64)
    }
}

bitstruct! {
    struct I128Struct(i128) {
        a: u64 = 64,
        b: u64 = 63, // 127 bits (max for i128)
    }
}

#[test]
fn test_i8_struct() {
    let mut s = I8Struct::from_bits(-1); // All 8 bits are 1
    assert_eq!(s.a(), true);
    assert_eq!(s.b(), 0b0011_1111);

    // Setting values should not affect the sign bit
    s.set_a(false);
    s.set_b(0);
    // Bits 0-6 are 0, Bit 7 (sign) is still 1
    assert_eq!(s.to_bits(), -128); // 1000_0000
}

#[test]
fn test_i16_struct() {
    let mut s = I16Struct::from_bits(0);
    s.set_x(0xF);
    s.set_y(0x7FF);
    assert_eq!(s.to_bits(), 0x7FFF); // 15 bits set
}

#[test]
fn test_i64_struct() {
    let s = I64Struct::from_bits(-1) // All 64 bits 1
        .with_a(0)
        .with_b(0);
    // Bits 0-62 are 0, Bit 63 (sign) is still 1
    assert_eq!(s.to_bits(), i64::MIN);
}

#[test]
fn test_i128_struct() {
    let s = I128Struct::default()
        .with_a(0x1234567890ABCDEF)
        .with_b(0x7FFFFFFFFFFFFFFF);
    
    assert_eq!(s.a(), 0x1234567890ABCDEF);
    assert_eq!(s.b(), 0x7FFFFFFFFFFFFFFF);
}
