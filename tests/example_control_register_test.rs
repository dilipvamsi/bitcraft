use bitstruct::{bitenum, bitstruct};

// CPU Power States (2-bit)
bitenum! {
    pub enum PowerState(2) {
        SLEEP = 0,
        IDLE = 1,
        ACTIVE = 2,
        TURBO = 3,
    }
}

bitstruct! {
    /// A typical 32-bit CPU Control Register (CR0).
    /// Used for low-level system configuration.
    pub struct ControlRegister(u32) {
        pub enabled: bool = 1,            // bit 0: Global Enable
        pub interrupts: bool = 1,         // bit 1: Interrupt Enable
        pub dma_ready: bool = 1,          // bit 2: DMA Controller Ready
        pub power: PowerState = 2,        // bits 3-4: 2-bit Power Mode
        pub error_code: u8 = 4,           // bits 5-8: 4-bit Error Identification
        pub target_freq: u16 = 16,        // bits 9-24: 16-bit target frequency (MHz)
        pub reserved: u8 = 7,             // bits 25-31: Reserved for future use
    }
}

#[test]
fn test_control_register_example() {
    // 1. Initial configuration for cold boot
    let mut reg = ControlRegister::default()
        .with_enabled(true)
        .with_interrupts(false)
        .with_power(PowerState::SLEEP)
        .with_target_freq(1200);

    assert_eq!(reg.to_bits() & 0x1, 1); // enabled bit
    assert_eq!(reg.power(), PowerState::SLEEP);
    assert_eq!(reg.target_freq(), 1200);

    // 2. Perform a "Turbo Boost" transition
    reg.set_power(PowerState::TURBO);
    reg.set_target_freq(2400);
    reg.set_interrupts(true);

    // 3. Inspect final state
    assert_eq!(reg.power(), PowerState::TURBO);
    assert_eq!(reg.target_freq(), 2400);
    assert!(reg.interrupts());
    assert!(reg.enabled());

    // Verify all bits are where they belong
    // power: 3-4, interrupts: 1, enabled: 0
    // (Power: 3 << 3) = 0x18
    // (interrupts: 1 << 1) = 0x02
    // (enabled: 1 << 0) = 0x01
    // (freq: 2400 << 9) = 0x12C000
    assert_eq!(reg.to_bits() & 0x1F, 0x1B); // power(3) | enable(1) | int(1)
}
