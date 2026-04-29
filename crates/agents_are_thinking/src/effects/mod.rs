mod bar;
mod braille;
mod dot;
mod shade;
mod square;
mod vblock;

pub use bar::*;
pub use braille::*;
pub use dot::*;
pub use shade::*;
pub use square::*;
pub use vblock::*;

use crate::effect::Effect;

pub fn all_effects() -> Vec<Box<dyn Effect>> {
    vec![
        Box::new(BrailleSpin::new()),
        Box::new(BrailleSpin2::new()),
        Box::new(BrailleWave::new()),
        Box::new(BrailleRandom::new()),
        Box::new(BrailleBreathe::new()),
        Box::new(BrailleRipple::new()),
        Box::new(BrailleBounce::new()),
        Box::new(BrailleRain::new()),
        Box::new(BrailleZigzag::new()),
        Box::new(BrailleDissolve::new()),
        Box::new(BrailleFire::new()),
        Box::new(BrailleNoise::new()),
        Box::new(BrailleHeartbeat::new()),
        Box::new(BrailleArrow::new()),
        Box::new(BrailleScanner::new()),
        Box::new(BrailleMatrix::new()),
        Box::new(BrailleCheckerboard::new()),
        Box::new(BrailleCheckerboard2x2::new()),
        Box::new(ShadeWave::new()),
        Box::new(ShadeScanner::new()),
        Box::new(ShadeFire::new()),
        Box::new(ShadeRipple::new()),
        Box::new(ShadeBreathe::new()),
        Box::new(ShadeSeeSaw::new()),
        Box::new(ShadeBlink::new()),
        Box::new(ShadeLayers::new()),
        Box::new(ShadePinch::new()),
        Box::new(VBlockWave::new()),
        Box::new(VBlockScanner::new()),
        Box::new(VBlockTide::new()),
        Box::new(VBlockBreathe::new()),
        Box::new(VBlockBounce::new()),
        Box::new(VBlockPulse::new()),
        Box::new(VBlockRipple::new()),
        Box::new(VBlockRain::new()),
        Box::new(VBlockCascade::new()),
        Box::new(BarBounce::new()),
        Box::new(BarWave::new()),
        Box::new(BarSeeSaw::new()),
        Box::new(SquarePulse::new()),
        Box::new(SquareFill::new()),
        Box::new(SquareBlink::new()),
        Box::new(SquareArrow::new()),
        Box::new(DotWave::new()),
        Box::new(DotPulse::new()),
        Box::new(DotHeartbeat::new()),
        Box::new(DotArrow::new()),
        Box::new(DotBounce::new()),
    ]
}
