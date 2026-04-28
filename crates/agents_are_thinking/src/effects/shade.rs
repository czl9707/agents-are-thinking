use std::f64::consts::{PI, TAU};

use crate::effect::{Effect, EffectState, WIDTH, cycle_length, spatial_frequency, temporal_speed};
use crate::frame::ShadeFrame;
use rand::Rng;



pub struct ShadeWave {
    state: EffectState,
}

impl ShadeWave {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeWave {
    fn name() -> &'static str {
        "shade-wave"
    }
    fn description() -> &'static str {
        "Sine wave using shade characters"
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

impl ShadeWave {
    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let v = ((i as f64 + self.state.frame as f64) * spatial_frequency::LOW).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct ShadeScanner {
    state: EffectState,
}

impl ShadeScanner {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeScanner {
    fn name() -> &'static str {
        "shade-scanner"
    }
    fn description() -> &'static str {
        "Scanning highlight in shade"
    }
    fn cycle_length() -> usize {
        Self::SCAN_RANGE
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl ShadeScanner {
    const SCAN_RANGE: usize = WIDTH + 6;

    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        let pos = self.state.frame % Self::SCAN_RANGE;
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeFire {
    fn name() -> &'static str {
        "shade-fire"
    }
    fn description() -> &'static str {
        "Fire effect using shade characters"
    }
    fn cycle_length() -> usize {
        20
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl ShadeFire {
    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for x in 0..WIDTH {
            let v = (self.state.frame as f64 * temporal_speed::FAST + x as f64 * spatial_frequency::DENSE)
                .sin()
                * 0.3
                + (self.state.frame as f64 * temporal_speed::INTENSE + x as f64 * spatial_frequency::HIGH)
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeRipple {
    fn name() -> &'static str {
        "shade-ripple"
    }
    fn description() -> &'static str {
        "Ripple from center in shade"
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

impl ShadeRipple {
    const CENTER_X: f64 = (WIDTH - 1) as f64 / 2.0;

    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let dist = (i as f64 - Self::CENTER_X) / Self::CENTER_X;
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

pub struct ShadeBreathe {
    state: EffectState,
}

impl ShadeBreathe {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeBreathe {
    fn name() -> &'static str {
        "shade-breathe"
    }
    fn description() -> &'static str {
        "Breathing shade animation"
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

impl ShadeBreathe {
    fn render(&mut self) -> String {
        let phase = self.state.frame % cycle_length::MEDIUM;
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeSeeSaw {
    fn name() -> &'static str {
        "shade-seesaw"
    }
    fn description() -> &'static str {
        "See-saw gradient in shade"
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

impl ShadeSeeSaw {
    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        let phase = (self.state.frame as f64 * temporal_speed::MODERATE).sin() / 2.0 + 0.5;
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
    offsets: Vec<f64>,
}

impl ShadeBlink {
    pub fn new() -> Self {
        let mut state = EffectState::new(42, Self::cycle_length());
        let offsets: Vec<f64> = (0..WIDTH)
            .map(|_| state.rng.random_range(0..=2usize) as f64 / 3.0)
            .collect();
        Self { state, offsets }
    }
}

impl Effect for ShadeBlink {
    fn name() -> &'static str {
        "shade-blink"
    }
    fn description() -> &'static str {
        "Tiered blinking shade"
    }
    fn cycle_length() -> usize {
        20
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl ShadeBlink {
    fn render(&mut self) -> String {
        let speed = temporal_speed::CRAWL;
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let offset = self.offsets[i];
            let v = ((self.state.frame as f64 * speed + offset) * TAU).sin() / 2.0 + 0.5;
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct ShadeLayers {
    state: EffectState,
}

impl ShadeLayers {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadeLayers {
    fn name() -> &'static str {
        "shade-layers"
    }
    fn description() -> &'static str {
        "Layered wave patterns in shade"
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

impl ShadeLayers {
    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        for i in 0..WIDTH {
            let w1 = (self.state.frame as f64 * temporal_speed::MODERATE + i as f64 * spatial_frequency::HIGH)
                .sin()
                / 2.0
                + 0.5;
            let w2 = (self.state.frame as f64 * temporal_speed::FAST + i as f64 * spatial_frequency::HIGH)
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
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for ShadePinch {
    fn name() -> &'static str {
        "shade-pinch"
    }
    fn description() -> &'static str {
        "Pinching gradient in shade"
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

impl ShadePinch {
    const CENTER_X: f64 = (WIDTH - 1) as f64 / 2.0;

    fn render(&mut self) -> String {
        let mut f = ShadeFrame::new(WIDTH);
        let phase = (self.state.frame as f64 * temporal_speed::GENTLE).sin() / 2.0 + 0.5;
        for i in 0..WIDTH {
            let edge_dist = (i as f64 - Self::CENTER_X).abs() / Self::CENTER_X;
            let side_phase = if i as f64 <= Self::CENTER_X { phase } else { 1.0 - phase };
            let density = edge_dist * (0.3 + 0.7 * side_phase);
            f.set(i, density);
        }
        f.render().join("\n")
    }
}
