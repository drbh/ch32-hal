//! Patches for some peripherals
//!
//! This should be removed in the future.
//!
//! - Patches for remap implementation
//! - Patches for peripherals without remap(fake remap)

#[cfg(peri_i2c2)]
mod i2c2 {
    impl crate::peripheral::sealed::RemapPeripheral for crate::peripherals::I2C2 {
        fn set_remap(_remap: u8) {}
    }
    impl crate::peripheral::RemapPeripheral for crate::peripherals::I2C2 {}
}

/// CH32V2, CH32V3
#[cfg(all(any(ch32v2, ch32v3), peri_usart1))]
mod usart1 {
    impl crate::peripheral::sealed::RemapPeripheral for crate::peripherals::USART1 {
        fn set_remap(remap: u8) {
            let afio = &crate::pac::AFIO;
            afio.pcfr1().modify(|w| w.set_usart1_rm(remap & 0b1 != 0));
            afio.pcfr2().modify(|w| w.set_usart1_rm2(remap & 0b10 != 0));
        }
    }
    impl crate::peripheral::RemapPeripheral for crate::peripherals::USART1 {}
}

#[cfg(all(peri_i2c1, ch32v0))]
mod i2c1 {
    impl crate::peripheral::sealed::RemapPeripheral for crate::peripherals::I2C1 {
        fn set_remap(remap: u8) {
            let afio = &crate::pac::AFIO;
            afio.pcfr1().modify(|w| w.set_i2c1_rm(remap & 0b1 != 0));
            afio.pcfr1().modify(|w| w.set_i2c1_rm1(remap & 0b10 != 0));
        }
    }
    impl crate::peripheral::RemapPeripheral for crate::peripherals::I2C1 {}
}

#[cfg(peri_spi2)]
mod spi2 {
    impl crate::peripheral::sealed::RemapPeripheral for crate::peripherals::SPI2 {
        fn set_remap(_remap: u8) {}
    }
    impl crate::peripheral::RemapPeripheral for crate::peripherals::SPI2 {}
}
