use std::f64::consts::TAU;

use crate::effect::{
    Effect, EffectState, WIDTH, cycle_length, spatial_frequency, temporal_speed,
};
use crate::frame::VBlockFrame;
use rand::Rng;

pub struct VBlockWave {
    state: EffectState,
}

impl VBlockWave {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockWave {
    fn name(&self) -> &'static str {
        "vblock-wave"
    }
    fn description(&self) -> &'static str {
        "Sine wave using vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        10
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockWave {
    fn render(&mut self, frame: usize) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = ((i as f64 + frame as f64) * spatial_frequency::LOW).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct VBlockFill {
    state: EffectState,
}

impl VBlockFill {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockFill {
    fn name(&self) -> &'static str {
        "vblock-fill"
    }
    fn description(&self) -> &'static str {
        "Progressive fill using vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        13
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockFill {
    fn render(&mut self, frame: usize) -> String {
        let cycle = WIDTH + 4;
        let mut f = VBlockFrame::new(WIDTH);
        let pos = frame % cycle;
        for i in 0..WIDTH {
            if i < pos {
                let dist = pos - i;
                let v = (1.0 - (dist - 1) as f64 / (WIDTH - 1) as f64).max(0.0);
                f.set(i, v);
            }
        }
        f.render().join("\n")
    }
}

pub struct VBlockTide {
    state: EffectState,
}

impl VBlockTide {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockTide {
    fn name(&self) -> &'static str {
        "vblock-tide"
    }
    fn description(&self) -> &'static str {
        "Tidal gradient using vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        25
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockTide {
    fn render(&mut self, frame: usize) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        let phase = (frame as f64 * temporal_speed::GENTLE).sin() / 2.0 + 0.5;
        for i in 0..WIDTH {
            let t = i as f64 / (WIDTH - 1) as f64;
            let density = phase * (1.0 - t) + (1.0 - phase) * t;
            f.set(i, density);
        }
        f.render().join("\n")
    }
}

pub struct VBlockBreathe {
    state: EffectState,
}

impl VBlockBreathe {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockBreathe {
    fn name(&self) -> &'static str {
        "vblock-breathe"
    }
    fn description(&self) -> &'static str {
        "Breathing vertical block animation"
    }
    fn cycle_length(&self) -> usize {
        18
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockBreathe {
    fn render(&mut self, frame: usize) -> String {
        let phase = frame % cycle_length::MEDIUM;
        let v = (phase as f64 * std::f64::consts::PI / 10.0).sin() / 2.0 + 0.5;
        let mut f = VBlockFrame::new(WIDTH);
        for i in 0..WIDTH {
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct VBlockBounce {
    state: EffectState,
}

impl VBlockBounce {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockBounce {
    fn name(&self) -> &'static str {
        "vblock-bounce"
    }
    fn description(&self) -> &'static str {
        "Bouncing highlight in vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        16
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockBounce {
    fn render(&mut self, frame: usize) -> String {
        let period = (WIDTH - 1) * 2;
        let mut f = VBlockFrame::new(WIDTH);
        let t = frame % period;
        let pos = if t < WIDTH { t } else { period - t };
        for i in 0..WIDTH {
            let dist = (pos as isize - i as isize).unsigned_abs() as f64;
            if dist < 4.0 {
                f.set(i, (1.0 - dist * 0.3).max(0.0));
            }
        }
        f.render().join("\n")
    }
}

pub struct VBlockPulse {
    state: EffectState,
}

impl VBlockPulse {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockPulse {
    fn name(&self) -> &'static str {
        "vblock-pulse"
    }
    fn description(&self) -> &'static str {
        "Expanding pulse in vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        13
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockPulse {
    fn render(&mut self, frame: usize) -> String {
        let cycle = WIDTH + 4;
        let mut f = VBlockFrame::new(WIDTH);
        let center = (WIDTH - 1) as f64 / 2.0;
        let age = (frame % cycle) as f64;
        let intensity = (age / (cycle as f64 * 0.6)).min(1.0);
        for i in 0..WIDTH {
            let dist = (i as f64 - center).abs();
            let wave = (1.0 - (dist - age + 2.0) * 0.25).max(0.0);
            f.set(i, wave * intensity);
        }
        f.render().join("\n")
    }
}

pub struct VBlockRipple {
    state: EffectState,
}

impl VBlockRipple {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockRipple {
    fn name(&self) -> &'static str {
        "vblock-ripple"
    }
    fn description(&self) -> &'static str {
        "Ripple from center in vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        16
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockRipple {
    fn render(&mut self, frame: usize) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        let cx = (WIDTH - 1) as f64 / 2.0;
        for i in 0..WIDTH {
            let dist = (i as f64 - cx).abs() / cx;
            let wave = ((dist * spatial_frequency::EXTRA_DENSE
                - frame as f64 * temporal_speed::MODERATE)
                .sin()
                + 1.0)
                / 2.0;
            let density = (wave - dist * 0.5).max(0.0);
            f.set(i, density);
        }
        f.render().join("\n")
    }
}

pub struct VBlockRain {
    state: EffectState,
    levels: Vec<f64>,
}

impl VBlockRain {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
            levels: vec![0.0; WIDTH],
        }
    }
}

impl Effect for VBlockRain {
    fn name(&self) -> &'static str {
        "vblock-rain"
    }
    fn description(&self) -> &'static str {
        "Rain drops with persistent levels"
    }
    fn cycle_length(&self) -> usize {
        25
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockRain {
    fn render(&mut self, _frame: usize) -> String {
        for i in 0..WIDTH {
            if self.state.rng.random::<f64>() < 0.12 {
                self.levels[i] =
                    (self.levels[i] + self.state.rng.random_range(0.5..=1.0)).min(1.0);
            } else {
                self.levels[i] *= 0.85;
            }
        }
        let mut f = VBlockFrame::new(WIDTH);
        for i in 0..WIDTH {
            f.set(i, self.levels[i]);
        }
        f.render().join("\n")
    }
}

pub struct VBlockCascade {
    state: EffectState,
}

impl VBlockCascade {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for VBlockCascade {
    fn name(&self) -> &'static str {
        "vblock-cascade"
    }
    fn description(&self) -> &'static str {
        "Cascade gaps in vertical blocks"
    }
    fn cycle_length(&self) -> usize {
        26
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl VBlockCascade {
    fn render(&mut self, frame: usize) -> String {
        let gaps: usize = 3;
        let spacing: f64 = WIDTH as f64 / 3.0;
        let mut f = VBlockFrame::new(WIDTH);
        let phase = frame as f64 / self.cycle_length() as f64;
        let t = phase * (WIDTH + 4) as f64;
        for i in 0..WIDTH {
            let mut v: f64 = 1.0;
            for g in 0..gaps {
                let pos = (t + g as f64 * spacing) % (WIDTH + 4) as f64 - 2.0;
                let wobble = 0.3 * (TAU * phase + g as f64 * 2.1).sin();
                let half = 1.0 + wobble;
                let dist = i as f64 - pos;
                if dist.abs() < half {
                    v = v.min(((dist.abs() - half + 0.6) / 0.6).max(0.0));
                }
            }
            f.set(i, v);
        }
        f.render().join("\n")
    }
}
