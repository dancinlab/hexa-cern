//! hexa-cern-laser-mcu library — §A.6.1 step E3.2
//!
//! Splits the firmware into reusable modules so each driver can be
//! unit-tested with `cargo test --target thumbv7em-none-eabihf` once
//! the hardware-specific bits are stubbable.
//!
//! Module map:
//!   - `consts` — n=6 lattice anchors + register addresses (single
//!     source of truth shared with .hexa sims + Verilog HDL)
//!   - `pi`     — PI controller (firmware/sim/control_loop.hexa parity)
//!   - `dac`    — LTC2668-class 16-bit SPI DAC driver
//!   - `state`  — TriggerSm + DAC code conversion utility
//!   - `interlock` — InterlockOk newtype + raw pin sampling
//!
//! The `app` binary in src/main.rs imports these modules.

#![no_std]

pub mod consts;
pub mod pi;
pub mod dac;
pub mod state;
pub mod interlock;
