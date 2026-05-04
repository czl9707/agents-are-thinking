# agents-are-thinking

Terminal animation effects for agents' "thinking" state. Braille, block characters, unicode glyphs. No runtime dependencies.

<img src="./agents-are-thinking-demo.gif" width="100%" height="100%">

## Get Started 

~The package is implemented in python.~ The package is written in rust. And provides python binding using [PY03](https://github.com/PyO3/pyo3), and wasm binding using [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen).

### "Too complicated, just want to watch them thinking!"

```bash
uv tool install agents-are-thinking[cli]
uv tool run agents-are-thinking preview
```

Or visit [agents-are-thinking.kiyo-n-zane.com](https://agents-are-thinking.kiyo-n-zane.com).

### Install

| Ecosystem | Package | Install |
|-----------|---------|---------|
| Rust | `agents-are-thinking` | `cargo add agents-are-thinking` |
| Python | `agents-are-thinking` | `uv add agents-are-thinking` |
| JS/TS | `@zane-chen/agents-are-thinking` | `npm install @zane-chen/agents-are-thinking` |

### Ecosystem guides

- [Rust](./RUST-GUIDE.md)
- [Python](./PYTHON-GUIDE.md)
- [JS/TS (WASM)](./JS-GUIDE.md)

## Effects

48 effects across 6 families: braille, shade, bar, vblock, square, dot.

### Braille

| Effect | Description |
|--------|-------------|
| `BrailleSpin` | Braille spinner, same char repeated |
| `BrailleSpin2` | Braille spinner, wider path |
| `BrailleWave` | Smooth sine wave scrolls across as braille dots |
| `BrailleRandom` | Each column picks random dots |
| `BrailleBreathe` | Gentle breathing pattern |
| `BrailleRipple` | Ripple radiating from center |
| `BrailleBounce` | Dots bouncing across |
| `BrailleRain` | Falling dots |
| `BrailleZigzag` | Zigzag path |
| `BrailleDissolve` | Dissolve and reform |
| `BrailleFire` | Fire simulation |
| `BrailleNoise` | Static noise |
| `BrailleHeartbeat` | Heartbeat pulse |
| `BrailleArrow` | Arrow pattern |
| `BrailleScanner` | Scanner sweep |
| `BrailleMatrix` | Matrix-style rain |
| `BrailleCheckerboard` | Checkerboard toggle |
| `BrailleCheckerboard2x2` | 2x2 checkerboard toggle |

### Shade

| Effect | Description |
|--------|-------------|
| `ShadeWave` | Smooth sine wave as shade blocks |
| `ShadeScanner` | Scanner sweep as shades |
| `ShadeFire` | Fire simulation with shade characters |
| `ShadeRipple` | Ripple radiating from center |
| `ShadeBreathe` | Gentle breathing pattern |
| `ShadeSeeSaw` | Left and right halves alternate |
| `ShadeBlink` | Blinking pattern |
| `ShadeLayers` | Layered shading |
| `ShadePinch` | Pinch effect |

### Bar

| Effect | Description |
|--------|-------------|
| `BarBounce` | Each bar bounces independently |
| `BarWave` | Smooth sine wave scrolls across as bar heights |
| `BarSeeSaw` | Left and right halves alternate like a seesaw |

### VBlock

| Effect | Description |
|--------|-------------|
| `VBlockWave` | Smooth sine wave |
| `VBlockScanner` | Scanner sweep |
| `VBlockTide` | Tide coming in and out |
| `VBlockBreathe` | Gentle breathing pattern |
| `VBlockBounce` | Bounce across |
| `VBlockPulse` | Pulse effect |
| `VBlockRipple` | Ripple from center |
| `VBlockRain` | Falling blocks |
| `VBlockCascade` | Cascade effect |

### Square

| Effect | Description |
|--------|-------------|
| `SquarePulse` | Pulse effect |
| `SquareFill` | Fill and drain |
| `SquareBlink` | Blinking pattern |
| `SquareArrow` | Arrow pattern |

### Dot

| Effect | Description |
|--------|-------------|
| `DotWave` | Smooth sine wave |
| `DotPulse` | Pulse effect |
| `DotHeartbeat` | Heartbeat pulse |
| `DotArrow` | Arrow pattern |
| `DotBounce` | Bounce across |

