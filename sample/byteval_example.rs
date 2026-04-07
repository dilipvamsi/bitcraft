use bitcraft::byteval;

// Original shorthand: 3 bytes (24 bits)
byteval! {
    pub struct Id24(3);
}

// New syntax: 3 * u16 = 6 bytes (48 bits)
byteval! {
    pub struct Id48(3, u16);
}

// Custom alignment (manual attribute)
#[repr(align(8))]
byteval! {
    pub struct AlignedId48(3, u16);
}

// New syntax: 2 * u32 = 8 bytes (64 bits)
byteval! {
    pub struct Id64(2, u32);
}

fn main() {
    let id_24 = Id24::from_u32(0xABCDEF);
    println!("Id24: {:?}", id_24);

    let id_48 = Id48::from_u64(0x112233445566);
    println!("Id48: {:?}", id_48);

    let aligned_id_48 = AlignedId48::from_u64(0x112233445566);
    println!("AlignedId48: {:?}", aligned_id_48);
    println!("Alignment: {}", std::mem::align_of::<AlignedId48>());

    let id_64 = Id64::from_u64(0xDEADBEEFCAFEBABE);
    println!("Id64: {:?}", id_64);
}
