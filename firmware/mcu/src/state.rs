//! Trigger state machine + DAC code conversion.
//!
//! Mirrors firmware/hdl/timing_ctrl.v RTL exactly so software and
//! hardware stay in sync.  See firmware/sim/timing_chain.hexa for the
//! numerical model and firmware/hdl/timing_ctrl.v for the Verilog.

use crate::consts::{CLK_HZ, DAC_VFS_MV, D_TRIG_CYCLES, GATE_CYCLES, TICK_HZ};
use crate::interlock::InterlockOk;

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

/// Convert a millivolt setpoint to a 16-bit DAC code.
/// Clamps out-of-range to [0, 0xFFFF]. Round-half-down via integer
/// truncation. Same mapping as firmware/sim/dac_chain.hexa.
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

// Unit tests (dac_code_for_mv endpoint mapping + clamp behavior) move
// to the integration-test crate alongside the PI tests.
