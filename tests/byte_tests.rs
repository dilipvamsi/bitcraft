use bitcraft::{BitstructError, bitenum, bitstruct, bytestruct, byteval};

#[test]
fn test_bitstruct_error_display() {
    let err1 = BitstructError::Overflow {
        value: 10,
        allocated_bits: 3,
    };
    assert_eq!(
        format!("{}", err1),
        "BitstructError: Value 10 overflows allocated 3 bits"
    );

    let err2 = BitstructError::InvalidVariant {
        value: 5,
        enum_name: "TestEnum",
    };
    assert_eq!(
        format!("{}", err2),
        "BitstructError: Value 5 is not a valid variant for enum TestEnum"
    );

    // Test that it implements std::error::Error
    use std::error::Error;
    assert!(err1.source().is_none());
}

#[test]
fn test_bitstruct_debug() {
    bitstruct! {
        struct DebugStruct(u8) {
            a: bool = 1,
            b: u8 = 7,
        }
    }
    let s = DebugStruct::from_bits(0xAB);
    let debug_str = format!("{:?}", s);
    assert!(debug_str.contains("DebugStruct"));
    assert!(debug_str.contains("raw: 171"));
    assert!(debug_str.contains("a: true"));
    assert!(debug_str.contains("b: 85")); // 171 >> 1 = 85
}

#[test]
fn test_bytestruct_debug() {
    bytestruct! {
        struct DebugByteStruct(2) {
            a: u8 = 8,
            b: u8 = 8,
        }
    }
    let s = DebugByteStruct::from_u16(0x1234);
    let debug_str = format!("{:?}", s);
    assert!(debug_str.contains("DebugByteStruct"));
    assert!(debug_str.contains("raw: [52, 18]")); // 0x34, 0x12
    assert!(debug_str.contains("a: 52"));
    assert!(debug_str.contains("b: 18"));
}

#[test]
fn test_bitenum_debug_and_unknown() {
    bitenum! {
        enum TestEnum(2) {
            A = 0,
            B = 1,
        }
    }
    let a = TestEnum::A;
    assert_eq!(format!("{:?}", a), "TestEnum(0)::A");

    // Test UNKNOWN case in Debug
    let unknown = TestEnum(3);
    assert_eq!(format!("{:?}", unknown), "TestEnum(3)::UNKNOWN");
}

#[test]
fn test_bitenum_is_defined() {
    bitenum! {
        enum TestEnum(2) {
            A = 0,
            B = 1,
        }
    }
    assert!(TestEnum::A.is_defined());
    assert!(TestEnum::B.is_defined());
    assert!(!TestEnum(3).is_defined());
}

#[test]
fn test_bytestruct_all_byte_sizes_fields() {
    // Testing 1 byte to 16 bytes with fields that Span the entire width
    // This hits all if len > N branches in read/write_localized_prim

    macro_rules! test_size {
        ($n:expr, $type:ty) => {
            paste::paste! {
                bytestruct! {
                    struct [<TestSize $n>]($n) {
                        v: $type = ($n * 8),
                    }
                }
                let mut s = [<TestSize $n>]::default();
                                let val: u128 = if $n <= 8 {
                    if $n == 8 { 0xFFFFFFFFFFFFFFFFu64 as u128 } else { ((1u64 << ($n * 8)) - 1) as u128 }
                } else {
                    if $n == 16 { 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128 } else { (1u128 << ($n * 8)) - 1 }
                };

                s.set_v(val as $type);
                assert_eq!(s.v(), val as $type);

                let s2 = [<TestSize $n>]::default().[<with_ v>](val as $type);
                assert_eq!(s2.v(), val as $type);

                assert_eq!(s.[<try_set_ v>](val as $type), Ok(()));
                assert_eq!(s.[<try_with_ v>](val as $type).unwrap().v(), val as $type);

                // Overflow test
                if ($n * 8) < (std::mem::size_of::<$type>() * 8) {
                    let overflow_val = val as u128 + 1;
                    assert!(s.[<try_set_ v>](overflow_val as $type).is_err());
                    assert!(s.[<try_with_ v>](overflow_val as $type).is_err());
                }
            }
        };
    }

    test_size!(1, u8);
    test_size!(2, u16);
    test_size!(3, u32);
    test_size!(4, u32);
    test_size!(5, u64);
    test_size!(6, u64);
    test_size!(7, u64);
    test_size!(8, u64);
    test_size!(9, u128);
    test_size!(10, u128);
    test_size!(11, u128);
    test_size!(12, u128);
    test_size!(13, u128);
    test_size!(14, u128);
    test_size!(15, u128);
    test_size!(16, u128);
}

#[test]
fn test_bytestruct_edge_cases() {
    bytestruct! {
        struct Edge(9) {
            a: bool = 1,
            b: u128 = 71,
        }
    }
    let mut e = Edge::default();
    e.set_a(true);
    e.set_b(0x1234567890ABCDE);
    assert!(e.a());
    assert_eq!(e.b(), 0x1234567890ABCDE);

    let e2 = Edge::default().with_a(true).with_b(0x1);
    assert!(e2.a());
    assert_eq!(e2.b(), 0x1);
}

#[test]
fn test_bytestruct_enum_overflow() {
    bitenum! { enum E(2) { A = 0, B = 1 } }
    bytestruct! {
        struct EnumOv(2) {
            f: E = 2,
        }
    }
    let mut s = EnumOv::default();
    assert_eq!(s.try_set_f(E::B), Ok(()));

}

#[test]
fn test_bitstruct_bool_try() {
    bitstruct! {
        struct BoolTry(u8) {
            b: bool = 1,
        }
    }
    let mut s = BoolTry::default();
    assert_eq!(s.try_set_b(true), Ok(()));
    assert!(!s.try_with_b(false).unwrap().b());
}

#[test]
fn test_byteval_variations_coverage() {
    byteval! { struct V1(1); }
    byteval! { struct V2(2); }
    byteval! { struct V3(3); }
    byteval! { struct V4(4); }
    byteval! { struct V5(5); }
    byteval! { struct V6(6); }
    byteval! { struct V7(7); }
    byteval! { struct V8(8); }
    byteval! { struct V9(9); }
    byteval! { struct V10(10); }
    byteval! { struct V11(11); }
    byteval! { struct V12(12); }
    byteval! { struct V13(13); }
    byteval! { struct V14(14); }
    byteval! { struct V15(15); }
    byteval! { struct V16(16); }

    assert_eq!(V1::from_u8(0xFF).value(), 0xFF);
    assert_eq!(V2::from_u16(0xFFFF).value(), 0xFFFF);
    assert_eq!(V3::from_u32(0xFFFFFF).value(), 0xFFFFFF);
    assert_eq!(V4::from_u32(0xFFFFFFFF).value(), 0xFFFFFFFF);
    assert_eq!(V5::from_u64(0xFFFFFFFFFF).value(), 0xFFFFFFFFFF);
    assert_eq!(V6::from_u64(0xFFFFFFFFFFFF).value(), 0xFFFFFFFFFFFF);
    assert_eq!(V7::from_u64(0xFFFFFFFFFFFFFF).value(), 0xFFFFFFFFFFFFFF);
    assert_eq!(V8::from_u64(0xFFFFFFFFFFFFFFFF).value(), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(
        V9::from_u128(0xFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V10::from_u128(0xFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V11::from_u128(0xFFFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V12::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V13::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V14::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V15::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    );
    assert_eq!(
        V16::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF).value(),
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    );
}

#[test]
fn test_bytestruct_localized_prim_cross_byte() {
    // Test a field that is NOT byte-aligned and spans multiple bytes
    bytestruct! {
        struct Cross(3) {
            pad: u8 = 4,
            field: u16 = 12,
        }
    }
    let mut c = Cross::default();
    c.set_field(0xFFF);
    assert_eq!(c.field(), 0xFFF);
    assert_eq!(c.pad(), 0);

    // Verify localized prim read/write logic for shifted offsets
    c.set_pad(0xA);
    assert_eq!(c.pad(), 0xA);
    assert_eq!(c.field(), 0xFFF);
}
