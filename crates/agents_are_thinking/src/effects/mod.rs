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
        Box::new(BrailleSpin::new(42)),
        Box::new(BrailleSpin2::new(42)),
        Box::new(BrailleWave::new(42)),
        Box::new(BrailleRandom::new(42)),
        Box::new(BrailleBreathe::new(42)),
        Box::new(BrailleRipple::new(42)),
        Box::new(BrailleBounce::new(42)),
        Box::new(BrailleRain::new(42)),
        Box::new(BrailleZigzag::new(42)),
        Box::new(BrailleDissolve::new(42)),
        Box::new(BrailleFire::new(42)),
        Box::new(BrailleNoise::new(42)),
        Box::new(BrailleHeartbeat::new(42)),
        Box::new(BrailleArrow::new(42)),
        Box::new(BrailleScanner::new(42)),
        Box::new(BrailleMatrix::new(42)),
        Box::new(BrailleCheckerboard::new(42)),
        Box::new(BrailleCheckerboard2x2::new(42)),
        Box::new(ShadeWave::new(42)),
        Box::new(ShadeScanner::new(42)),
        Box::new(ShadeFire::new(42)),
        Box::new(ShadeRipple::new(42)),
        Box::new(ShadeBreathe::new(42)),
        Box::new(ShadeSeeSaw::new(42)),
        Box::new(ShadeBlink::new(42)),
        Box::new(ShadeLayers::new(42)),
        Box::new(ShadePinch::new(42)),
        Box::new(VBlockWave::new(42)),
        Box::new(VBlockFill::new(42)),
        Box::new(VBlockTide::new(42)),
        Box::new(VBlockBreathe::new(42)),
        Box::new(VBlockBounce::new(42)),
        Box::new(VBlockPulse::new(42)),
        Box::new(VBlockRipple::new(42)),
        Box::new(VBlockRain::new(42)),
        Box::new(VBlockCascade::new(42)),
        Box::new(BarBounce::new(42)),
        Box::new(BarWave::new(42)),
        Box::new(BarSeeSaw::new(42)),
        Box::new(SquarePulse::new(42)),
        Box::new(SquareFill::new(42)),
        Box::new(SquareBlink::new(42)),
        Box::new(SquareArrow::new(42)),
        Box::new(DotWave::new(42)),
        Box::new(DotPulse::new(42)),
        Box::new(DotHeartbeat::new(42)),
        Box::new(DotArrow::new(42)),
        Box::new(DotBounce::new(42)),
    ]
}
