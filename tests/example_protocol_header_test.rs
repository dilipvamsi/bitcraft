use bitstruct::{bitenum, bytestruct};

// 2-bit Packet Priority
bitenum! {
    pub enum Priority(2) {
        LOW = 0,
        MEDIUM = 1,
        HIGH = 2,
        URGENT = 3,
    }
}

bytestruct! {
    /// A custom 128-bit (16-byte) protocol header.
    /// Used for zero-copy parsing of network packets.
    pub struct CustomHeader(16) {
        pub version: u8 = 4,      // 4 bits
        pub priority: Priority = 2, // 2 bits
        pub is_encrypted: bool = 1, // 1 bit
        pub has_checksum: bool = 1, // 1 bit
        pub sequence_id: u32 = 24,  // 24-bit counter
        pub payload_len: u32 = 32,  // 32-bit length
        pub sender_id: u64 = 64,    // 64-bit globally unique ID
    }
}

#[test]
fn test_protocol_header_example() {
    // 1. Pack a header using the Builder pattern
    let header = CustomHeader::default()
        .with_version(1)
        .with_priority(Priority::HIGH)
        .with_is_encrypted(true)
        .with_sequence_id(12345)
        .with_payload_len(1024)
        .with_sender_id(0xDEADBEEF_CAFEBABE);

    // 2. Access fields with zero-cost getters
    assert_eq!(header.version(), 1);
    assert_eq!(header.priority(), Priority::HIGH);
    assert!(header.is_encrypted());
    assert_eq!(header.sequence_id(), 12345);
    assert_eq!(header.payload_len(), 1024);
    assert_eq!(header.sender_id(), 0xDEADBEEF_CAFEBABE);

    // 3. Zero-Copy: Get the raw bytes for transmission
    let raw_bytes: [u8; 16] = header.to_array();

    // 4. Zero-Copy: Parse from a raw buffer (e.g., from a socket)
    let parsed = CustomHeader::from_array(raw_bytes);
    assert_eq!(parsed, header);
}
