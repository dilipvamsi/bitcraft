#![cfg(feature = "alloc")]

use bitcraft::bytebox;

bytebox! {
    pub struct NibbleBox(u 4, 128);
}

bytebox! {
    pub struct FlagBox(bool, 50);
}

#[test]
fn test_bytebox_initialization() {
    let mut bx = NibbleBox::new();
    assert_eq!(bx.len(), 128);
    assert_eq!(bx.get(0), 0);
    assert_eq!(bx.get(127), 0);

    bx.set(0, 0xA);
    bx.set(127, 0xF);

    assert_eq!(bx.get(0), 0xA);
    assert_eq!(bx.get(127), 0xF);
}

#[test]
fn test_bytebox_as_bytes() {
    let mut bx = FlagBox::new();
    assert_eq!(bx.len(), 50);

    // 50 bits = 7 bytes exactly.
    assert_eq!(bx.as_bytes().len(), 7);

    bx.set(0, true);
    assert_eq!(bx.as_bytes()[0], 0x01);
}

#[test]
fn test_bytebox_slices() {
    let mut bx = NibbleBox::new();
    bx.set(10, 0x5);

    {
        let slice = bx.as_slice();
        assert_eq!(slice.len(), 128);
        assert_eq!(slice.get(10), 0x5);
    }

    {
        let mut mut_slice = bx.as_mut_slice();
        mut_slice.set(10, 0xA);
    }

    assert_eq!(bx.get(10), 0xA);
}

bytebox! {
    pub struct RuntimeNibbleBox(u 4);
}

#[test]
fn test_bytebox_runtime() {
    let mut bx = RuntimeNibbleBox::new(100);
    assert_eq!(bx.len(), 100);
    assert_eq!(bx.as_bytes().len(), 50); // 100 * 4 / 8 = 50

    bx.set(99, 0xF);
    assert_eq!(bx.get(99), 0xF);

    let slice = bx.as_slice();
    assert_eq!(slice.len(), 100);
    assert_eq!(slice.get(99), 0xF);
}

#[test]
fn test_bytebox_iterators() {
    let mut bx = NibbleBox::new();
    bx.set(0, 0x1);
    bx.set(1, 0x2);
    bx.set(2, 0xA);

    // Iterator over IntoIterator on &bx
    let items: Vec<u128> = bx.iter().take(3).collect();
    assert_eq!(items, vec![0x1, 0x2, 0xA]);

    // Explicit for loop
    let mut count = 0;
    for _ in &bx {
        count += 1;
    }
    assert_eq!(count, 128);
}
