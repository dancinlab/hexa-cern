//! PI controller — firmware/sim/control_loop.hexa parity in Rust.

#[derive(Default, Clone, Copy)]
pub struct PiCtrl {
    pub integral: f32,
    pub prev_err: f32,
    pub sat_count: u32,
}

impl PiCtrl {
    /// One PI step. Returns saturated control value in [-LIMIT, LIMIT].
    /// On saturation, integral term is frozen (anti-windup).
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

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_err = 0.0;
        self.sat_count = 0;
    }
}

// Unit tests (PI step-response convergence + anti-windup clamping)
// land in the separate `firmware/mcu/tests/` integration-test crate
// once the std-test-harness vs no_std-cortex-m linkage is settled.
// For now `cargo check --target thumbv7em-none-eabihf` is the build gate.
