//! Interlock typestate.
//!
//! `InterlockOk` is constructed *only* when all 4 interlock pins read
//! OK simultaneously.  Driver code that wants to arm a trigger must
//! pass `&InterlockOk` → compiler enforces the safety check.
//!
//! Mirrors firmware/hdl/timing_ctrl_regs.v `interlock_ok_d1` edge
//! detector + the `enable_and_interlock` AND term in
//! firmware/hdl/timing_ctrl_top.v.

#[derive(Clone, Copy, Debug)]
pub struct InterlockOk(());

impl InterlockOk {
    /// Construct only when ALL four interlock conditions are OK.
    ///
    /// SAFETY: the four boolean inputs come from hardware GPIOs sampled
    /// within the same cycle; never call this with stale or mixed-time
    /// values. Production drivers MUST sample all 4 in one MMIO read or
    /// at least within a critical-section.
    pub fn from_pins(hv_ok: bool, vacuum_ok: bool, water_ok: bool, shutter_ok: bool) -> Option<Self> {
        if hv_ok && vacuum_ok && water_ok && shutter_ok {
            Some(InterlockOk(()))
        } else {
            None
        }
    }
}

/// Bit layout for a 4-bit interlock raw register (matches the
/// firmware/hdl/timing_ctrl_regs.v INTERLOCK_RAW register).
#[derive(Clone, Copy, Debug)]
pub struct InterlockRaw(pub u8);

impl InterlockRaw {
    pub fn hv_ok(self) -> bool       { (self.0 & 0b0001) != 0 }
    pub fn vacuum_ok(self) -> bool   { (self.0 & 0b0010) != 0 }
    pub fn water_ok(self) -> bool    { (self.0 & 0b0100) != 0 }
    pub fn shutter_ok(self) -> bool  { (self.0 & 0b1000) != 0 }

    pub fn all_ok(self) -> bool {
        self.hv_ok() && self.vacuum_ok() && self.water_ok() && self.shutter_ok()
    }

    pub fn into_typestate(self) -> Option<InterlockOk> {
        InterlockOk::from_pins(
            self.hv_ok(),
            self.vacuum_ok(),
            self.water_ok(),
            self.shutter_ok(),
        )
    }
}

// Unit tests (InterlockOk all-or-nothing constructor + InterlockRaw bit
// decoder) move to the integration-test crate.
