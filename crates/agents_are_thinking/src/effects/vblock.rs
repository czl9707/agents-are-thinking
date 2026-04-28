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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockWave {
    fn name() -> &'static str {
        "vblock-wave"
    }
    fn description() -> &'static str {
        "Sine wave using vertical blocks"
    }
    fn cycle_length() -> usize {
        10
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockWave {
    fn render(&mut self) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = ((i as f64 + self.state.frame as f64) * spatial_frequency::LOW).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct VBlockFill {
    state: EffectState,
}

impl VBlockFill {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockFill {
    fn name() -> &'static str {
        "vblock-fill"
    }
    fn description() -> &'static str {
        "Progressive fill using vertical blocks"
    }
    fn cycle_length() -> usize {
        Self::FILL_CYCLE
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockFill {
    const FILL_CYCLE: usize = WIDTH + 4;

    fn render(&mut self) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        let pos = self.state.frame % Self::FILL_CYCLE;
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockTide {
    fn name() -> &'static str {
        "vblock-tide"
    }
    fn description() -> &'static str {
        "Tidal gradient using vertical blocks"
    }
    fn cycle_length() -> usize {
        25
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockTide {
    fn render(&mut self) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        let phase = (self.state.frame as f64 * temporal_speed::GENTLE).sin() / 2.0 + 0.5;
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockBreathe {
    fn name() -> &'static str {
        "vblock-breathe"
    }
    fn description() -> &'static str {
        "Breathing vertical block animation"
    }
    fn cycle_length() -> usize {
        18
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockBreathe {
    fn render(&mut self) -> String {
        let phase = self.state.frame % cycle_length::MEDIUM;
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockBounce {
    fn name() -> &'static str {
        "vblock-bounce"
    }
    fn description() -> &'static str {
        "Bouncing highlight in vertical blocks"
    }
    fn cycle_length() -> usize {
        Self::BOUNCE_PERIOD
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockBounce {
    const BOUNCE_PERIOD: usize = (WIDTH - 1) * 2;

    fn render(&mut self) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        let t = self.state.frame % Self::BOUNCE_PERIOD;
        let pos = if t < WIDTH { t } else { Self::BOUNCE_PERIOD - t };
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockPulse {
    fn name() -> &'static str {
        "vblock-pulse"
    }
    fn description() -> &'static str {
        "Expanding pulse in vertical blocks"
    }
    fn cycle_length() -> usize {
        WIDTH + 4
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockPulse {
    const CENTER: f64 = (WIDTH - 1) as f64 / 2.0;

    fn render(&mut self) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        let age = self.state.frame as f64;
        let intensity: f64 = (self.state.frame as f64 / (Self::cycle_length() as f64 * 0.6)).min(1.0);
        for i in 0..WIDTH {
            let dist = (i as f64 - Self::CENTER).abs();
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockRipple {
    fn name() -> &'static str {
        "vblock-ripple"
    }
    fn description() -> &'static str {
        "Ripple from center in vertical blocks"
    }
    fn cycle_length() -> usize {
        16
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockRipple {
    const CENTER: f64 = (WIDTH - 1) as f64 / 2.0;

    fn render(&mut self) -> String {
        let mut f = VBlockFrame::new(WIDTH);
        for i in 0..WIDTH {
            let dist = (i as f64 - Self::CENTER).abs() / Self::CENTER;
            let wave = ((dist * spatial_frequency::EXTRA_DENSE
                - self.state.frame as f64 * temporal_speed::MODERATE)
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
            levels: vec![0.0; WIDTH],
        }
    }
}

impl Effect for VBlockRain {
    fn name() -> &'static str {
        "vblock-rain"
    }
    fn description() -> &'static str {
        "Rain drops with persistent levels"
    }
    fn cycle_length() -> usize {
        25
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockRain {
    fn render(&mut self) -> String {
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for VBlockCascade {
    fn name() -> &'static str {
        "vblock-cascade"
    }
    fn description() -> &'static str {
        "Cascade gaps in vertical blocks"
    }
    fn cycle_length() -> usize {
        26
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl VBlockCascade {
    const CASCADE_SPACING: f64 = WIDTH as f64 / 3.0;

    fn render(&mut self) -> String {
        let gaps: usize = 3;
        let mut f = VBlockFrame::new(WIDTH);
        let phase = self.state.frame as f64 / Self::cycle_length() as f64;
        let t = phase * (WIDTH + 4) as f64;
        for i in 0..WIDTH {
            let mut v: f64 = 1.0;
            for g in 0..gaps {
                let pos = (t + g as f64 * Self::CASCADE_SPACING) % (WIDTH + 4) as f64 - 2.0;
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
