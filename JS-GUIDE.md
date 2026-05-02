## Install

```bash
npm install @zane-chen/agents-are-thinking
```

Requires a bundler (Vite, Webpack 5, Rollup, etc.) or Node.js with WASM support.

## Basic usage

```ts
import { BrailleFire } from '@zane-chen/agents-are-thinking'

const ef = new BrailleFire()

setInterval(() => {
  console.log(ef.step())
}, 100)

// call ef.free() when done to release WASM memory
```

## List all effects

```ts
import { EFFECTS } from '@zane-chen/agents-are-thinking'

for (const Cls of EFFECTS) {
  console.log(Cls.name, '-', Cls.description)
}
```

## With React

```tsx
import { useEffect, useRef, useState } from 'react'
import { ShadeFire } from '@zane-chen/agents-are-thinking'

function EffectDisplay() {
  const [frame, setFrame] = useState('')

  useEffect(() => {
    const ef = new ShadeFire()
    const id = setInterval(() => setFrame(ef.step()), 100)
    return () => {
      clearInterval(id)
      ef.free()
    }
  }, [])

  return <pre style={{ fontFamily: 'monospace' }}>{frame}</pre>
}
```

Use a monospace font. Each character in the frame string must be the same width — wrap each char in a fixed-width `<span>` for unicode block characters.

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
