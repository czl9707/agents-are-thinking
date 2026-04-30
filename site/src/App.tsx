import { useEffect, useState } from 'react';
import * as Wasm from 'agents-are-thinking-js';

const EFFECTS = [
  Wasm.BarBounce, Wasm.BarSeeSaw, Wasm.BarWave,
  Wasm.BrailleArrow, Wasm.BrailleBounce, Wasm.BrailleBreathe, Wasm.BrailleCheckerboard, Wasm.BrailleCheckerboard2x2, Wasm.BrailleDissolve, Wasm.BrailleFire, Wasm.BrailleHeartbeat, Wasm.BrailleMatrix, Wasm.BrailleNoise, Wasm.BrailleRain, Wasm.BrailleRandom, Wasm.BrailleRipple, Wasm.BrailleScanner, Wasm.BrailleSpin, Wasm.BrailleSpin2, Wasm.BrailleWave, Wasm.BrailleZigzag,
  Wasm.DotArrow, Wasm.DotBounce, Wasm.DotHeartbeat, Wasm.DotPulse, Wasm.DotWave,
  Wasm.ShadeBlink, Wasm.ShadeBreathe, Wasm.ShadeFire, Wasm.ShadeLayers, Wasm.ShadePinch, Wasm.ShadeRipple, Wasm.ShadeScanner, Wasm.ShadeSeeSaw, Wasm.ShadeWave,
  Wasm.SquareArrow, Wasm.SquareBlink, Wasm.SquareFill, Wasm.SquarePulse,
  Wasm.VBlockBounce, Wasm.VBlockBreathe, Wasm.VBlockCascade, Wasm.VBlockPulse, Wasm.VBlockRain, Wasm.VBlockRipple, Wasm.VBlockScanner, Wasm.VBlockTide, Wasm.VBlockWave,
];

function App() {
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    if (EFFECTS.length > 0) setLoaded(true);
  }, []);

  return (
    <div>
      {loaded ? `Loaded ${EFFECTS.length} effects` : 'Loading...'}
    </div>
  );
}

export default App;
