use std::f64::consts::TAU;

use crate::effect::{Effect, EffectState, WIDTH, cycle_length, pause, spatial_frequency, temporal_speed};
use crate::frame::DotFrame;
use rand::Rng;

pub struct DotWave {
    state: EffectState,
}

impl DotWave {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for DotWave {
    fn name(&self) -> &'static str {
        "dot-wave"
    }
    fn description(&self) -> &'static str {
        "Sine wave using dot characters"
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

impl DotWave {
    fn render(&mut self, frame: usize) -> String {
        let mut f = DotFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = ((i as f64 + frame as f64) * spatial_frequency::LOW).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct DotHeartbeat {
    state: EffectState,
}

impl DotHeartbeat {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for DotHeartbeat {
    fn name(&self) -> &'static str {
        "dot-heartbeat"
    }
    fn description(&self) -> &'static str {
        "Heartbeat pulse in dots"
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

impl DotHeartbeat {
    fn render(&mut self, frame: usize) -> String {
        let mut f = DotFrame::new(WIDTH);
        let phase = frame % cycle_length::MEDIUM;
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
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for DotPulse {
    fn name(&self) -> &'static str {
        "dot-pulse"
    }
    fn description(&self) -> &'static str {
        "Expanding ring in dot characters"
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

impl DotPulse {
    fn render(&mut self, frame: usize) -> String {
        let full_cycle = WIDTH * 2;
        let mut f = DotFrame::new(WIDTH);
        let cx = WIDTH as f64 / 2.0;
        let frame_mod = frame % full_cycle;
        let ring = if frame_mod < full_cycle / 2 {
            frame_mod
        } else {
            full_cycle - frame_mod - 1
        };
        for i in 0..WIDTH {
            let dist = (i as f64 - cx + 0.5).abs();
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
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for DotArrow {
    fn name(&self) -> &'static str {
        "dot-arrow"
    }
    fn description(&self) -> &'static str {
        "Arrow bouncing in dot characters"
    }
    fn cycle_length(&self) -> usize {
        46
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl DotArrow {
    fn render(&mut self, frame: usize) -> String {
        let speed: isize = 1;
        let paws: usize = pause::MEDIUM;
        let length: isize = 5;
        let travel: isize = WIDTH as isize + 2 * (length - 1);
        let cycle = (travel * 2 + paws as isize) as usize;
        let mut f = DotFrame::new(WIDTH);
        let frame = frame % cycle;
        let (head, direction): (isize, isize) = if frame < travel as usize {
            (-(length - 1) + frame as isize * speed, 1)
        } else if frame < travel as usize * 2 {
            let f2 = frame as isize - travel;
            ((WIDTH - 1) as isize + (length - 1) - f2 * speed, -1)
        } else {
            return f.render().join("\n");
        };
        for t in 0..length {
            let pos = head - direction * t;
            if pos >= 0 && (pos as usize) < WIDTH {
                let v = (length - t) as f64 / length as f64;
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
    pub fn new(seed: u64) -> Self {
        let mut state = EffectState::new(seed);
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
    fn name(&self) -> &'static str {
        "dot-bounce"
    }
    fn description(&self) -> &'static str {
        "Bouncing dots with random phases"
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

impl DotBounce {
    fn render(&mut self, frame: usize) -> String {
        let mut f = DotFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = (self.phases[i] + frame as f64 * self.speeds[i]).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}
