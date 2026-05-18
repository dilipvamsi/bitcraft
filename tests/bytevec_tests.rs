#![cfg(feature = "alloc")]

use bitcraft::bytevec;

bytevec! {
    pub struct NibbleVec(u 4);
}

bytevec! {
    pub struct FlagVec(bool, 128); // Pre-allocates for 128 items
}

bytevec! {
    pub struct SignedVec(i 7);
}

#[test]
fn test_bytevec_push_pop() {
    let mut vec = NibbleVec::new();
    assert_eq!(vec.len(), 0);

    vec.push(0xA);
    vec.push(0x5);
    vec.push(0xF);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), 0xA);
    assert_eq!(vec.get(1), 0x5);
    assert_eq!(vec.get(2), 0xF);

    assert_eq!(vec.pop(), Some(0xF));
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.pop(), Some(0x5));
    assert_eq!(vec.pop(), Some(0xA));
    assert_eq!(vec.pop(), None);
}

#[test]
fn test_bytevec_capacity() {
    let mut vec = FlagVec::new();
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 128);

    for i in 0..150 {
        vec.push(i % 2 == 0);
    }

    assert_eq!(vec.len(), 150);
    assert_eq!(vec.get(0), true);
    assert_eq!(vec.get(1), false);
    assert_eq!(vec.get(149), false);
}

#[test]
fn test_bytevec_signed() {
    let mut vec = SignedVec::new();
    vec.push(-5);
    vec.push(63);
    vec.push(-64);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), -5);
    assert_eq!(vec.get(1), 63);
    assert_eq!(vec.get(2), -64);
}

#[test]
fn test_bytevec_as_bytes() {
    let mut vec = NibbleVec::new();
    vec.push(0x1);
    vec.push(0x2);
    // 0x1 is at lowest 4 bits, 0x2 is at next 4 bits -> byte 0 is 0x21
    assert_eq!(vec.as_bytes(), &[0x21]);

    vec.push(0xA);
    // 0xA is at lowest 4 bits of byte 1 -> byte 1 is 0x0A
    assert_eq!(vec.as_bytes(), &[0x21, 0x0A]);
}

#[test]
fn test_bytevec_slices() {
    let mut vec = NibbleVec::new();
    vec.push(0x1);
    vec.push(0x2);
    vec.push(0xA);

    {
        // Test read-only slice
        let slice = vec.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice.get(0), 0x1);
        assert_eq!(slice.get(1), 0x2);
        assert_eq!(slice.get(2), 0xA);
    }

    {
        // Test mutable slice
        let mut slice_mut = vec.as_mut_slice();
        slice_mut.set(1, 0x5);
        assert_eq!(slice_mut.get(1), 0x5);
    }

    assert_eq!(vec.get(1), 0x5);

    // Test creating view directly from existing byte buffer
    let raw_buffer: &[u8] = &[0x51, 0x0A]; // 0x1, 0x5, 0xA
    let view = NibbleVecSlice::new(raw_buffer, 3);
    assert_eq!(view.get(0), 0x1);
    assert_eq!(view.get(1), 0x5);
    assert_eq!(view.get(2), 0xA);
}

#[test]
fn test_bytevec_iterators() {
    let mut vec = NibbleVec::new();
    vec.push(0x1);
    vec.push(0x2);
    vec.push(0xA);

    // Test iterator on vec
    let items: Vec<u128> = vec.iter().collect();
    assert_eq!(items, vec![0x1, 0x2, 0xA]);

    // Test IntoIterator on &vec
    let mut sum = 0;
    for val in &vec {
        sum += val;
    }
    assert_eq!(sum, 0xD);

    // Test iterator on slice
    let slice = vec.as_slice();
    let slice_items: Vec<u128> = slice.into_iter().collect();
    assert_eq!(slice_items, vec![0x1, 0x2, 0xA]);
}
