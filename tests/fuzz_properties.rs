use bitstruct::{bitenum, bitstruct, bytestruct, byteval};
use proptest::prelude::*;

bitenum! {
    enum FuzzEnum(2) {
        A = 0,
        B = 1,
        C = 2,
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
        for i in 0..13 {
            recon |= (arr[i] as u128) << (i * 8);
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
        for i in 0..5 {
            prop_assert_eq!(arr[i], ((val >> (i * 8)) & 0xFF) as u8);
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_byteval_id56_fuzz(val in 0u64..(1u64 << 56)) {
        let id = FuzzId56::from_u64(val);
        prop_assert_eq!(id.value(), val);
        prop_assert_eq!(id.to_u64(), val);
        let arr = id.to_array();
        for i in 0..7 {
            prop_assert_eq!(arr[i], ((val >> (i * 8)) & 0xFF) as u8);
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
        for i in 0..13 {
            prop_assert_eq!(arr[i], ((masked >> (i * 8)) & 0xFF) as u8);
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
}
