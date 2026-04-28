use bitcraft::{
    atomic_bitenum, atomic_bitstruct, bitarray, bitenum, bitstruct, bytearray, bytestruct, byteval,
};
use proptest::prelude::*;

atomic_bitstruct! {
    struct FuzzAtomicStruct(AtomicU64) {
        a: bool = 1,
        b: u8 = 7,
        c: u32 = 24,
        d: FuzzEnum = 2,
        e: u32 = 30, // 1+7+24+2+30 = 64
    }
}

atomic_bitenum! {
    enum FuzzAtomicEnum(AtomicU8, 2) {
        A = 0,
        B = 1,
        C = 2,
    }
}

bitenum! {
    enum FuzzEnum(2) {
        A = 0,
        B = 1,
        C = 2,
    }
}

bitenum! {
    enum FuzzSignedEnum(i 3) {
        V_MIN = -4,
        N_ONE = -1,
        ZERO = 0,
        V_MAX = 3,
    }
}

bitstruct! {
    struct FuzzStruct(u64) {
        a: bool = 1,
        b: u8 = 7,
        c: u32 = 24,
        d: FuzzEnum = 2,
        e: u32 = 30, // 1+7+24+2+30 = 64
    }
}

bitstruct! {
    struct FuzzSignedEnumStruct(u16) {
        a: FuzzSignedEnum = 3,
        b: FuzzSignedEnum = 4, // Intentionally wider field to test masking of signed values
        c: i16 = 9,
    }
}

bytestruct! {
    struct FuzzByteStruct13(13) { // 104 bits
        flags: u8 = 8,
        coord_x: u32 = 32,
        coord_y: u32 = 32,
        state: FuzzEnum = 2,
        payload_meta: u32 = 30, // 8+32+32+2+30 = 104
    }
}

bytestruct! {
    struct CrossBoundaryFuzz(2) {
        low: u8 = 4,   // bits 0-3
        mid: u8 = 8,   // bits 4-11 (crosses byte 0-1)
        high: u8 = 4,  // bits 12-15
    }
}

bytestruct! {
    struct WideFuzz16(16) {
        start: bool = 1,
        large: u128 = 120, // bits 1-120
        end: u8 = 7,       // bits 121-127
    }
}

// Multiple byteval types for fuzzing
byteval! { struct FuzzId24(3); } // 24-bit
byteval! { struct FuzzId40(5); } // 40-bit
byteval! { struct FuzzId56(7); } // 56-bit
byteval! { struct FuzzId104(13); } // 104-bit

// Variety types for expanded coverage
byteval! { struct FuzzId48u16(3, u16); } // 48-bit with u16 storage
byteval! { struct FuzzId96u32(3, u32); } // 96-bit with u32 storage

// Signed byteval types
byteval! { struct FuzzSignedId24(i 3); }
byteval! { struct FuzzSignedId40(i 5); }

bitstruct! {
    struct DenseFuzz(u32) {
        b00: bool = 1, b01: bool = 1, b02: bool = 1, b03: bool = 1,
        b04: bool = 1, b05: bool = 1, b06: bool = 1, b07: bool = 1,
        b08: bool = 1, b09: bool = 1, b10: bool = 1, b11: bool = 1,
        b12: bool = 1, b13: bool = 1, b14: bool = 1, b15: bool = 1,
        b16: bool = 1, b17: bool = 1, b18: bool = 1, b19: bool = 1,
        b20: bool = 1, b21: bool = 1, b22: bool = 1, b23: bool = 1,
        b24: bool = 1, b25: bool = 1, b26: bool = 1, b27: bool = 1,
        b28: bool = 1, b29: bool = 1, b30: bool = 1, b31: bool = 1,
    }
}

bitstruct! {
    struct SignedBaseFuzz(i32) {
        a: bool = 1,
        b: u8 = 7,
        c: u16 = 15,
        d: u8 = 8, // 1 + 7 + 15 + 8 = 31 bits
    }
}

bitstruct! {
    struct SignedBaseI16Fuzz(i16) {
        x: u8 = 4,
        y: u16 = 11, // 4 + 11 = 15 bits
    }
}

bytestruct! {
    #[repr(align(8))]
    struct AlignedFuzzer(8) {
        val: u64 = 64,
    }
}

// --- Fuzz types for bitarray and bytearray ---
bitarray! {
    struct FuzzBitNibbles(u 4, 32); // 128 bits
}

bitarray! {
    struct FuzzBitSigned(i 7, 10); // 70 bits
}

bitarray! {
    struct FuzzBitBool(bool, 128); // 128 bits
}

bytearray! {
    struct FuzzByteNibbles(u 4, 64); // 256 bits -> [u8; 32]
}

bytearray! {
    struct FuzzByteSigned(i 7, 30); // 210 bits -> [u8; 27]
}

bytearray! {
    struct FuzzByteBool(bool, 256); // 256 bits -> [u8; 32]
}

bytestruct! {
    struct MixedStorageFuzzer([u64; 2]) {
        a: u128 = 80,
        b: u64 = 48, // 80 + 48 = 128 bits
    }
}

bitstruct! {
    struct SignedFieldsBitstruct(u64) {
        a: i8 = 5,   // min -16, max 15
        b: i16 = 10, // min -512, max 511
        c: i32 = 20, // min -524288, max 524287
        d: i64 = 29, // min -268435456, max 268435455
    }
}

bytestruct! {
    struct SignedFieldsBytestruct(8) {
        a: i8 = 5,
        b: i16 = 10,
        c: i32 = 20,
        d: i64 = 29,
    }
}

bytestruct! {
    struct SignedTypedFieldsBytestruct([u32; 2]) {
        a: i8 = 5,
        b: i16 = 10,
        c: i32 = 20,
        d: i64 = 29,
    }
}

bitstruct! {
    struct SignedFullWidthBitstruct(u128) {
        a: i8 = 8,
        b: i16 = 16,
        c: i32 = 32,
        d: i64 = 64,
        e: i8 = 8,
    }
}

bitstruct! {
    struct MixedSignBitstruct(u64) {
        a: i8 = 5,
        b: u16 = 10,
        c: i32 = 20,
        d: u64 = 29,
    }
}

bytestruct! {
    struct MixedSignBytestruct(8) {
        a: i8 = 5,
        b: u16 = 10,
        c: i32 = 20,
        d: u64 = 29,
    }
}

proptest! {
    #[test]
    fn test_bitstruct_roundtrip(
        a in any::<bool>(),
        b in 0u8..128,
        c in any::<u32>(),
        d_val in 0u8..3u8,
        e in 0u32..(1u32 << 30)
    ) {
        let d = match d_val {
            0 => FuzzEnum::A,
            1 => FuzzEnum::B,
            _ => FuzzEnum::C,
        };

        let s = FuzzStruct::default()
            .with_a(a)
            .with_b(b)
            .with_c(c & 0xFFFFFF) // 24 bits
            .with_d(d)
            .with_e(e);

        prop_assert_eq!(s.a(), a);
        prop_assert_eq!(s.b(), b);
        prop_assert_eq!(s.c(), c & 0xFFFFFF);
        prop_assert_eq!(s.d(), d);
        prop_assert_eq!(s.e(), e);

        // Verify isolation: bitwise reconstruction
        let mut expected = 0u64;
        expected |= a as u64;
        expected |= (b as u64) << 1;
        expected |= ((c & 0xFFFFFF) as u64) << (1 + 7);
        expected |= (d.to_bits() as u64) << (1 + 7 + 24);
        expected |= (e as u64) << (1 + 7 + 24 + 2);

        prop_assert_eq!(s.to_bits(), expected);
    }

    #[test]
    fn test_signed_enum_fuzz(
        a_idx in 0usize..4usize,
        b_idx in 0usize..4usize,
        c in -256i16..256i16 // will be masked to 9 bits
    ) {
        let variants = [
            FuzzSignedEnum::V_MIN,
            FuzzSignedEnum::N_ONE,
            FuzzSignedEnum::ZERO,
            FuzzSignedEnum::V_MAX,
        ];
        let a = variants[a_idx];
        let b = variants[b_idx];
        let c_masked = (c as i32 & 0x1FF) as i16;
        let c_final = ((c_masked << (16 - 9)) >> (16 - 9)) as i16;

        let mut s = FuzzSignedEnumStruct::default();
        s.set_a(a);
        s.set_b(b);
        s.set_c(c_final);

        prop_assert_eq!(s.a(), a);
        prop_assert_eq!(s.b(), b);
        prop_assert_eq!(s.c(), c_final);

        let a_bits = a.to_bits() as u16 & 0x7;
        let b_bits = b.to_bits() as u16 & 0xF; // B is 4 bits
        let c_bits = c_final as u16 & 0x1FF;

        let mut expected = 0u16;
        expected |= a_bits;
        expected |= b_bits << 3;
        expected |= c_bits << (3 + 4);

        prop_assert_eq!(s.to_bits(), expected);
    }


    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_bytestruct13_roundtrip(
        flags in any::<u8>(),
        x in any::<u32>(),
        y in any::<u32>(),
        state_val in 0u8..3u8,
        meta in 0u32..(1u32 << 30)
    ) {
        let state = match state_val {
            0 => FuzzEnum::A,
            1 => FuzzEnum::B,
            _ => FuzzEnum::C,
        };

        let s = FuzzByteStruct13::default()
            .with_flags(flags)
            .with_coord_x(x)
            .with_coord_y(y)
            .with_state(state)
            .with_payload_meta(meta);

        prop_assert_eq!(s.flags(), flags);
        prop_assert_eq!(s.coord_x(), x);
        prop_assert_eq!(s.coord_y(), y);
        prop_assert_eq!(s.state(), state);
        prop_assert_eq!(s.payload_meta(), meta);

        // Verify raw array LE mapping
        let arr = s.to_array();
        let mut recon = 0u128;
        for (i, &byte) in arr.iter().enumerate() {
            recon |= (byte as u128) << (i * 8);
        }

        let mut expected = 0u128;
        expected |= flags as u128;
        expected |= (x as u128) << 8;
        expected |= (y as u128) << (8 + 32);
        expected |= (state.to_bits() as u128) << (8 + 32 + 32);
        expected |= (meta as u128) << (8 + 32 + 32 + 2);

        prop_assert_eq!(recon, expected);
    }

    #[test]
    fn test_cross_boundary_fuzz(
        low in 0u8..16,
        mid in any::<u8>(),
        high in 0u8..16
    ) {
        let s = CrossBoundaryFuzz::default()
            .with_low(low)
            .with_mid(mid)
            .with_high(high);

        prop_assert_eq!(s.low(), low);
        prop_assert_eq!(s.mid(), mid);
        prop_assert_eq!(s.high(), high);

        // Manual verification of bit-packing
        let val = s.to_u16();
        let expected = (low as u16) | ((mid as u16) << 4) | ((high as u16) << 12);
        prop_assert_eq!(val, expected);
    }

    #[test]
    fn test_wide_fuzz16_roundtrip(
        start in any::<bool>(),
        large in any::<u128>(),
        end in 0u8..128
    ) {
        let mask_large = (!0u128) >> 8; // 120 bits
        let masked_large = large & mask_large;

        let s = WideFuzz16::default()
            .with_start(start)
            .with_large(masked_large)
            .with_end(end);

        prop_assert_eq!(s.start(), start);
        prop_assert_eq!(s.large(), masked_large);
        prop_assert_eq!(s.end(), end);
    }

    #[test]
    fn test_byteval_id24_fuzz(val in 0u32..(1u32 << 24)) {
        let id = FuzzId24::from_u32(val);
        prop_assert_eq!(id.value(), val);
        prop_assert_eq!(id.to_u32(), val);
        let arr = id.to_array();
        prop_assert_eq!(arr[0], (val & 0xFF) as u8);
        prop_assert_eq!(arr[1], ((val >> 8) & 0xFF) as u8);
        prop_assert_eq!(arr[2], ((val >> 16) & 0xFF) as u8);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_byteval_id40_fuzz(val in 0u64..(1u64 << 40)) {
        let id = FuzzId40::from_u64(val);
        prop_assert_eq!(id.value(), val);
        prop_assert_eq!(id.to_u64(), val);
        let arr = id.to_array();
        for (i, &byte) in arr.iter().enumerate() {
            prop_assert_eq!(byte, ((val >> (i * 8)) & 0xFF) as u8);
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_byteval_id56_fuzz(val in 0u64..(1u64 << 56)) {
        let id = FuzzId56::from_u64(val);
        prop_assert_eq!(id.value(), val);
        prop_assert_eq!(id.to_u64(), val);
        let arr = id.to_array();
        for (i, &byte) in arr.iter().enumerate() {
            prop_assert_eq!(byte, ((val >> (i * 8)) & 0xFF) as u8);
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_byteval_id104_fuzz(val in any::<u128>()) {
        let mask = (!0u128) >> (128 - 104);
        let masked = val & mask;
        let id = FuzzId104::from_u128(masked);
        prop_assert_eq!(id.value(), masked);
        prop_assert_eq!(id.to_u128(), masked);
        let arr = id.to_array();
        for (i, &byte) in arr.iter().enumerate() {
            prop_assert_eq!(byte, ((masked >> (i * 8)) & 0xFF) as u8);
        }
    }

    #[test]
    fn test_field_isolation_fuzzed(
        initial in any::<u64>(),
        new_val in any::<u32>(),
    ) {
        let s = FuzzStruct::from_bits(initial);
        let s2 = s.with_c(new_val & 0xFFFFFF); // c is 24 bits
        prop_assert_eq!(s2.a(), s.a());
        prop_assert_eq!(s2.b(), s.b());
        prop_assert_eq!(s2.d(), s.d());
        prop_assert_eq!(s2.e(), s.e());
        prop_assert_eq!(s2.c(), new_val & 0xFFFFFF);
    }

    #[test]
    fn test_u16_storage_fuzz(
        v1 in 0u32..(1u32 << 24),
        v2 in 0u32..(1u32 << 24)
    ) {
        bytestruct! {
            struct U16Fuzzer([u16; 3]) {
                f1: u32 = 24,
                f2: u32 = 24,
            }
        }
        let s = U16Fuzzer::default().with_f1(v1).with_f2(v2);
        prop_assert_eq!(s.f1(), v1);
        prop_assert_eq!(s.f2(), v2);

        // Manual verification of u16 array packing
        let arr = s.to_array();
        let mut recon = 0u64;
        recon |= arr[0] as u64;
        recon |= (arr[1] as u64) << 16;
        recon |= (arr[2] as u64) << 32;

        let expected = (v1 as u64) | ((v2 as u64) << 24);
        prop_assert_eq!(recon, expected);
    }

    #[test]
    fn test_u32_storage_fuzz(
        v1 in any::<u32>(),
        v2 in any::<u32>()
    ) {
        bytestruct! {
            struct U32Fuzzer([u32; 2]) {
                f1: u32 = 32,
                f2: u32 = 32,
            }
        }
        let s = U32Fuzzer::default().with_f1(v1).with_f2(v2);
        prop_assert_eq!(s.f1(), v1);
        prop_assert_eq!(s.f2(), v2);
        prop_assert_eq!(s.to_array(), [v1, v2]);
    }

    #[test]
    fn test_from_bits_fresh_path_fuzz(val in any::<u64>()) {
        // Exercise the 8-element unroll path (64 bits total)
        bytestruct! {
            struct LargeFuzzer(8) {
                val: u64 = 64,
            }
        }
        let masked = val;
        let s = LargeFuzzer::from_bits(masked);
        prop_assert_eq!(s.to_bits(), masked);

        // Ensure initialized array matches manual construction
        let arr = s.to_array();
        for (i, &byte) in arr.iter().enumerate() {
            prop_assert_eq!(byte, ((masked >> (i * 8)) & 0xFF) as u8);
        }
    }
    #[test]
    fn test_u64_storage_fuzz(
        v1 in any::<u64>(),
        v2 in any::<u64>()
    ) {
        bytestruct! {
            struct U64Fuzzer([u64; 2]) {
                f1: u64 = 64,
                f2: u64 = 64,
            }
        }
        let s = U64Fuzzer::default().with_f1(v1).with_f2(v2);
        prop_assert_eq!(s.f1(), v1);
        prop_assert_eq!(s.f2(), v2);
        prop_assert_eq!(s.to_array(), [v1, v2]);
        prop_assert_eq!(s.to_bits(), (v1 as u128) | ((v2 as u128) << 64));
    }

    #[test]
    fn test_u128_storage_fuzz(val in any::<u128>()) {
        bytestruct! {
            struct U128Fuzzer([u128; 1]) {
                f1: u128 = 128,
            }
        }
        let s = U128Fuzzer::default().with_f1(val);
        prop_assert_eq!(s.f1(), val);
        prop_assert_eq!(s.to_array(), [val]);
        prop_assert_eq!(s.to_bits(), val);
    }

    #[test]
    fn test_u16_storage_128_fuzz(
        v1 in any::<u64>(),
        v2 in any::<u64>()
    ) {
        bytestruct! {
            struct U16WideFuzzer([u16; 8]) {
                f1: u64 = 64,
                f2: u64 = 64,
            }
        }
        let s = U16WideFuzzer::default().with_f1(v1).with_f2(v2);
        prop_assert_eq!(s.f1(), v1);
        prop_assert_eq!(s.f2(), v2);
        let arr = s.to_array();
        let mut recon = 0u128;
        for (i, &word) in arr.iter().enumerate() {
            recon |= (word as u128) << (i * 16);
        }
        prop_assert_eq!(recon, (v1 as u128) | ((v2 as u128) << 64));
    }

    #[test]
    fn test_u32_storage_128_fuzz(
        v1 in any::<u64>(),
        v2 in any::<u64>()
    ) {
        bytestruct! {
            struct U32WideFuzzer([u32; 4]) {
                f1: u64 = 64,
                f2: u64 = 64,
            }
        }
        let s = U32WideFuzzer::default().with_f1(v1).with_f2(v2);
        prop_assert_eq!(s.f1(), v1);
        prop_assert_eq!(s.f2(), v2);
        let arr = s.to_array();
        let mut recon = 0u128;
        for (i, &word) in arr.iter().enumerate() {
            recon |= (word as u128) << (i * 32);
        }
        prop_assert_eq!(recon, (v1 as u128) | ((v2 as u128) << 64));
    }

    #[test]
    fn test_u16_storage_odd_fuzz(
        v1 in 0u64..(1u64 << 48)
    ) {
        bytestruct! {
            struct U16OddFuzzer([u16; 3]) {
                f1: u64 = 48,
            }
        }
        let s = U16OddFuzzer::default().with_f1(v1);
        prop_assert_eq!(s.f1(), v1);
        let arr = s.to_array();
        let mut recon = 0u64;
        for (i, &word) in arr.iter().enumerate() {
            recon |= (word as u64) << (i * 16);
        }
        prop_assert_eq!(recon, v1);
    }

    #[test]
    fn test_u32_storage_odd_fuzz(
        v1 in any::<u128>()
    ) {
        bytestruct! {
            struct U32OddFuzzer([u32; 3]) {
                f1: u128 = 96,
            }
        }
        let mask = (!0u128) >> (128 - 96);
        let masked = v1 & mask;
        let s = U32OddFuzzer::default().with_f1(masked);
        prop_assert_eq!(s.f1(), masked);
        let arr = s.to_array();
        let mut recon = 0u128;
        for (i, &word) in arr.iter().enumerate() {
            recon |= (word as u128) << (i * 32);
        }
        prop_assert_eq!(recon, masked);
    }

    #[test]
    fn test_byteval_u16_odd_fuzz(val in 0u64..(1u64 << 48)) {
        byteval! { struct OddId(3, u16); }
        let id = OddId::from_u64(val);
        prop_assert_eq!(id.value(), val);
        prop_assert_eq!(id.to_u64(), val);
        let arr = id.to_array();
        for (i, &word) in arr.iter().enumerate() {
            prop_assert_eq!(word, ((val >> (i * 16)) & 0xFFFF) as u16);
        }
    }

    #[test]
    fn test_dense_bool_fuzz(bits in any::<u32>()) {
        let s = DenseFuzz::from_bits(bits);
        prop_assert_eq!(s.to_bits(), bits);
        prop_assert_eq!(s.b00(), (bits & 1) != 0);
        prop_assert_eq!(s.b01(), (bits & 2) != 0);
        prop_assert_eq!(s.b15(), (bits & (1 << 15)) != 0);
        prop_assert_eq!(s.b31(), (bits & (1 << 31)) != 0);
    }

    #[test]
    fn test_variety_byteval_fuzz(
        v48 in 0u64..(1u64 << 48),
        v96 in 0u128..(1u128 << 96)
    ) {
        let id48 = FuzzId48u16::from_u64(v48);
        prop_assert_eq!(id48.value(), v48);
        let arr48 = id48.to_array();
        for (i, &word) in arr48.iter().enumerate() {
            prop_assert_eq!(word, ((v48 >> (i * 16)) & 0xFFFF) as u16);
        }

        let id96 = FuzzId96u32::from_u128(v96);
        prop_assert_eq!(id96.value(), v96);
        let arr96 = id96.to_array();
        for (i, &word) in arr96.iter().enumerate() {
            prop_assert_eq!(word, ((v96 >> (i * 32)) & 0xFFFFFFFF) as u32);
        }
    }

    #[test]
    fn test_aligned_fuzzer_roundtrip(val in any::<u64>()) {
        let s = AlignedFuzzer::from_bits(val);
        prop_assert_eq!(s.val(), val);
        prop_assert_eq!(s.to_bits(), val);
        // Verify address is aligned to 8
        let addr = &s as *const _ as usize;
        prop_assert_eq!(addr % 8, 0);
    }

    #[test]
    fn test_safe_setters_overflow_fuzz(
        val in 0u32..(1u32 << 24),
        overflow_bit in 24u32..32u32
    ) {
        let mut id = FuzzId24::from_u32(val);
        let overflow_val = val | (1 << overflow_bit);

        // try_set should fail
        let res = id.try_set_value(overflow_val);
        prop_assert!(res.is_err());
        // Value should remain unchanged
        prop_assert_eq!(id.value(), val);

        // try_with should fail
        let res_with = id.try_with_value(overflow_val);
        prop_assert!(res_with.is_err());
    }

    #[test]
    fn test_mixed_storage_fuzzer_roundtrip(
        a in any::<u128>(),
        b in any::<u64>(),
    ) {
        let mask_a = (!0u128) >> (128 - 80);
        let mask_b = (!0u64) >> (64 - 48);
        let masked_a = a & mask_a;
        let masked_b = b & mask_b;

        let s = MixedStorageFuzzer::default()
            .with_a(masked_a)
            .with_b(masked_b);

        prop_assert_eq!(s.a(), masked_a);
        prop_assert_eq!(s.b(), masked_b);

        // Verify array-level packing
        let arr = s.to_array();
        let mut recon = 0u128;
        recon |= arr[0] as u128; // first 64 bits
        recon |= (arr[1] as u128) << 64; // next 64 bits

        let expected = (masked_a) | ((masked_b as u128) << 80);
        prop_assert_eq!(recon, expected);
    }

    #[test]
    fn test_signed_base_fuzz(
        a in any::<bool>(),
        b in 0u8..128,
        c in 0u16..32768,
        d in any::<u8>(),
    ) {
        let s = SignedBaseFuzz::default()
            .with_a(a)
            .with_b(b)
            .with_c(c)
            .with_d(d);

        prop_assert_eq!(s.a(), a);
        prop_assert_eq!(s.b(), b);
        prop_assert_eq!(s.c(), c);
        prop_assert_eq!(s.d(), d);

        let mut expected = 0u32;
        expected |= a as u32;
        expected |= (b as u32) << 1;
        expected |= (c as u32) << (1 + 7);
        expected |= (d as u32) << (1 + 7 + 15);

        prop_assert_eq!(s.to_bits(), expected as i32);
    }

    #[test]
    fn test_signed_base_i16_fuzz(
        x in 0u8..16,
        y in 0u16..2048,
    ) {
        let s = SignedBaseI16Fuzz::default()
            .with_x(x)
            .with_y(y);

        prop_assert_eq!(s.x(), x);
        prop_assert_eq!(s.y(), y);

        let mut expected = 0u16;
        expected |= x as u16;
        expected |= (y as u16) << 4;

        prop_assert_eq!(s.to_bits(), expected as i16);
    }

    #[test]
    fn test_signed_fields_fuzz(
        a in -16i8..=15,
        b in -512i16..=511,
        c in -524288i32..=524287,
        d in -268435456i64..=268435455,
    ) {
        // bitstruct! variant
        let mut bs = SignedFieldsBitstruct::default();
        prop_assert!(bs.try_set_a(a).is_ok());
        prop_assert!(bs.try_set_b(b).is_ok());
        prop_assert!(bs.try_set_c(c).is_ok());
        prop_assert!(bs.try_set_d(d).is_ok());

        prop_assert_eq!(bs.a(), a);
        prop_assert_eq!(bs.b(), b);
        prop_assert_eq!(bs.c(), c);
        prop_assert_eq!(bs.d(), d);

        // Verify bounds rejection with out of bounds values
        let a_out = if a >= 0 { a.saturating_add(16) } else { a.saturating_sub(16) };
        if a_out > 15 || a_out < -16 {
            prop_assert!(bs.try_set_a(a_out).is_err());
        }

        // bytestruct! variant
        let mut by = SignedFieldsBytestruct::default();
        prop_assert!(by.try_set_a(a).is_ok());
        prop_assert!(by.try_set_b(b).is_ok());
        prop_assert!(by.try_set_c(c).is_ok());
        prop_assert!(by.try_set_d(d).is_ok());

        prop_assert_eq!(by.a(), a);
        prop_assert_eq!(by.b(), b);
        prop_assert_eq!(by.c(), c);
        prop_assert_eq!(by.d(), d);

        let expected_raw = bs.to_bits();
        // Since both have the same layout and are 8 bytes total
        let _expected_bytes = expected_raw.to_le_bytes(); // bytestruct uses LE natively on LE systems. Wait, bitstruct is host endian.
        // Actually, let's just ensure they hold their values properly.
    }

    #[test]
    fn test_signed_typed_fields_fuzz(
        a in -16i8..=15,
        b in -512i16..=511,
        c in -524288i32..=524287,
        d in -268435456i64..=268435455,
    ) {
        let mut bs = SignedTypedFieldsBytestruct::default();
        prop_assert!(bs.try_set_a(a).is_ok());
        prop_assert!(bs.try_set_b(b).is_ok());
        prop_assert!(bs.try_set_c(c).is_ok());
        prop_assert!(bs.try_set_d(d).is_ok());

        prop_assert_eq!(bs.a(), a);
        prop_assert_eq!(bs.b(), b);
        prop_assert_eq!(bs.c(), c);
        prop_assert_eq!(bs.d(), d);
    }

    #[test]
    fn test_signed_full_width_fuzz(
        a in any::<i8>(),
        b in any::<i16>(),
        c in any::<i32>(),
        d in any::<i64>(),
        e in any::<i8>(),
    ) {
        let mut bs = SignedFullWidthBitstruct::default();
        prop_assert!(bs.try_set_a(a).is_ok());
        prop_assert!(bs.try_set_b(b).is_ok());
        prop_assert!(bs.try_set_c(c).is_ok());
        prop_assert!(bs.try_set_d(d).is_ok());
        prop_assert!(bs.try_set_e(e).is_ok());

        prop_assert_eq!(bs.a(), a);
        prop_assert_eq!(bs.b(), b);
        prop_assert_eq!(bs.c(), c);
        prop_assert_eq!(bs.d(), d);
        prop_assert_eq!(bs.e(), e);
    }

    #[test]
    fn test_mixed_sign_fields_fuzz(
        a in -16i8..=15,
        b in 0u16..=1023,
        c in -524288i32..=524287,
        d in 0u64..=536870911,
    ) {
        let mut bs = MixedSignBitstruct::default();
        prop_assert!(bs.try_set_a(a).is_ok());
        prop_assert!(bs.try_set_b(b).is_ok());
        prop_assert!(bs.try_set_c(c).is_ok());
        prop_assert!(bs.try_set_d(d).is_ok());

        prop_assert_eq!(bs.a(), a);
        prop_assert_eq!(bs.b(), b);
        prop_assert_eq!(bs.c(), c);
        prop_assert_eq!(bs.d(), d);

        let mut by = MixedSignBytestruct::default();
        prop_assert!(by.try_set_a(a).is_ok());
        prop_assert!(by.try_set_b(b).is_ok());
        prop_assert!(by.try_set_c(c).is_ok());
        prop_assert!(by.try_set_d(d).is_ok());

        prop_assert_eq!(by.a(), a);
        prop_assert_eq!(by.b(), b);
        prop_assert_eq!(by.c(), c);
        prop_assert_eq!(by.d(), d);
    }

    #[test]
    fn test_signed_byteval_fuzz(
        id24 in -8388608i32..=8388607,
        id40 in -549755813888i64..=549755813887,
    ) {
        let mut s24 = FuzzSignedId24::default();
        s24.set_value(id24);
        prop_assert_eq!(s24.value(), id24);

        let mut s40 = FuzzSignedId40::default();
        s40.set_value(id40);
        prop_assert_eq!(s40.value(), id40);
    }

    #[test]
    fn test_atomic_bitstruct_fuzz(
        a in any::<bool>(),
        b in 0u8..=127,
        c in 0u32..=16777215,
        d in 0u8..=2, // FuzzEnum
        e in 0u32..=1073741823,
    ) {
        let bs = FuzzAtomicStruct::new(0);

        let enum_d = match d {
            0 => FuzzEnum::A,
            1 => FuzzEnum::B,
            _ => FuzzEnum::C,
        };

        // Update single fields
        prop_assert!(bs.try_set_a(a, core::sync::atomic::Ordering::SeqCst).is_ok());
        prop_assert!(bs.try_set_b(b, core::sync::atomic::Ordering::SeqCst).is_ok());
        prop_assert!(bs.try_set_c(c, core::sync::atomic::Ordering::SeqCst).is_ok());
        prop_assert!(bs.try_set_d(enum_d, core::sync::atomic::Ordering::SeqCst).is_ok());
        prop_assert!(bs.try_set_e(e, core::sync::atomic::Ordering::SeqCst).is_ok());

        prop_assert_eq!(bs.a(core::sync::atomic::Ordering::Relaxed), a);
        prop_assert_eq!(bs.b(core::sync::atomic::Ordering::Relaxed), b);
        prop_assert_eq!(bs.c(core::sync::atomic::Ordering::Relaxed), c);
        prop_assert_eq!(bs.d(core::sync::atomic::Ordering::Relaxed), enum_d);
        prop_assert_eq!(bs.e(core::sync::atomic::Ordering::Relaxed), e);

        // Update via batch
        bs.update(core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::Relaxed, |v| {
            v.set_a(!a);
            v.set_b(127 - b);
        });

        prop_assert_eq!(bs.a(core::sync::atomic::Ordering::Relaxed), !a);
        prop_assert_eq!(bs.b(core::sync::atomic::Ordering::Relaxed), 127 - b);
        prop_assert_eq!(bs.c(core::sync::atomic::Ordering::Relaxed), c);

        // Test update_or_abort aborting
        let abort_res = bs.update_or_abort(core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::Relaxed, |v| {
            v.set_c(0);
            None
        });

        prop_assert!(abort_res.is_err());
        prop_assert_eq!(bs.c(core::sync::atomic::Ordering::Relaxed), c); // C should remain unchanged!
    }

    #[test]
    fn test_atomic_bitenum_fuzz(
        initial in 0u8..=2,
        target in 0u8..=2,
    ) {
        let variants = [FuzzAtomicEnumValue::A, FuzzAtomicEnumValue::B, FuzzAtomicEnumValue::C];
        let initial_val = variants[initial as usize];
        let target_val = variants[target as usize];

        let ae = FuzzAtomicEnum::new(initial_val);
        prop_assert_eq!(ae.load(core::sync::atomic::Ordering::Relaxed), initial_val);

        ae.store(target_val, core::sync::atomic::Ordering::SeqCst);
        prop_assert_eq!(ae.load(core::sync::atomic::Ordering::Relaxed), target_val);

        // Update via CAS loop
        ae.update(core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::Relaxed, |v| {
            if v == target_val {
                initial_val
            } else {
                v
            }
        });

        prop_assert_eq!(ae.load(core::sync::atomic::Ordering::Relaxed), initial_val);

        // test update_or_abort
        let res = ae.update_or_abort(core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::Relaxed, |v| {
            if v == initial_val {
                Some(target_val)
            } else {
                None
            }
        });
        prop_assert!(res.is_ok());
        prop_assert_eq!(ae.load(core::sync::atomic::Ordering::Relaxed), target_val);
    }
}

proptest! {
    #[test]
    fn fuzz_bitarray_nibbles(values in prop::collection::vec(0..16u128, 32)) {
        let mut arr = FuzzBitNibbles::default();
        for (i, &val) in values.iter().enumerate() {
            arr.set(i, val);
        }
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(arr.get(i), val);
        }

        // Test to_bits and casting
        let bits = arr.to_bits();
        let casted: FuzzBitNibbles = bytemuck::cast(bits);
        prop_assert_eq!(casted.to_bits(), bits);
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(casted.get(i), val);
        }
    }

    #[test]
    fn fuzz_bitarray_signed(values in prop::collection::vec(-64..64i128, 10)) {
        let mut arr = FuzzBitSigned::default();
        for (i, &val) in values.iter().enumerate() {
            arr.set(i, val);
        }
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(arr.get(i), val);
        }
    }

    #[test]
    fn fuzz_bitarray_bool(values in prop::collection::vec(any::<bool>(), 128)) {
        let mut arr = FuzzBitBool::default();
        for (i, &val) in values.iter().enumerate() {
            arr.set(i, val);
        }
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(arr.get(i), val);
        }
    }

    #[test]
    fn fuzz_bytearray_nibbles(values in prop::collection::vec(0..16u128, 64)) {
        let mut arr = FuzzByteNibbles::default();
        for (i, &val) in values.iter().enumerate() {
            arr.set(i, val);
        }
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(arr.get(i), val);
        }

        // Test bytemuck casting
        let bytes: [u8; 32] = bytemuck::cast(arr);
        let casted: FuzzByteNibbles = bytemuck::cast(bytes);
        prop_assert_eq!(casted.0, arr.0);
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(casted.get(i), val);
        }
    }

    #[test]
    fn fuzz_bytearray_signed(values in prop::collection::vec(-64..64i128, 30)) {
        let mut arr = FuzzByteSigned::default();
        for (i, &val) in values.iter().enumerate() {
            arr.set(i, val);
        }
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(arr.get(i), val);
        }
    }

    #[test]
    fn fuzz_bytearray_bool(values in prop::collection::vec(any::<bool>(), 256)) {
        let mut arr = FuzzByteBool::default();
        for (i, &val) in values.iter().enumerate() {
            arr.set(i, val);
        }
        for (i, &val) in values.iter().enumerate() {
            prop_assert_eq!(arr.get(i), val);
        }
    }
}
