use bitcraft::byteval;

// 3 * u16 = 6 bytes (48 bits)
byteval! {
    pub struct Id48(3, u16);
}

// 2 * u32 = 8 bytes (64 bits)
byteval! {
    struct Id64(2, u32);
}

// Custom alignment (manual attribute)
byteval! {
    #[repr(align(2))]
    struct AlignedId48(3, u16);
}

byteval! {
    #[repr(align(8))]
    struct AlignedId64(2, u32);
}

// Shorthand (defaults to u8)
byteval! {
    struct Id24(3);
}

#[test]
fn test_new_byteval_syntax() {
    let id = Id48::from_u64(0x112233445566);
    assert_eq!(id.value(), 0x112233445566);
    assert_eq!(std::mem::size_of::<Id48>(), 6);
    assert_eq!(std::mem::align_of::<Id48>(), 2);

    let id64 = Id64::from_u64(0xDEADBEEFCAFEBABE);
    assert_eq!(id64.value(), 0xDEADBEEFCAFEBABE);
    assert_eq!(std::mem::size_of::<Id64>(), 8);
}

#[test]
fn test_manual_alignment() {
    assert_eq!(std::mem::align_of::<AlignedId48>(), 2);
    assert_eq!(std::mem::size_of::<AlignedId48>(), 6);

    assert_eq!(std::mem::align_of::<AlignedId64>(), 8);
    assert_eq!(std::mem::size_of::<AlignedId64>(), 8);
}

#[test]
fn test_default_u8() {
    let id = Id24::from_u32(0xABCDEF);
    assert_eq!(id.value(), 0xABCDEF);
    assert_eq!(std::mem::size_of::<Id24>(), 3);
}
