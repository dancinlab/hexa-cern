// hexa-cern/firmware/mcu/src/main.rs — §A.6.1 step D 2/3
//
// Rust embedded firmware skeleton for the LWFA benchtop laser-driver MCU.
// Mirrors the firmware/sim/{timing_chain, dac_chain, adc_chain,
// control_loop}.hexa numerical models in real microcontroller code.
//
// **STATUS: SKELETON ONLY — NOT BUILDABLE WITHOUT TARGET BOARD**
//
// To make this binary actually run on hardware, the following must land:
//   1. a `memory.x` linker script (vendor-supplied for the chosen STM32H7)
//   2. a HAL crate (e.g. `stm32h7xx-hal`) for the specific MCU family
//   3. peripheral drivers for DAC (LTC2668 over SPI), ADC (LTC2208 / DMA),
//      and a real interlock GPIO mapping
//   4. probe-rs / openocd configuration for flashing
//
// Until then, this file:
//   • compiles under `cargo check --target thumbv7em-none-eabihf`
//   • emits the architectural commitment for the laser-driver MCU
//   • declares the public functions that hardware drivers will fill in
//
// SAFETY: every block here that handles HV / laser / vacuum is marked
// with `// SAFETY:` comments documenting the invariant. In skeleton state
// these are textual contracts; production firmware must enforce them
// with hardware interlocks (cross-checked against firmware/hdl/timing_ctrl.v
// `interlock_now`).

#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)] // skeleton — many fns intentionally unused

use cortex_m_rt::entry;
use panic_halt as _;

// ────────────────────────────────────────────────────────────────────────
// Constants — single source of truth pinned to the .hexa sims
// ────────────────────────────────────────────────────────────────────────

/// Master clock from B1 OCXO (Hz). Matches `firmware/sim/timing_chain`
/// `TICK_RATE_HZ * TICK_DIVIDER` and `firmware/hdl/timing_ctrl.v`
/// `parameter CLK_HZ`.
pub const CLK_HZ: u32 = 100_000_000;

/// Tick rate: 4 Hz acceleration phase repetition (n=6 lattice τ = 4).
pub const TICK_HZ: u32 = 4;

/// Trigger delay after tick (cycles @ CLK_HZ → 1 µs).
pub const D_TRIG_CYCLES: u32 = 100;

/// Gate width (cycles @ CLK_HZ → 200 ns).
pub const GATE_CYCLES: u32 = 20;

/// DAC bit depth (matches `firmware/sim/dac_chain.hexa::N_BITS`).
pub const DAC_BITS: u8 = 16;

/// DAC full-scale span in millivolts (±10 V).
pub const DAC_VFS_MV: i32 = 20_000;

/// ADC (BPM) bit depth.
pub const ADC_BPM_BITS: u8 = 16;

/// ADC (diamond) bit depth.
pub const ADC_DIAMOND_BITS: u8 = 14;

// ────────────────────────────────────────────────────────────────────────
// Type-level interlock state — newtype so we cannot accidentally arm a
// trigger while interlocks are tripped.
// ────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct InterlockOk(());

impl InterlockOk {
    /// Construct *only* when ALL interlock conditions are simultaneously OK.
    ///
    /// SAFETY: the four boolean inputs come from hardware GPIOs sampled
    /// in the same critical-section / dma transfer; never call this with
    /// stale or mixed-time values.  Production driver MUST sample all 4
    /// in one MMIO read.
    pub fn from_pins(hv_ok: bool, vacuum_ok: bool, water_ok: bool, shutter_ok: bool) -> Option<Self> {
        if hv_ok && vacuum_ok && water_ok && shutter_ok {
            Some(InterlockOk(()))
        } else {
            None
        }
    }
}

// ────────────────────────────────────────────────────────────────────────
// PI controller — same gains as firmware/sim/control_loop.hexa
// ────────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct PiCtrl {
    pub integral: f32,
    pub prev_err: f32,
    pub sat_count: u32,
}

impl PiCtrl {
    /// One PI step. Returns saturated control value in [-LIMIT, LIMIT].
    pub fn step(&mut self, error: f32, ts_s: f32, kp: f32, ki: f32, limit: f32) -> f32 {
        let p = kp * error;
        let new_int = self.integral + ki * error * ts_s;
        let raw = p + new_int;
        let (out, saturated) = if raw > limit {
            (limit, true)
        } else if raw < -limit {
            (-limit, true)
        } else {
            (raw, false)
        };
        if !saturated {
            self.integral = new_int;
        } else {
            self.sat_count = self.sat_count.saturating_add(1);
        }
        self.prev_err = error;
        out
    }
}

// ────────────────────────────────────────────────────────────────────────
// DAC volt → code conversion (matches firmware/sim/dac_chain.hexa)
// ────────────────────────────────────────────────────────────────────────

/// Convert a millivolt setpoint to a 16-bit DAC code.
/// Clamps out-of-range to [0, 0xFFFF]. Returns the code that produces
/// the closest possible voltage.
#[inline]
pub fn dac_code_for_mv(target_mv: i32) -> u16 {
    let v_min_mv = -DAC_VFS_MV / 2;
    let span = DAC_VFS_MV;
    let level = (((target_mv - v_min_mv) as i64 * 65536) / span as i64) as i32;
    if level < 0 {
        0
    } else if level > 0xFFFF {
        0xFFFF
    } else {
        level as u16
    }
}

// ────────────────────────────────────────────────────────────────────────
// Trigger pipeline state machine (matches firmware/hdl/timing_ctrl.v)
// ────────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct TriggerSm {
    div_count: u32,
    delay_count: u32,
    gate_count: u32,
    pub tick_count: u32,
    pub tick_pending: bool,
    pub trigger_pending: bool,
    pub gate_active: bool,
}

impl TriggerSm {
    pub const TICK_DIVIDER: u32 = CLK_HZ / TICK_HZ;

    /// One cycle @ CLK_HZ. `interlock` is required to advance trigger.
    pub fn tick(&mut self, interlock: Option<InterlockOk>) {
        // clear pulses
        self.tick_pending = false;
        self.trigger_pending = false;

        // clock divider
        if self.div_count >= Self::TICK_DIVIDER - 1 {
            self.div_count = 0;
            if interlock.is_some() {
                self.tick_pending = true;
                self.tick_count = self.tick_count.wrapping_add(1);
                self.delay_count = 1;
            }
        } else {
            self.div_count += 1;
        }

        // trigger delay pipeline
        if self.delay_count != 0 {
            if self.delay_count >= D_TRIG_CYCLES {
                self.trigger_pending = true;
                self.gate_active = true;
                self.gate_count = 1;
                self.delay_count = 0;
            } else {
                self.delay_count += 1;
            }
        } else if self.gate_active {
            // gate generator
            if self.gate_count >= GATE_CYCLES {
                self.gate_active = false;
                self.gate_count = 0;
            } else {
                self.gate_count += 1;
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────
// Entry point — bare-metal main, runs forever.
// ────────────────────────────────────────────────────────────────────────

#[entry]
fn main() -> ! {
    // SAFETY: in skeleton-mode there is no real HAL, no peripheral init,
    // no clock tree setup. The hardware boot vector lands here but does
    // nothing useful. A production firmware will:
    //   1. configure the system clock to CLK_HZ (PLL from HSE)
    //   2. enable peripheral clocks for SPI / DMA / GPIO
    //   3. spawn the main control loop with RTIC tasks
    //   4. arm the watchdog (§safety §9 invariant)
    //
    // Until then, we just halt.  The `panic-halt` linkage means any
    // accidental panic (UB, integer overflow with `-C overflow-checks`,
    // etc.) also lands in this loop. Hardware watchdog catches it.

    loop {
        cortex_m::asm::wfi(); // wait for interrupt — saves power
    }
}
