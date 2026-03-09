use bitcraft::{bitenum, bitstruct, bytestruct};

bitenum! {
    pub enum TestEnum(2) {
        A = 0,
        B = 1,
        C = 2,
    }
}

bitstruct! {
    pub struct TestStruct(u8) {
        pub a: bool = 1,
        pub b: TestEnum = 2,
    }
}

bytestruct! {
    pub struct TestArray(2) {
        pub x: u8 = 8,
        pub y: u8 = 8,
    }
}

#[test]
fn test_debug_formatting() {
    let e = TestEnum::B;
    let s_enum = format!("{:?}", e);
    println!("Enum Debug: {}", s_enum);
    assert!(s_enum.contains("TestEnum(1)::B"));

    let st = TestStruct::default().with_a(true).with_b(TestEnum::C);
    let s_struct = format!("{:?}", st);
    println!("Struct Debug: {}", s_struct);
    assert!(s_struct.contains("TestStruct"));
    assert!(s_struct.contains("a: true"));
    assert!(s_struct.contains("b: TestEnum(2)::C"));

    let arr = TestArray::from_bits(0x0201);
    let s_arr = format!("{:?}", arr);
    println!("Array Debug: {}", s_arr);
    assert!(s_arr.contains("TestArray"));
    assert!(s_arr.contains("x: 1"));
    assert!(s_arr.contains("y: 2"));
}
