//! Single-source-of-truth constants — must match firmware/sim/*.hexa
//! and firmware/hdl/*.v exactly. cross_doc_audit.hexa enforces this.

/// Master clock frequency in Hz.
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

/// PI controller gains (firmware/sim/control_loop.hexa final tuning).
pub const PI_KP: f32 = 0.5;
pub const PI_KI: f32 = 50.0;
pub const PI_LIMIT: f32 = 1.5;

/// Wishbone register-file address constants — must match
/// firmware/hdl/timing_ctrl_regs.v address map.
pub mod regs {
    pub const CTRL: u32          = 0x00;
    pub const STATUS: u32        = 0x04;
    pub const TICK_COUNT: u32    = 0x08;
    pub const D_TRIG: u32        = 0x0C;
    pub const GATE_WIDTH: u32    = 0x10;
    pub const TICK_DIVIDER: u32  = 0x14;
    pub const INTERLOCK_RAW: u32 = 0x18;
    pub const IRQ_MASK: u32      = 0x1C;
    pub const IRQ_PENDING: u32   = 0x20;
    pub const SCRATCH: u32       = 0x24;
    pub const N_TIER_LOCK: u32   = 0x28;

    pub const CTRL_ENABLE: u32     = 1 << 0;
    pub const CTRL_SOFT_RESET: u32 = 1 << 1;
    pub const CTRL_FORCE_TICK: u32 = 1 << 2;

    pub const IRQ_INTERLOCK: u32 = 1 << 0;
    pub const IRQ_TICK: u32      = 1 << 1;
    pub const IRQ_FAULT: u32     = 1 << 2;

    /// Sentinel: J₂ = 24 = 0x18 (n=6 lattice anchor).
    pub const N_TIER_LOCK_VAL: u32 = 0x0000_0018;
}
