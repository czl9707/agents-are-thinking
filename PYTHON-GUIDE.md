## Install

```bash
uv add agents-are-thinking
```

Requires Python 3.11+.

## Basic usage

```python
import time
from agents_are_thinking import ShadeFire

ef = ShadeFire()
t0 = time.time()
for frame in ef:
    print(f"\r{frame}", end="", flush=True)
    if time.time() - t0 > 5:
        break
```

Effects are iterators. They never stop on their own — break when you're done.

## With Rich

```python
import time
from rich.live import Live
from agents_are_thinking import BrailleFire

ef = BrailleFire()
t0 = time.time()

with Live(refresh_per_second=16) as live:
    for frame in ef:
        time.sleep(0.1)
        live.update(frame)
        if time.time() - t0 > 5:
            break
```

## List all effects

```python
from agents_are_thinking import EFFECTS

for cls in EFFECTS:
    print(cls.name, "-", cls.description)
```

## CLI

Install with the `[cli]` extra for a terminal preview:

```bash
uv tool install agents-are-thinking[cli]
```

```bash
agents-are-thinking preview          # animated gallery with verbs
agents-are-thinking list             # list all effects
agents-are-thinking run braille-spin # run a single effect
agents-are-thinking                  # name-effect showcase
```

## API

Each effect class has:

| Method | Description |
|--------|-------------|
| `cls.name` | Effect name (class attribute) |
| `cls.description` | One-line description (class attribute) |
| `cls.cycle_length` | Frames before the animation repeats (class attribute) |
| `instance.step()` | Returns the next frame string |
| `iter(instance)` | Yields frame strings forever |

All 48 classes are importable directly: `from agents_are_thinking import BrailleFire, ShadeWave, ...`.
