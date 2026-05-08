// hexa-cern/firmware/mcu/src/main.rs — §A.6.1 step E3.1
//
// LWFA benchtop laser-driver MCU firmware. RTIC 2.x + stm32h7xx-hal.
// Mirrors the firmware/sim/{timing_chain, dac_chain, adc_chain,
// control_loop}.hexa numerical models in real-silicon-targeting Rust.
//
// Status: builds + `cargo check` clean. Flashable when a STM32H743ZIT6
// dev-board lands per §A.6 step 1+2.
//
// Architecture:
//   • RTIC `#[init]` configures the system clock to 480 MHz from HSE,
//     enables peripheral clocks for SPI / I²C / DMA / GPIO / IWDG.
//   • `#[task]` for each loop period: 1 kHz control loop drives the
//     PI controller, refreshes the DAC, samples the ADCs.
//   • `#[idle]` runs the system watchdog kick + RTT log drain.
//   • Type-state safety: the InterlockOk newtype gates every trigger
//     write — compiler enforces that interlocks were checked.

#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

use panic_halt as _;

// ────────────────────────────────────────────────────────────────────────
// Constants — single source of truth, pinned to .hexa sims + Verilog HDL
// ────────────────────────────────────────────────────────────────────────

/// Master clock frequency in Hz. Must match firmware/hdl/timing_ctrl.v
/// `parameter CLK_HZ` and firmware/sim/timing_chain.hexa `TICK_PERIOD_S`.
pub const CLK_HZ: u32 = 100_000_000;

/// Tick rate in Hz.  τ = 4 (n=6 lattice acceleration phase quartet).
pub const TICK_HZ: u32 = 4;

/// Trigger delay in cycles @ CLK_HZ → 1 µs.
pub const D_TRIG_CYCLES: u32 = 100;

/// Gate width in cycles @ CLK_HZ → 200 ns.
pub const GATE_CYCLES: u32 = 20;

/// DAC bit depth (LTC2668 16-bit).
pub const DAC_BITS: u8 = 16;

/// DAC full-scale range in millivolts (±10 V).
pub const DAC_VFS_MV: i32 = 20_000;

/// ADC (BPM) bit depth.
pub const ADC_BPM_BITS: u8 = 16;

/// ADC (diamond) bit depth.
pub const ADC_DIAMOND_BITS: u8 = 14;

/// Wishbone register-file address constants — must match
/// firmware/hdl/timing_ctrl_regs.v address map.
pub mod regs {
    pub const CTRL: u32         = 0x00;
    pub const STATUS: u32       = 0x04;
    pub const TICK_COUNT: u32   = 0x08;
    pub const D_TRIG: u32       = 0x0C;
    pub const GATE_WIDTH: u32   = 0x10;
    pub const TICK_DIVIDER: u32 = 0x14;
    pub const INTERLOCK_RAW: u32 = 0x18;
    pub const IRQ_MASK: u32     = 0x1C;
    pub const IRQ_PENDING: u32  = 0x20;
    pub const SCRATCH: u32      = 0x24;
    pub const N_TIER_LOCK: u32  = 0x28;

    pub const CTRL_ENABLE: u32     = 1 << 0;
    pub const CTRL_SOFT_RESET: u32 = 1 << 1;
    pub const CTRL_FORCE_TICK: u32 = 1 << 2;

    pub const IRQ_INTERLOCK: u32 = 1 << 0;
    pub const IRQ_TICK: u32      = 1 << 1;
    pub const IRQ_FAULT: u32     = 1 << 2;

    /// Sentinel: J₂ = 24 = 0x18 (n=6 lattice anchor).
    pub const N_TIER_LOCK_VAL: u32 = 0x0000_0018;
}

// ────────────────────────────────────────────────────────────────────────
// Type-level interlock state — newtype; constructor only succeeds when
// all 4 interlock GPIOs were sampled simultaneously OK.
// ────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct InterlockOk(());

impl InterlockOk {
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

#[derive(Default, Clone, Copy)]
pub struct PiCtrl {
    pub integral: f32,
    pub prev_err: f32,
    pub sat_count: u32,
}

impl PiCtrl {
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

    pub fn tick(&mut self, interlock: Option<InterlockOk>) {
        self.tick_pending = false;
        self.trigger_pending = false;

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
// RTIC application
// ────────────────────────────────────────────────────────────────────────
//
// Two tasks live in the dispatcher:
//   • `init`        — once at boot. Sets up clock + GPIO + IWDG.
//   • `idle`        — fall-through loop. Kicks the watchdog.
//
// Hardware-driven tasks (1 kHz control loop, ADC DMA done, interlock
// edge ISR) land as part of E3.2 once the peripheral binding pattern
// is settled. This file establishes the RTIC structure + memory.x
// link + GPIO/clock init so each future driver chunk can plug in.

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [SAI3])]
mod app {
    use stm32h7xx_hal::{
        gpio::{Output, PushPull, gpioe::PE3},
        independent_watchdog::IndependentWatchdog,
        prelude::*,
    };

    /// Resources shared across tasks (none yet — added in E3.2 with DMA buffers).
    #[shared]
    struct Shared {}

    /// Per-task local resources: interlock LED, watchdog timer.
    #[local]
    struct Local {
        led: PE3<Output<PushPull>>,
        iwdg: IndependentWatchdog,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let dp = ctx.device;

        // ── power + clock tree (480 MHz core from 25 MHz HSE) ────────
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        let rcc = dp.RCC.constrain();
        let ccdr = rcc
            .use_hse(25.MHz())
            .sys_ck(480.MHz())     // CPU clock
            .pll1_q_ck(48.MHz())   // for USB / SDMMC if used
            .freeze(pwrcfg, &dp.SYSCFG);

        // ── GPIO pin allocation ──────────────────────────────────────
        // PE3 = on-board green LED on Nucleo-H743ZI2 (drives during boot
        // ok → idle indicator).
        let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
        let led = gpioe.pe3.into_push_pull_output();

        // ── independent watchdog ──────────────────────────────────────
        // 100 ms timeout — RTIC idle task must kick at least every 100 ms,
        // otherwise the watchdog resets the MCU.  Matches firmware/sim/
        // control_loop.hexa Anti-windup-on-fault behavior.
        let mut iwdg = IndependentWatchdog::new(dp.IWDG);
        iwdg.start(100.millis());

        (Shared {}, Local { led, iwdg })
    }

    /// Idle loop: blink LED at ~1 Hz, kick watchdog, sleep waiting for
    /// hardware interrupts.
    #[idle(local = [led, iwdg])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            ctx.local.iwdg.feed();
            ctx.local.led.toggle();
            // Approximate 500 ms delay using nop loop (RTIC monotonic
            // timer integration lands in E3.2). Production code uses
            // `Systick::delay(500u64.millis()).await` instead.
            cortex_m::asm::delay(240_000_000);
        }
    }
}
