//! LTC2668-class 16-bit SPI DAC driver.
//!
//! Mirrors firmware/sim/dac_chain.hexa: 16-bit, ±10 V, 1 MS/s, 4
//! channels (HV / coil / pump / valve). Real LTC2668 has more bits but
//! we model the production spec at the 16-bit ENOB sweet spot.
//!
//! Wire protocol (LTC2668 datasheet):
//!   24-bit transaction: [4-bit cmd | 4-bit addr | 16-bit data]
//!   commands of interest:
//!     CMD_WRITE_DAC_AND_UPDATE = 0x3
//!     CMD_WRITE_DAC_NO_UPDATE  = 0x0
//!     CMD_UPDATE_DAC           = 0x1
//!     CMD_POWER_DOWN_CHANNEL   = 0x4
//!     CMD_POWER_DOWN_CHIP      = 0x5
//!
//! Channels 0..3 = HV / coil / pump / valve.

use core::convert::Infallible;

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;

/// LTC2668 24-bit command words.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Cmd {
    WriteDacNoUpdate    = 0x0,
    UpdateDac           = 0x1,
    WriteDacAndUpdate   = 0x3,
    PowerDownChannel    = 0x4,
    PowerDownChip       = 0x5,
}

/// Channel identifiers.  Match firmware/sim/dac_chain.hexa N_CHANNELS=4.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Channel {
    Hv     = 0,
    Coil   = 1,
    Pump   = 2,
    Valve  = 3,
}

pub struct Ltc2668<SPI, CS> {
    spi: SPI,
    cs:  CS,
}

impl<SPI, CS, E> Ltc2668<SPI, CS>
where
    SPI: SpiBus<u8, Error = E>,
    CS:  OutputPin<Error = Infallible>,
{
    pub fn new(spi: SPI, mut cs: CS) -> Self {
        // CS idle high
        cs.set_high().ok();
        Self { spi, cs }
    }

    /// Write a 16-bit code and immediately update the channel output.
    /// Returns SPI error type unchanged so the RTIC task can react.
    pub fn write_and_update(&mut self, ch: Channel, code: u16) -> Result<(), E> {
        let word: [u8; 3] = [
            (Cmd::WriteDacAndUpdate as u8) << 4 | (ch as u8 & 0x0F),
            (code >> 8)   as u8,
            (code & 0xFF) as u8,
        ];
        self.cs.set_low().ok();
        let r = self.spi.write(&word);
        self.cs.set_high().ok();
        r
    }

    /// Power-down the chip — used by the watchdog/safety path.
    pub fn power_down_chip(&mut self) -> Result<(), E> {
        let word: [u8; 3] = [(Cmd::PowerDownChip as u8) << 4, 0, 0];
        self.cs.set_low().ok();
        let r = self.spi.write(&word);
        self.cs.set_high().ok();
        r
    }
}

// Tests for SPI command encoding will land via embedded-hal-mock in
// the integration-test crate (firmware/mcu/tests/) — this avoids
// pulling test-only dependencies into the no_std build.
