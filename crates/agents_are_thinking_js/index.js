import init, {
  BarBounce, BarSeeSaw, BarWave,
  BrailleArrow, BrailleBounce, BrailleBreathe, BrailleCheckerboard,
  BrailleCheckerboard2x2, BrailleDissolve, BrailleFire, BrailleHeartbeat,
  BrailleMatrix, BrailleNoise, BrailleRain, BrailleRandom, BrailleRipple,
  BrailleScanner, BrailleSpin, BrailleSpin2, BrailleWave, BrailleZigzag,
  DotArrow, DotBounce, DotHeartbeat, DotPulse, DotWave,
  ShadeBlink, ShadeBreathe, ShadeFire, ShadeLayers, ShadePinch,
  ShadeRipple, ShadeScanner, ShadeSeeSaw, ShadeWave,
  SquareArrow, SquareBlink, SquareFill, SquarePulse,
  VBlockBounce, VBlockBreathe, VBlockCascade, VBlockPulse, VBlockRain,
  VBlockRipple, VBlockScanner, VBlockTide, VBlockWave,
  WIDTH,
} from "./pkg/agents_are_thinking_js.js";

await init();

export const EFFECTS = [
  BrailleSpin, BrailleSpin2, BrailleWave, BrailleRandom,
  BrailleBreathe, BrailleRipple, BrailleBounce, BrailleRain,
  BrailleZigzag, BrailleDissolve, BrailleFire, BrailleNoise,
  BrailleHeartbeat, BrailleArrow, BrailleScanner, BrailleMatrix,
  BrailleCheckerboard, BrailleCheckerboard2x2,
  ShadeWave, ShadeScanner, ShadeFire, ShadeRipple,
  ShadeBreathe, ShadeSeeSaw, ShadeBlink, ShadeLayers, ShadePinch,
  BarBounce, BarWave, BarSeeSaw,
  VBlockWave, VBlockScanner, VBlockTide, VBlockBreathe,
  VBlockBounce, VBlockPulse, VBlockRipple, VBlockRain, VBlockCascade,
  SquarePulse, SquareFill, SquareBlink, SquareArrow,
  DotWave, DotPulse, DotHeartbeat, DotArrow, DotBounce,
];

export {
  BarBounce, BarSeeSaw, BarWave,
  BrailleArrow, BrailleBounce, BrailleBreathe, BrailleCheckerboard,
  BrailleCheckerboard2x2, BrailleDissolve, BrailleFire, BrailleHeartbeat,
  BrailleMatrix, BrailleNoise, BrailleRain, BrailleRandom, BrailleRipple,
  BrailleScanner, BrailleSpin, BrailleSpin2, BrailleWave, BrailleZigzag,
  DotArrow, DotBounce, DotHeartbeat, DotPulse, DotWave,
  ShadeBlink, ShadeBreathe, ShadeFire, ShadeLayers, ShadePinch,
  ShadeRipple, ShadeScanner, ShadeSeeSaw, ShadeWave,
  SquareArrow, SquareBlink, SquareFill, SquarePulse,
  VBlockBounce, VBlockBreathe, VBlockCascade, VBlockPulse, VBlockRain,
  VBlockRipple, VBlockScanner, VBlockTide, VBlockWave,
  WIDTH,
};
