use std::f64::consts::{PI, TAU};

use crate::effect::{Effect, EffectState, WIDTH, cycle_length, spatial_frequency, temporal_speed};
use crate::frame::ShadeFrame;
use rand::Rng;

pub struct ShadeWave {
    state: EffectState,
}

impl ShadeWave {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeWave {
    fn name(&self) -> &'static str {
        "shade-wave"
    }
    fn description(&self) -> &'static str {
        "Sine wave using shade characters"
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

impl ShadeWave {
    fn render(&mut self, frame: usize) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = ((i as f64 + frame as f64) * spatial_frequency::LOW).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct ShadeScanner {
    state: EffectState,
}

impl ShadeScanner {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeScanner {
    fn name(&self) -> &'static str {
        "shade-scanner"
    }
    fn description(&self) -> &'static str {
        "Scanning highlight in shade"
    }
    fn cycle_length(&self) -> usize {
        15
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl ShadeScanner {
    fn render(&mut self, frame: usize) -> String {
        let scan_range = WIDTH + 6;
        let mut f = ShadeFrame::new(WIDTH);
        let pos = frame % scan_range;
        for i in 0..WIDTH {
            let dist = (i as isize - pos as isize).unsigned_abs() as f64;
            let density = (1.0 - dist / 4.0).max(0.0);
            f.set(i, density);
        }
        f.render().join("\n")
    }
}

pub struct ShadeFire {
    state: EffectState,
}

impl ShadeFire {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeFire {
    fn name(&self) -> &'static str {
        "shade-fire"
    }
    fn description(&self) -> &'static str {
        "Fire effect using shade characters"
    }
    fn cycle_length(&self) -> usize {
        20
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl ShadeFire {
    fn render(&mut self, frame: usize) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for x in 0..WIDTH {
            let v = (frame as f64 * temporal_speed::FAST + x as f64 * spatial_frequency::DENSE)
                .sin()
                * 0.3
                + (frame as f64 * temporal_speed::INTENSE + x as f64 * spatial_frequency::HIGH)
                    .sin()
                    * 0.2
                + 0.5;
            f.set(x, v.clamp(0.0, 1.0));
        }
        f.render().join("\n")
    }
}

pub struct ShadeRipple {
    state: EffectState,
}

impl ShadeRipple {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeRipple {
    fn name(&self) -> &'static str {
        "shade-ripple"
    }
    fn description(&self) -> &'static str {
        "Ripple from center in shade"
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

impl ShadeRipple {
    fn render(&mut self, frame: usize) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        let cx = (WIDTH - 1) as f64 / 2.0;
        for i in 0..WIDTH {
            let dist = (i as f64 - cx) / cx;
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

pub struct ShadeBreathe {
    state: EffectState,
}

impl ShadeBreathe {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeBreathe {
    fn name(&self) -> &'static str {
        "shade-breathe"
    }
    fn description(&self) -> &'static str {
        "Breathing shade animation"
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

impl ShadeBreathe {
    fn render(&mut self, frame: usize) -> String {
        let phase = frame % cycle_length::MEDIUM;
        let v = (phase as f64 * PI / 10.0).sin() / 2.0 + 0.5;
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct ShadeSeeSaw {
    state: EffectState,
}

impl ShadeSeeSaw {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeSeeSaw {
    fn name(&self) -> &'static str {
        "shade-seesaw"
    }
    fn description(&self) -> &'static str {
        "See-saw gradient in shade"
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

impl ShadeSeeSaw {
    fn render(&mut self, frame: usize) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        let phase = (frame as f64 * temporal_speed::MODERATE).sin() / 2.0 + 0.5;
        for i in 0..WIDTH {
            let t = i as f64 / (WIDTH - 1) as f64;
            let raw = (t * PI).cos() / 2.0 + 0.5;
            let blend = 0.5 + (raw - 0.5) * 2.0;
            let density = phase * blend + (1.0 - phase) * (1.0 - blend);
            f.set(i, density);
        }
        f.render().join("\n")
    }
}

pub struct ShadeBlink {
    state: EffectState,
    tiers: Vec<usize>,
}

impl ShadeBlink {
    pub fn new(seed: u64) -> Self {
        let mut state = EffectState::new(seed);
        let tiers: Vec<usize> = (0..WIDTH)
            .map(|_| state.rng.random_range(0..=2usize))
            .collect();
        Self { state, tiers }
    }
}

impl Effect for ShadeBlink {
    fn name(&self) -> &'static str {
        "shade-blink"
    }
    fn description(&self) -> &'static str {
        "Tiered blinking shade"
    }
    fn cycle_length(&self) -> usize {
        20
    }

    fn step(&mut self) -> String {
        let frame_idx = self.state.frame;
        let result = self.render(frame_idx);
        self.state.advance(self.cycle_length());
        result
    }
}

impl ShadeBlink {
    fn render(&mut self, frame: usize) -> String {
        let speed = temporal_speed::CRAWL;
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let offset = self.tiers[i] as f64 / 3.0;
            let v = ((frame as f64 * speed + offset) * TAU).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct ShadeLayers {
    state: EffectState,
}

impl ShadeLayers {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadeLayers {
    fn name(&self) -> &'static str {
        "shade-layers"
    }
    fn description(&self) -> &'static str {
        "Layered wave patterns in shade"
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

impl ShadeLayers {
    fn render(&mut self, frame: usize) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let w1 = (frame as f64 * temporal_speed::MODERATE + i as f64 * spatial_frequency::HIGH)
                .sin()
                / 2.0
                + 0.5;
            let w2 = (frame as f64 * temporal_speed::FAST + i as f64 * spatial_frequency::HIGH)
                .sin()
                / 2.0
                + 0.5;
            f.add(i, w1 * 0.5);
            f.add(i, w2 * 0.5);
        }
        f.render().join("\n")
    }
}

pub struct ShadePinch {
    state: EffectState,
}

impl ShadePinch {
    pub fn new(seed: u64) -> Self {
        Self {
            state: EffectState::new(seed),
        }
    }
}

impl Effect for ShadePinch {
    fn name(&self) -> &'static str {
        "shade-pinch"
    }
    fn description(&self) -> &'static str {
        "Pinching gradient in shade"
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

impl ShadePinch {
    fn render(&mut self, frame: usize) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        let cx = (WIDTH - 1) as f64 / 2.0;
        let phase = (frame as f64 * temporal_speed::GENTLE).sin() / 2.0 + 0.5;
        for i in 0..WIDTH {
            let edge_dist = (i as f64 - cx).abs() / cx;
            let side_phase = if i as f64 <= cx { phase } else { 1.0 - phase };
            let density = edge_dist * (0.3 + 0.7 * side_phase);
            f.set(i, density);
        }
        f.render().join("\n")
    }
}
