use std::f64::consts::TAU;

use crate::effect::{Effect, EffectState, WIDTH, spatial_frequency, temporal_speed};
use crate::frame::BarFrame;
use rand::Rng;

pub struct BarBounce {
    state: EffectState,
    phases: Vec<f64>,
    speeds: Vec<f64>,
}

impl BarBounce {
    pub fn new() -> Self {
        let mut state = EffectState::new(42, Self::cycle_length());
        let phases: Vec<f64> = (0..WIDTH)
            .map(|_| state.rng.random_range(0.0..=TAU))
            .collect();
        let speeds: Vec<f64> = (0..WIDTH)
            .map(|_| state.rng.random_range(temporal_speed::GENTLE..=temporal_speed::FAST))
            .collect();
        Self {
            state,
            phases,
            speeds,
        }
    }
}

impl Effect for BarBounce {
    fn name() -> &'static str {
        "bar-bounce"
    }
    fn description() -> &'static str {
        "Bouncing bars with random phases"
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

impl BarBounce {
    fn render(&mut self) -> String {
        let mut f = BarFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = (self.phases[i] + self.state.frame as f64 * self.speeds[i]).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct BarWave {
    state: EffectState,
}

impl BarWave {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BarWave {
    fn name() -> &'static str {
        "bar-wave"
    }
    fn description() -> &'static str {
        "Sine wave using bar characters"
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

impl BarWave {
    fn render(&mut self) -> String {
        let mut f = BarFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = (i as f64 * spatial_frequency::LOW + self.state.frame as f64 * temporal_speed::GENTLE)
                .sin()
                / 2.0
                + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct BarSeeSaw {
    state: EffectState,
}

impl BarSeeSaw {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BarSeeSaw {
    fn name() -> &'static str {
        "bar-seesaw"
    }
    fn description() -> &'static str {
        "See-saw gradient in bar"
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

impl BarSeeSaw {
    fn render(&mut self) -> String {
        let mut f = BarFrame::new(WIDTH);
        let t = (self.state.frame as f64 * temporal_speed::GENTLE).sin() / 2.0 + 0.5;
        for i in 0..WIDTH {
            let ratio = if WIDTH > 1 {
                i as f64 / (WIDTH - 1) as f64
            } else {
                0.5
            };
            let v = t * (1.0 - ratio) + (1.0 - t) * ratio;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}
