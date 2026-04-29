use rand::rngs::StdRng;
use rand::SeedableRng;

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 1;

pub mod temporal_speed {
    pub const CRAWL: f64 = 0.05;
    pub const SLOW: f64 = 0.12;
    pub const GENTLE: f64 = 0.25;
    pub const MODERATE: f64 = 0.4;
    pub const FAST: f64 = 0.7;
    pub const INTENSE: f64 = 1.3;
}

pub mod spatial_frequency {
    pub const LOW: f64 = 0.6;
    pub const HIGH: f64 = 1.2;
    pub const DENSE: f64 = 2.1;
    pub const EXTRA_DENSE: f64 = 6.0;
}

pub mod cycle_length {
    pub const SHORT: usize = 9;
    pub const MEDIUM: usize = 18;
    pub const LONG: usize = 36;
}

pub mod pause {
    pub const SHORT: usize = 8;
    pub const MEDIUM: usize = 12;
    pub const LONG: usize = 16;
}

pub mod trail {
    pub const SHORT: usize = 3;
    pub const LONG: usize = 5;
    pub const EXTENDED: usize = 8;
}

pub mod toggle_rate {
    pub const SLOW: usize = 8;
}

pub fn new_rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

pub trait Effect: Send + Sync {
    fn name() -> &'static str where Self: Sized;
    fn description() -> &'static str where Self: Sized;
    fn cycle_length() -> usize where Self: Sized;
    fn step(&mut self) -> String;
}

pub struct EffectState {
    pub frame: usize,
    pub rng: StdRng,
    cycle: usize,
}

impl EffectState {
    pub fn new(seed: u64, cycle: usize) -> Self {
        Self {
            frame: 0,
            rng: new_rng(seed),
            cycle: cycle
        }
    }

    pub fn advance(&mut self) {
        self.frame = (self.frame + 1) % self.cycle;
    }
}
