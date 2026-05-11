use std::f64::consts::TAU;

use crate::effect::{
    Effect, EffectState, HEIGHT, WIDTH, cycle_length, pause, spatial_frequency, temporal_speed,
    toggle_rate, trail
};
use crate::frame::BrailleFrame;
use rand::{RngExt};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub struct BrailleSpin {
    state: EffectState,
}

impl BrailleSpin {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleSpin {
    fn name() -> &'static str {
        "braille-spin"
    }
    fn description() -> &'static str {
        "Braille spinner, same char repeated"
    }
    fn cycle_length() -> usize {
        8
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleSpin {
    const PATH: [(usize, usize); 8] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 3),
        (1, 2),
        (1, 1),
        (1, 0),
    ];

    fn render(&mut self) -> String {
        let t = trail::SHORT;
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for cx in (0..WIDTH * 2).step_by(4) {
            for i in 0..t {
                let (dx, dy) = Self::PATH[(self.state.frame + i) % Self::PATH.len()];
                f.set(cx + dx, dy);
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleSpin2 {
    state: EffectState,
}

impl BrailleSpin2 {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleSpin2 {
    fn name() -> &'static str {
        "braille-spin2"
    }
    fn description() -> &'static str {
        "Larger braille spinner with wider path"
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

impl BrailleSpin2 {
    const PATH: [(usize, usize); 16] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
        (4, 3),
        (5, 3),
        (5, 2),
        (5, 1),
        (5, 0),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0),
    ];
    fn render(&mut self) -> String {
        let t = trail::SHORT;
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for cx in (0..WIDTH * 2 - 4).step_by(4) {
            for i in 0..t {
                let (dx, dy) = Self::PATH[(self.state.frame + i) % Self::PATH.len()];
                f.set(cx + dx, dy);
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleWave {
    state: EffectState,
}

impl BrailleWave {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleWave {
    fn name() -> &'static str {
        "braille-wave"
    }
    fn description() -> &'static str {
        "Sine wave across braille cells"
    }
    fn cycle_length() -> usize {
        8
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleWave {
    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for i in 0..WIDTH {
            let phase = (self.state.frame + i) % 8;
            let y = if phase < 4 { phase } else { 7 - phase };
            f.set(i * 2, y);
        }
        f.render().join("\n")
    }
}

pub struct BrailleRandom {
    state: EffectState,
}

impl BrailleRandom {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleRandom {
    fn name() -> &'static str {
        "braille-random"
    }
    fn description() -> &'static str {
        "Random braille noise"
    }
    fn cycle_length() -> usize {
        36
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleRandom {
    const PIXEL_ORDER: [(usize, usize); 8] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (0, 3),
        (1, 3),
    ];

    fn render(&mut self) -> String {
        let mut rng = StdRng::seed_from_u64(self.state.frame as u64);
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for i in 0..WIDTH {
            let val = rng.random::<u8>();
            for b in 0..8u8 {
                if val & (1 << b) != 0 {
                    let (px, py) = Self::PIXEL_ORDER[b as usize];
                    f.set(i * 2 + px, py);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleBreathe {
    state: EffectState,
}

impl BrailleBreathe {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleBreathe {
    fn name() -> &'static str {
        "braille-breathe"
    }
    fn description() -> &'static str {
        "Breathing braille expansion"
    }
    fn cycle_length() -> usize {
        9
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleBreathe {
    const PIXEL_ORDER: [(usize, usize); 8] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (0, 3),
        (1, 3),
    ];

    fn render(&mut self) -> String {
        let cl = cycle_length::SHORT;
        let phase = self.state.frame % cl;
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for i in 0..WIDTH {
            let dist = (i as isize - WIDTH as isize / 2).unsigned_abs();
            let ci = (phase + dist) % cl;
            let count = if ci < 5 { ci } else { cl - ci };
            let n = count * 8 / 5;
            for b in 0..n {
                let (px, py) = Self::PIXEL_ORDER[b];
                f.set(i * 2 + px, py);
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleRipple {
    state: EffectState,
}

impl BrailleRipple {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleRipple {
    fn name() -> &'static str {
        "braille-ripple"
    }
    fn description() -> &'static str {
        "Ripple wave in braille dots"
    }
    fn cycle_length() -> usize {
        9
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleRipple {
    const CENTER_X: f64 = WIDTH as f64;
    const CENTER_Y: f64 = 1.5;

    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for x in 0..(2 * WIDTH) {
            for y in 0..4usize {
                let d = ((x as f64 - Self::CENTER_X).powi(2) + (y as f64 - Self::CENTER_Y).powi(2)).sqrt();
                let wave = (d * 2.0 - self.state.frame as f64 * temporal_speed::FAST).sin();
                if wave > 0.3 {
                    f.set(x, y);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleBounce {
    state: EffectState,
}

impl BrailleBounce {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleBounce {
    fn name() -> &'static str {
        "braille-bounce"
    }
    fn description() -> &'static str {
        "Bouncing center with random dots"
    }
    fn cycle_length() -> usize {
        36
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleBounce {
    fn render(&mut self) -> String {
        let mut rng = StdRng::seed_from_u64(42);
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        let t = self.state.frame % (WIDTH * 4);
        let center = if t < WIDTH * 2 { t } else { WIDTH * 4 - t };
        for x in 0..(2 * WIDTH) {
            let dist = (x as isize - center as isize).unsigned_abs() as f64;
            let p = (1.1 - dist / 8.0).max(0.0);
            for y in 0..4usize {
                if rng.random::<f64>() < p {
                    f.set(x, y);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleRain {
    state: EffectState,
}

impl BrailleRain {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleRain {
    fn name() -> &'static str {
        "braille-rain"
    }
    fn description() -> &'static str {
        "Falling rain drops in braille"
    }
    fn cycle_length() -> usize {
        7
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleRain {
    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for i in 0..WIDTH {
            let speed = 1 + (i * 3 + 1) % 3;
            let offset = (i * 7) % 8;
            let drop_y = (self.state.frame * speed + offset) % 7;
            let drop_y = drop_y as isize - 1;
            for t in 0..3isize {
                let y = drop_y - t;
                if y >= 0 && y < 4 {
                    f.set(i * 2 + (t as usize % 2), y as usize);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleZigzag {
    state: EffectState,
}

impl BrailleZigzag {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleZigzag {
    fn name() -> &'static str {
        "braille-zigzag"
    }
    fn description() -> &'static str {
        "Zigzag pattern in braille"
    }
    fn cycle_length() -> usize {
        6
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleZigzag {
    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for x in 0..(2 * WIDTH) {
            let phase = (x + self.state.frame) % 6;
            let y = if phase < 4 { phase } else { 6 - phase };
            f.set(x, y);
        }
        f.render().join("\n")
    }
}

pub struct BrailleDissolve {
    state: EffectState,
}

impl BrailleDissolve {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleDissolve {
    fn name() -> &'static str {
        "braille-dissolve"
    }
    fn description() -> &'static str {
        "Dissolving and rebuilding braille"
    }
    fn cycle_length() -> usize {
        cycle_length::LONG
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleDissolve {
    const TOTAL_PIXELS: usize = 2 * WIDTH * 4;
    const HALF_CYCLE: usize = cycle_length::LONG / 2;

    fn render(&mut self) -> String {
        let mut rng = StdRng::seed_from_u64(42);
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        let cycle = cycle_length::LONG;
        let phase = self.state.frame % cycle;
        let mut order: Vec<usize> = (0..Self::TOTAL_PIXELS).collect();
        order.shuffle(&mut rng);
        let remaining = if phase < Self::HALF_CYCLE {
            Self::TOTAL_PIXELS.saturating_sub(phase * Self::TOTAL_PIXELS / Self::HALF_CYCLE)
        } else {
            (phase - Self::HALF_CYCLE) * Self::TOTAL_PIXELS / Self::HALF_CYCLE
        };
        let remaining = remaining.min(Self::TOTAL_PIXELS);
        for &idx in &order[..remaining] {
            let x = idx % (2 * WIDTH);
            let y = idx / (2 * WIDTH);
            f.set(x, y);
        }
        f.render().join("\n")
    }
}

pub struct BrailleFire {
    state: EffectState,
}

impl BrailleFire {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleFire {
    fn name() -> &'static str {
        "braille-fire"
    }
    fn description() -> &'static str {
        "Fire effect using braille dots"
    }
    fn cycle_length() -> usize {
        9
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleFire {
    fn render(&mut self) -> String {
        let mut rng = StdRng::seed_from_u64(self.state.frame as u64);
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for x in 0..(2 * WIDTH) {
            for y in 0..4usize {
                let row_from_bottom = 3 - y;
                let decay = row_from_bottom as f64 * 0.28;
                let fast = (self.state.frame as f64 * temporal_speed::FAST
                    + x as f64 * spatial_frequency::HIGH)
                    .sin()
                    * 0.2;
                let heat = (1.0 - decay + fast).max(0.0);
                if rng.random::<f64>() < heat {
                    f.set(x, y);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleNoise {
    state: EffectState,
}

impl BrailleNoise {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleNoise {
    fn name() -> &'static str {
        "braille-noise"
    }
    fn description() -> &'static str {
        "Smooth noise in braille dots"
    }
    fn cycle_length() -> usize {
        36
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleNoise {
    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        let phase = TAU * self.state.frame as f64 / Self::cycle_length() as f64;
        for x in 0..(2 * WIDTH) {
            for y in 0..4usize {
                let nx = x as f64 * spatial_frequency::LOW;
                let ny = y as f64 * 0.5;
                let v = (nx * 2.0 + ny * 2.0 + phase * 2.0).sin() * 0.5
                    + (nx - ny - phase).sin() * 0.3
                    + (nx * 2.0 + ny + phase * 3.0).sin() * 0.2;
                if v > 0.1 {
                    f.set(x, y);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleHeartbeat {
    state: EffectState,
}

impl BrailleHeartbeat {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleHeartbeat {
    fn name() -> &'static str {
        "braille-heartbeat"
    }
    fn description() -> &'static str {
        "Heartbeat pulse in braille"
    }
    fn cycle_length() -> usize {
        36
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleHeartbeat {
    fn render(&mut self) -> String {
        const PATTERNS: [[usize; 18]; 2] = [
            [0, 0, 0, 0, 1, 2, 1, 0, 2, 4, 3, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 2, 2, 3, 1, 0, 0, 0],
        ];
        const LEN: isize = cycle_length::MEDIUM as isize;
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for x in 0..(2 * WIDTH) {
            let gi: isize = self.state.frame as isize - x as isize;
            let beat = gi.div_euclid(LEN).rem_euclid(PATTERNS.len() as isize) as usize;
            let pat = &PATTERNS[beat];
            let i = gi.rem_euclid(LEN) as usize;
            let y_off = pat[i];
            let cy: isize = 2;
            for dy in -(y_off as isize)..=(y_off as isize) {
                let ty = cy + dy;
                if ty >= 0 && ty < 4 {
                    f.set(x, ty as usize);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleScanner {
    state: EffectState,
}

impl BrailleScanner {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleScanner {
    fn name() -> &'static str {
        "braille-scanner"
    }
    fn description() -> &'static str {
        "Scanning beam across braille"
    }
    fn cycle_length() -> usize {
        cycle_length::MEDIUM + trail::EXTENDED + 4
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleScanner {
    fn render(&mut self) -> String {
        let mut rng = StdRng::seed_from_u64(self.state.frame as u64);
        let t = trail::EXTENDED;
        let pos = (self.state.frame % Self::cycle_length()) as isize;
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for y in 0..4usize {
            for dx in 0..=t {
                let x = pos - dx as isize;
                if x >= 0 && (x as usize) < 2 * WIDTH {
                    if dx == 0 {
                        f.set(x as usize, y);
                    } else if rng.random::<f64>() < 1.0 - dx as f64 / t as f64 {
                        f.set(x as usize, y);
                    }
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleMatrix {
    state: EffectState,
    dot_init: Vec<(i64, i64)>,
}

impl BrailleMatrix {
    const TRAVEL_START: i64 = -6;
    const TRAVEL_END: i64 = 10;
    pub fn new() -> Self {
        let mut state = EffectState::new(42, Self::cycle_length());
        let dot_init = (0..=WIDTH*2).map(|_| 
            ([1i64, 1, 2][state.rng.random_range(0..3usize)], state.rng.random_range(BrailleMatrix::TRAVEL_START..BrailleMatrix::TRAVEL_END ))
        ).collect();

        Self {
            state: state,
            dot_init: dot_init,
        }
    }
}

impl Effect for BrailleMatrix {
    fn name() -> &'static str {
        "braille-matrix"
    }
    fn description() -> &'static str {
        "Matrix-style falling columns"
    }
    fn cycle_length() -> usize {
        (BrailleMatrix::TRAVEL_END - BrailleMatrix::TRAVEL_START) as usize
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleMatrix {
    fn render(&mut self) -> String {
        let tail = trail::SHORT;
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        for (i, (speed, init_offset)) in self.dot_init.iter().enumerate() {
            // let span: i64 = 4 + tail as i64 + init_offset.abs();
            let steps: i64 = self.state.frame as i64 * speed;
            let head: i64 = init_offset + steps;
            for t in 0..tail as i64 {
                let y: i64 = (head - t - BrailleMatrix::TRAVEL_START) % (BrailleMatrix::TRAVEL_END - BrailleMatrix::TRAVEL_START) + BrailleMatrix::TRAVEL_START;
                if y >= 0 && y < 4 {
                    f.set(i, y as usize);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleCheckerboard {
    state: EffectState,
}

impl BrailleCheckerboard {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleCheckerboard {
    fn name() -> &'static str {
        "braille-checkerboard"
    }
    fn description() -> &'static str {
        "Alternating checkerboard pattern"
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

impl BrailleCheckerboard {
    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        let offset = (self.state.frame / toggle_rate::SLOW) % 2;
        for x in 0..(2 * WIDTH) {
            for y in 0..4usize {
                if (x + y + offset) % 2 == 0 {
                    f.set(x, y);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleCheckerboard2x2 {
    state: EffectState,
}

impl BrailleCheckerboard2x2 {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleCheckerboard2x2 {
    fn name() -> &'static str {
        "braille-checkerboard-2x2"
    }
    fn description() -> &'static str {
        "2x2 block checkerboard pattern"
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

impl BrailleCheckerboard2x2 {
    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        let offset = (self.state.frame / toggle_rate::SLOW) % 2;
        for x in 0..(2 * WIDTH) {
            for y in 0..4usize {
                if (x / 2 + y / 2 + offset) % 2 == 0 {
                    f.set(x, y);
                }
            }
        }
        f.render().join("\n")
    }
}

pub struct BrailleArrow {
    state: EffectState,
}

impl BrailleArrow {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for BrailleArrow {
    fn name() -> &'static str {
        "braille-arrow"
    }
    fn description() -> &'static str {
        "Arrow bouncing across braille display"
    }
    fn cycle_length() -> usize {
        2 * Self::TRAVEL + pause::LONG
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl BrailleArrow {
    const RIGHT_ARROW : [[u8; 2]; 24] = [
        [0, 0], [1, 0], [2, 0],                [5, 0], [6, 0], [7, 0],
            [1, 1], [2, 1], [3, 1],                [6, 1], [7, 1], [8, 1],
            [1, 2], [2, 2], [3, 2],                [6, 2], [7, 2], [8, 2],
        [0, 3], [1, 3], [2, 3],                [5, 3], [6, 3], [7, 3],
    ];

    const LEFT_ARROW : [[u8; 2]; 24] = [
            [1, 0], [2, 0], [3, 0],                [6, 0], [7, 0], [8, 0],
        [0, 1], [1, 1], [2, 1],                [5, 1], [6, 1], [7, 1],
        [0, 2], [1, 2], [2, 2],                [5, 2], [6, 2], [7, 2],
            [1, 3], [2, 3], [3, 3],                [6, 3], [7, 3], [8, 3],
    ];

    const SPEED: usize = 2;
    const ARROW_WIDTH: usize = 8;
    const TRAVEL: usize = (2 * WIDTH + Self::ARROW_WIDTH) / Self::SPEED + 1;

    fn render(&mut self) -> String {
        let mut f = BrailleFrame::new(WIDTH, HEIGHT);
        if self.state.frame >= 2 * Self::TRAVEL  {
            return f.render().join("\n");
        }
        let (base, offset) = if self.state.frame < Self::TRAVEL  {
            (
                - (Self::ARROW_WIDTH as isize) + ((self.state.frame * Self::SPEED) as isize),
                Self::RIGHT_ARROW,
            )
        }
        else {
            (
                (2 * WIDTH) as isize - (self.state.frame as isize - Self::TRAVEL as isize) * Self::SPEED as isize,
                Self::LEFT_ARROW,
            )
        };

        for [dx, dy] in offset {
            let x = base + dx as isize;
            if 0 <= x && x < (2 * WIDTH) as isize  {
                f.set(x as usize, dy as usize);
            }
        }

        f.render().join("\n")
    }
}
