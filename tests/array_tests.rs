use bitcraft::{bitarray, bytearray};

bitarray! {
    pub struct NibbleArray(u 4, 8);   // → pub struct NibbleArray(pub u32)
}

bitarray! {
    pub struct SignedBitArray(i 3, 10);
}

bitarray! {
    pub struct FlagArray(bool, 32);   // → pub struct FlagArray(pub u32)
}

bytearray! {
    pub struct LargeNibbleArray(u 4, 64); // 4*64=256 bits → [u8; 32]
}

bytearray! {
    pub struct LargeSignedArray(i 7, 20); // 7*20=140 bits → [u8; 18]
}

bytearray! {
    pub struct LargeFlagArray(bool, 128); // 128 bits → [u8; 16]
}

bytearray! {
    pub struct LargeByteFlagArray(bool, 128); // [u8; 16] backed
}

bitarray! {
    pub struct LargeBitFlagArray(bool, 128); // u128 backed (also 16 bytes)
}

#[test]
fn test_bitarray_basic() {
    let mut arr = NibbleArray::default();
    assert_eq!(arr.to_bits(), 0u32);

    arr.set(0, 0xA);
    arr.set(7, 0xF);

    assert_eq!(arr.get(0), 0xA_u128);
    assert_eq!(arr.get(7), 0xF_u128);
    assert_eq!(arr.get(1), 0_u128);

    // Check internal layout (LSB first):
    // 0xF at index 7 (bits 28-31), 0xA at index 0 (bits 0-3)
    assert_eq!(arr.to_bits(), (0xF << 28) | 0xA);
}

#[test]
fn test_bitarray_signed() {
    let mut arr = SignedBitArray::default();

    // 3 bits can hold -4 to 3
    arr.set(0, -1);
    arr.set(1, -4);
    arr.set(2, 3);
    arr.set(3, 0);

    assert_eq!(arr.get(0), -1);
    assert_eq!(arr.get(1), -4);
    assert_eq!(arr.get(2), 3);
    assert_eq!(arr.get(3), 0);
}

#[test]
fn test_bitarray_bool() {
    let mut arr = FlagArray::default();
    arr.set(0, true);
    arr.set(31, true);

    assert!(arr.get(0));
    assert!(arr.get(31));
    assert!(!arr.get(1));
}

#[test]
fn test_bytearray_basic() {
    let mut arr = LargeNibbleArray::default();
    arr.set(0, 0xD);
    arr.set(63, 0xE);

    assert_eq!(arr.get(0), 0xD_u128);
    assert_eq!(arr.get(63), 0xE_u128);
    assert_eq!(arr.get(32), 0_u128);
}

#[test]
fn test_bytearray_signed_crossing_boundaries() {
    // 7 bits each. Element 1 starts at bit 7 (byte 0 bit 7, byte 1 bits 0-5)
    let mut arr = LargeSignedArray::default();

    arr.set(0, -64); // Min 7-bit signed
    arr.set(1, 63);  // Max 7-bit signed
    arr.set(19, -1);

    assert_eq!(arr.get(0), -64);
    assert_eq!(arr.get(1), 63);
    assert_eq!(arr.get(19), -1);
}

#[test]
fn test_bytearray_bool() {
    let mut arr = LargeFlagArray::default();

    for i in 0..128 {
        assert!(!arr.get(i), "bit {i} should be false by default");
    }

    arr.set(0, true);
    arr.set(63, true);
    arr.set(127, true);

    assert!(arr.get(0));
    assert!(!arr.get(1));
    assert!(arr.get(63));
    assert!(!arr.get(64));
    assert!(arr.get(127));

    arr.set(63, false);
    assert!(!arr.get(63));

    // Verify storage size constants
    assert_eq!(LargeFlagArray::BYTES, 16);
    assert_eq!(LargeFlagArray::ELEMENT_COUNT, 128);
    assert_eq!(LargeFlagArray::ELEMENT_WIDTH, 1);
}

#[test]
fn test_bytearray_bytemuck_cast() {
    // --- bytemuck::cast: [u8; 32] ↔ LargeNibbleArray ---
    let raw = [0xABu8; 32];
    let arr: LargeNibbleArray = bytemuck::cast(raw);
    assert_eq!(arr.0, raw);

    let borrowed: &LargeNibbleArray = bytemuck::from_bytes(&raw);
    assert_eq!(borrowed.0, raw);

    let round_tripped: [u8; 32] = bytemuck::cast(arr);
    assert_eq!(round_tripped, raw);

    let zeroed: LargeNibbleArray = bytemuck::Zeroable::zeroed();
    assert_eq!(zeroed.0, [0u8; 32]);

    let flag_bytes = [0b1010_1010u8; 16];
    let flags: LargeFlagArray = bytemuck::cast(flag_bytes);
    assert_eq!(flags.0, flag_bytes);
}

#[test]
fn test_bitarray_bytemuck_cast() {
    // NibbleArray(u 4, 8) → backed by u32 (32 bits exact match)
    let raw: u32 = 0xDEAD_BEEF;
    let arr: NibbleArray = bytemuck::cast(raw);
    assert_eq!(arr.to_bits(), raw);

    let round_tripped: u32 = bytemuck::cast(arr);
    assert_eq!(round_tripped, raw);

    let zeroed: NibbleArray = bytemuck::Zeroable::zeroed();
    assert_eq!(zeroed.to_bits(), 0u32);
    assert_eq!(zeroed, NibbleArray::default());

    // FlagArray(bool, 32) → backed by u32
    let raw_flags: u32 = u32::MAX;
    let flags: FlagArray = bytemuck::cast(raw_flags);
    assert_eq!(flags.to_bits(), raw_flags);

    for i in 0..32 {
        assert!(flags.get(i), "bit {i} should be true");
    }
}

#[test]
fn test_array_sizes() {
    use core::mem::size_of;

    // --- bitarray: backed by smallest uX that fits ---
    // NibbleArray(u 4, 8)   → 4*8=32 bits  → u32  (4 bytes)
    assert_eq!(size_of::<NibbleArray>(), 4);
    assert_eq!(NibbleArray::TOTAL_BITS, 32);
    assert_eq!(NibbleArray::ELEMENT_WIDTH, 4);
    assert_eq!(NibbleArray::ELEMENT_COUNT, 8);

    // SignedBitArray(i 3, 10) → 3*10=30 bits → fallback u128 (16 bytes)
    assert_eq!(size_of::<SignedBitArray>(), 16);
    assert_eq!(SignedBitArray::TOTAL_BITS, 30);
    assert_eq!(SignedBitArray::ELEMENT_WIDTH, 3);
    assert_eq!(SignedBitArray::ELEMENT_COUNT, 10);

    // FlagArray(bool, 32) → 1*32=32 bits → u32 (4 bytes)
    assert_eq!(size_of::<FlagArray>(), 4);
    assert_eq!(FlagArray::TOTAL_BITS, 32);
    assert_eq!(FlagArray::ELEMENT_WIDTH, 1);
    assert_eq!(FlagArray::ELEMENT_COUNT, 32);

    // --- bytearray: backed by [u8; ceil(total/8)] ---
    // LargeNibbleArray(u 4, 64) → 4*64=256 bits → [u8; 32]
    assert_eq!(size_of::<LargeNibbleArray>(), 32);
    assert_eq!(LargeNibbleArray::BYTES, 32);
    assert_eq!(LargeNibbleArray::TOTAL_BITS, 256);
    assert_eq!(LargeNibbleArray::ELEMENT_WIDTH, 4);
    assert_eq!(LargeNibbleArray::ELEMENT_COUNT, 64);

    // LargeSignedArray(i 7, 20) → 7*20=140 bits → ceil(140/8)=18 → [u8; 18]
    assert_eq!(size_of::<LargeSignedArray>(), 18);
    assert_eq!(LargeSignedArray::BYTES, 18);
    assert_eq!(LargeSignedArray::TOTAL_BITS, 140);
    assert_eq!(LargeSignedArray::ELEMENT_WIDTH, 7);
    assert_eq!(LargeSignedArray::ELEMENT_COUNT, 20);

    // LargeFlagArray(bool, 128) → 1*128=128 bits → [u8; 16]
    assert_eq!(size_of::<LargeFlagArray>(), 16);
    assert_eq!(LargeFlagArray::BYTES, 16);
    assert_eq!(LargeFlagArray::TOTAL_BITS, 128);
    assert_eq!(LargeFlagArray::ELEMENT_WIDTH, 1);
    assert_eq!(LargeFlagArray::ELEMENT_COUNT, 128);
}

#[test]
fn test_cast_from_bytes() {
    // Source: a raw [u8; 16] with a known bit pattern.
    // Bits set: 0, 7, 8, 15, 63, 64, 127  (scattered across byte boundaries)
    let mut raw = [0u8; 16];
    raw[0]  = 0b1000_0001; // bits 0 and 7
    raw[1]  = 0b1000_0001; // bits 8 and 15
    raw[7]  = 0b1000_0000; // bit 63
    raw[8]  = 0b0000_0001; // bit 64
    raw[15] = 0b1000_0000; // bit 127

    // --- LargeByteFlagArray: bytearray backed by [u8; 16] ---
    // Direct cast: [u8; 16] → LargeByteFlagArray (same backing memory)
    let byte_arr: LargeByteFlagArray = bytemuck::cast(raw);
    assert_eq!(byte_arr.0, raw, "backing bytes must match exactly");
    assert!(byte_arr.get(0),   "bit 0 should be set");
    assert!(byte_arr.get(7),   "bit 7 should be set");
    assert!(byte_arr.get(8),   "bit 8 should be set");
    assert!(byte_arr.get(15),  "bit 15 should be set");
    assert!(byte_arr.get(63),  "bit 63 should be set");
    assert!(byte_arr.get(64),  "bit 64 should be set");
    assert!(byte_arr.get(127), "bit 127 should be set");
    assert!(!byte_arr.get(1),  "bit 1 should be clear");
    assert!(!byte_arr.get(100),"bit 100 should be clear");

    // --- LargeBitFlagArray: bitarray backed by u128 (also 16 bytes) ---
    // Direct cast: [u8; 16] → LargeBitFlagArray (Pod, same size)
    let bit_arr: LargeBitFlagArray = bytemuck::cast(raw);
    assert!(bit_arr.get(0),   "bit 0 should be set");
    assert!(bit_arr.get(7),   "bit 7 should be set");
    assert!(bit_arr.get(8),   "bit 8 should be set");
    assert!(bit_arr.get(15),  "bit 15 should be set");
    assert!(bit_arr.get(63),  "bit 63 should be set");
    assert!(bit_arr.get(64),  "bit 64 should be set");
    assert!(bit_arr.get(127), "bit 127 should be set");
    assert!(!bit_arr.get(1),  "bit 1 should be clear");
    assert!(!bit_arr.get(100),"bit 100 should be clear");

    // --- Both should agree on every bit ---
    for i in 0..128 {
        assert_eq!(
            byte_arr.get(i), bit_arr.get(i),
            "bit {i}: bytearray and bitarray disagree"
        );
    }

    // --- Round-trip: cast back to [u8; 16] ---
    let back: [u8; 16] = bytemuck::cast(byte_arr);
    assert_eq!(back, raw, "bytearray round-trip must be lossless");

    let back2: [u8; 16] = bytemuck::cast(bit_arr);
    assert_eq!(back2, raw, "bitarray round-trip must be lossless");
}
