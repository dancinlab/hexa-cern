// hexa-cern/firmware/mcu/src/main.rs — §A.6.1 step E3.2
//
// LWFA benchtop laser-driver MCU firmware. RTIC 2.x + stm32h7xx-hal.
// Library logic lives in src/{consts,pi,state,interlock,dac}.rs so it
// can be unit-tested with `cargo test`. This file is just the RTIC
// app glue + boot/clock/IWDG/GPIO init.
//
// Status: builds + `cargo check` clean. `cargo test --lib` runs the
// pure-software tests on the host. Flashable to STM32H743ZIT6 via
// `cargo build --release` + `probe-rs run --chip STM32H743ZITx`.
//
// SKELETON ONLY for board-driven peripherals (SPI DAC connection / DMA
// ADC streams) — these come online once §A.6 step 1+2 land a board.

#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

use panic_halt as _;

// Re-export the lib crate so RTIC dispatch macros find the types.
use hexa_cern_laser_mcu as fw;

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [SAI3])]
mod app {
    use super::fw;
    use stm32h7xx_hal::{
        gpio::{Output, PushPull, gpioe::PE3},
        independent_watchdog::IndependentWatchdog,
        prelude::*,
    };

    /// Resources shared across tasks.
    #[shared]
    struct Shared {
        pi: fw::pi::PiCtrl,
    }

    /// Per-task local resources.
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
            .sys_ck(480.MHz())
            .pll1_q_ck(48.MHz())
            .freeze(pwrcfg, &dp.SYSCFG);

        // ── GPIO pin allocation ──────────────────────────────────────
        // PE3 = on-board green LED on Nucleo-H743ZI2.
        let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
        let led = gpioe.pe3.into_push_pull_output();

        // ── independent watchdog ──────────────────────────────────────
        // 100 ms timeout — RTIC idle task must kick at least every 100 ms.
        let mut iwdg = IndependentWatchdog::new(dp.IWDG);
        iwdg.start(100.millis());

        (
            Shared { pi: fw::pi::PiCtrl::default() },
            Local { led, iwdg },
        )
    }

    /// Idle loop: blink LED at ~1 Hz, kick watchdog, sleep waiting for
    /// hardware interrupts. Production firmware replaces this with
    /// `Systick::delay(...).await` once the monotonic timer integration
    /// lands in E3.3.
    #[idle(local = [led, iwdg])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            ctx.local.iwdg.feed();
            ctx.local.led.toggle();
            cortex_m::asm::delay(240_000_000);
        }
    }
}
