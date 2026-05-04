## Install

```bash
npm install @zane-chen/agents-are-thinking
```

Requires a bundler (Vite, Webpack 5, Rollup, etc.) or Node.js with WASM support.




## With React

```tsx
import { useEffect, useRef, useState } from 'react'
import { ShadeFire } from '@zane-chen/agents-are-thinking'

function EffectDisplay() {
  const [frame, setFrame] = useState('')

  useEffect(() => {
    const ef = new ShadeFire()
    let last = 0
    let raf
    const tick = (t) => {
      if (t - last >= 100) { setFrame(ef.step()); last = t }
      raf = requestAnimationFrame(tick)
    }
    raf = requestAnimationFrame(tick)
    return () => {
      cancelAnimationFrame(raf)
      ef.free()
    }
  }, [])

  return (
    <span style={{
      fontFamily: 'monospace',
      display: 'inline-block',
      minWidth: '9ch',
    }}>
      {frame}
    </span>
  )
}
```

## Font selection

Effects render fixed-width text (9 characters). Use a monospace font — but not all monospace fonts handle every glyph set equally:

| Family | Characters | Recommended font |
|--------|-----------|-----------------|
| **dot** | `· ∘ • ○ ●` | Fira Code (most others misalign these) |
| **vblock** | `▏ ▎ ▍ ▌ ▋ ▊ ▉ █` | Cascadia Code (Fira Code does not render these correctly) |
| **bar** | `▁ ▂ ▃ ▄ ▅ ▆ ▇ █` | Either works |
| **shade** | `░ ▒ ▓ █` | Either works |
| **square** | `· □ ■` | Either works |
| **braille** | `⠁ ⠃ ⠇ …` | Either works |

No single font covers all families perfectly (per my research). For mixed usage, Cascadia Code is the best default — only the dot family needs Fira Code instead.

## List all effects

```ts
import { EFFECTS } from '@zane-chen/agents-are-thinking'

for (const Cls of EFFECTS) {
  console.log(Cls.name, '-', Cls.description)
}
```

## API

Each effect class:

| Member | Description |
|--------|-------------|
| `new Cls()` | Create an instance |
| `instance.step()` | Returns the next frame string |
| `instance.free()` | Release WASM memory |
| `Cls.name()` | Static — effect name |
| `Cls.description()` | Static — one-line description |
| `Cls.cycleLength()` | Static — frames before repeat |
| `WIDTH` | Constant: frame width in characters (9) |
| `EFFECTS` | Array of all 48 effect classes |
