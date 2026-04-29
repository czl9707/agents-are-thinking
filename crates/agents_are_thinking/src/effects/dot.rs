use std::f64::consts::TAU;

use crate::effect::{Effect, EffectState, WIDTH, cycle_length, pause, spatial_frequency, temporal_speed};
use crate::frame::DotFrame;
use rand::Rng;

pub struct DotWave {
    state: EffectState,
}

impl DotWave {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for DotWave {
    fn name() -> &'static str {
        "dot-wave"
    }
    fn description() -> &'static str {
        "Sine wave using dot characters"
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

impl DotWave {
    fn render(&mut self) -> String {
        let mut f = DotFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = ((i as f64 + self.state.frame as f64) * spatial_frequency::LOW).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct DotHeartbeat {
    state: EffectState,
}

impl DotHeartbeat {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for DotHeartbeat {
    fn name() -> &'static str {
        "dot-heartbeat"
    }
    fn description() -> &'static str {
        "Heartbeat pulse in dots"
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

impl DotHeartbeat {
    fn render(&mut self) -> String {
        let mut f = DotFrame::new(WIDTH);
        let phase = self.state.frame % cycle_length::MEDIUM;
        let v = if phase < 5 {
            phase as f64 / 5.0
        } else if phase < 8 {
            1.0 - (phase - 5) as f64 / 6.0
        } else if phase < 10 {
            (phase - 8) as f64 / 4.0
        } else {
            (1.0 - (phase - 10) as f64 / 8.0).max(0.0)
        };
        for i in 0..WIDTH {
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct DotPulse {
    state: EffectState,
}

impl DotPulse {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for DotPulse {
    fn name() -> &'static str {
        "dot-pulse"
    }
    fn description() -> &'static str {
        "Expanding ring in dot characters"
    }
    fn cycle_length() -> usize {
        2 * WIDTH
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl DotPulse {
    const CX: f64 = WIDTH as f64 / 2.0;

    fn render(&mut self) -> String {
        let mut f = DotFrame::new(WIDTH);
        let frame_mod = self.state.frame % Self::cycle_length();
        let ring = if frame_mod < Self::cycle_length() / 2 {
            frame_mod
        } else {
            Self::cycle_length() - frame_mod - 1
        };
        for i in 0..WIDTH {
            let dist = (i as f64 - Self::CX + 0.5).abs();
            let v = ((ring as f64 - dist + 1.0) / 3.0).clamp(0.0, 1.0);
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct DotArrow {
    state: EffectState,
}

impl DotArrow {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for DotArrow {
    fn name() -> &'static str {
        "dot-arrow"
    }
    fn description() -> &'static str {
        "Arrow bouncing in dot characters"
    }
    fn cycle_length() -> usize {
        (Self::TRAVEL * 2 + pause::MEDIUM as isize) as usize
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl DotArrow {
    const SPEED: isize = 1;
    const LENGTH: isize = 5;
    const TRAVEL: isize = WIDTH as isize + 2 * (Self::LENGTH - 1);

    fn render(&mut self) -> String {
        let mut f = DotFrame::new(WIDTH);
        let (head, direction): (isize, isize) = if self.state.frame < Self::TRAVEL as usize {
            (-(Self::LENGTH - 1) + self.state.frame as isize * Self::SPEED, 1)
        } else if self.state.frame < Self::TRAVEL as usize * 2 {
            let f2 = self.state.frame as isize - Self::TRAVEL;
            ((WIDTH - 1) as isize + (Self::LENGTH - 1) - f2 * Self::SPEED, -1)
        } else {
            return f.render().join("\n");
        };
        for t in 0..Self::LENGTH {
            let pos = head - direction * t;
            if pos >= 0 && (pos as usize) < WIDTH {
                let v = (Self::LENGTH - t) as f64 / Self::LENGTH as f64;
                f.set(pos as usize, v);
            }
        }
        f.render().join("\n")
    }
}

pub struct DotBounce {
    state: EffectState,
    phases: Vec<f64>,
    speeds: Vec<f64>,
}

impl DotBounce {
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

impl Effect for DotBounce {
    fn name() -> &'static str {
        "dot-bounce"
    }
    fn description() -> &'static str {
        "Bouncing dots with random phases"
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

impl DotBounce {
    fn render(&mut self) -> String {
        let mut f = DotFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = (self.phases[i] + self.state.frame as f64 * self.speeds[i]).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}
