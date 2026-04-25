# Frame Rendering Pipeline

## Problem

Effects currently manipulate braille bit positions directly using `dot()` and `encode()`.
This ties effect logic to braille internals and makes it hard to create effects that
think in terms of spatial patterns.

## Design

### Frame class

A `Frame` in `src/braille.py` replaces the old module-level functions (`dot`, `encode`,
`BRAILLE`, `ALL_DOTS`). Effects work exclusively in pixel coordinates.

- `set(x, y)` — set a single dot by absolute pixel position
  - `x ∈ [0, width * 2)`, `y ∈ [0, height * 4)`
  - Internally maps to the correct braille cell and bit
- `render() -> list[str]` — returns one string per braille row

Internal details (bit mapping, character table) are private to the class.

### Effect migration

All effects rewritten to use pixel coordinates. No effect imports or references
braille internals.

- **BrailleSpin** — perimeter trail stamped at each char position
- **BrailleWave** — sweeping dot with phase offset per column (no lookup table)
- **BrailleCascade** — progressive dot fill left to right
- **BrailleRandom** — pseudo-random pixel pattern per column
- **BrailleBreathe** — proportional fill intensity from center outward

### Scope

- WIDTH=9, HEIGHT=1 (same as current)
- No multi-row support in this iteration
- BlockProgress in progress.py is unchanged (non-braille)
