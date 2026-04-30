use wasm_bindgen::prelude::*;

use agents_are_thinking::effect::Effect;

#[wasm_bindgen(js_name = "WIDTH")]
pub fn width() -> usize {
    agents_are_thinking::effect::WIDTH
}

macro_rules! js_effect {
    ($cls:ident) => {
        #[wasm_bindgen]
        pub struct $cls {
            inner: Box<dyn Effect>,
        }

        #[wasm_bindgen]
        impl $cls {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self {
                    inner: Box::new(agents_are_thinking::effects::$cls::new()),
                }
            }

            pub fn step(&mut self) -> String {
                self.inner.step()
            }

            #[wasm_bindgen(js_name = "name")]
            pub fn name() -> String {
                agents_are_thinking::effects::$cls::name().to_string()
            }

            #[wasm_bindgen(js_name = "description")]
            pub fn description() -> String {
                agents_are_thinking::effects::$cls::description().to_string()
            }

            #[wasm_bindgen(js_name = "cycleLength")]
            pub fn cycle_length() -> usize {
                agents_are_thinking::effects::$cls::cycle_length()
            }
        }
    };
}

js_effect!(BrailleSpin);
js_effect!(BrailleSpin2);
js_effect!(BrailleWave);
js_effect!(BrailleRandom);
js_effect!(BrailleBreathe);
js_effect!(BrailleRipple);
js_effect!(BrailleBounce);
js_effect!(BrailleRain);
js_effect!(BrailleZigzag);
js_effect!(BrailleDissolve);
js_effect!(BrailleFire);
js_effect!(BrailleNoise);
js_effect!(BrailleHeartbeat);
js_effect!(BrailleArrow);
js_effect!(BrailleScanner);
js_effect!(BrailleMatrix);
js_effect!(BrailleCheckerboard);
js_effect!(BrailleCheckerboard2x2);

js_effect!(ShadeWave);
js_effect!(ShadeScanner);
js_effect!(ShadeFire);
js_effect!(ShadeRipple);
js_effect!(ShadeBreathe);
js_effect!(ShadeSeeSaw);
js_effect!(ShadeBlink);
js_effect!(ShadeLayers);
js_effect!(ShadePinch);

js_effect!(BarBounce);
js_effect!(BarWave);
js_effect!(BarSeeSaw);

js_effect!(VBlockWave);
js_effect!(VBlockScanner);
js_effect!(VBlockTide);
js_effect!(VBlockBreathe);
js_effect!(VBlockBounce);
js_effect!(VBlockPulse);
js_effect!(VBlockRipple);
js_effect!(VBlockRain);
js_effect!(VBlockCascade);

js_effect!(SquarePulse);
js_effect!(SquareFill);
js_effect!(SquareBlink);
js_effect!(SquareArrow);

js_effect!(DotWave);
js_effect!(DotPulse);
js_effect!(DotHeartbeat);
js_effect!(DotArrow);
js_effect!(DotBounce);
