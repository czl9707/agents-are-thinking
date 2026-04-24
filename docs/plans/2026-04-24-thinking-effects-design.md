# Thinking Effects Playground — Design

## Goal

Build a Python playground for terminal-based loading/thinking spinner animations. Start with individual effects, then compose them into a grid of animated "agents" — eventually reaching a 100-agent dashboard.

## Stack

- **Language:** Python 3.11+
- **Package manager:** [uv](https://docs.astral.sh/uv/)
- **Rendering:** [rich](https://github.com/Textualize/rich) — Live display, colors, layouts
- **CLI:** [click](https://click.palletsprojects.com/) — command selection and options
- No other deps.

## Project Structure

```
thinking/
├── docs/
│   └── plans/
├── src/
│   ├── effects/
│   │   ├── __init__.py        # Registry of all effects
│   │   ├── spinners.py        # Spinner animations (braille, dots, arrows, etc.)
│   │   └── progress.py        # Progress bar (spinner + bar combo)
│   ├── grid.py                # Grid layout engine (rows x cols) — Phase 3
│   ├── cell.py                # Single cell renderer — Phase 3
│   └── agent.py               # Simulated agent state machine — Phase 3
├── main.py                    # CLI entry
└── requirements.txt
```

## Effect Interface

```python
class Effect(ABC):
    name: str
    description: str

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    @abstractmethod
    def tick(self, frame: int) -> list[str]:
        """Return list of strings (one per line) to render. Called every frame."""
        ...
```

- `width` / `height` define character bounds
- `tick()` called at ~15fps with incrementing `frame`
- Returns exactly `height` lines, each at most `width` chars
- Purely functional — no side effects

## Effects (Bootstrapped)

### Spinners (`spinners.py`)

One spinner character set for now, easy to add more later:

| Name       | Frames              |
|------------|---------------------|
| braille    | `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏` |

Each spinner renders: the spinning character + a label, centered in the cell.

### Progress (`progress.py`)

One progress style for now:

| Name   | Style                        |
|--------|------------------------------|
| blocks | `[████░░░░░░] 40%`           |

Blocks spinner + filling progress bar. Loops 0% → 100% then resets.

## CLI

```
$ python main.py                     # Interactive menu
$ python main.py --effect braille    # Run specific effect
$ python main.py --list              # List effects
$ python main.py --grid 4x6         # Grid mode (Phase 3)
```

## Frame Loop

```python
from rich.live import Live

frame = 0
effect = BrailleSpinner(width=40, height=3)

with Live(refresh_per_second=15):
    while True:
        lines = effect.tick(frame)
        # render lines
        frame += 1
```

- 12-15 fps target
- Clean exit on Ctrl+C

## Dependencies (`pyproject.toml` — managed by uv)

```
rich>=13.0
click>=8.0
```

## Phase Plan

### Phase 1 — Bootstrap (now)
- [ ] Project setup (`uv init`, `uv add rich click`, src/)
- [ ] Effect base class + registry
- [ ] Braille spinner (1 character set)
- [ ] Block progress bar (1 style)
- [ ] Interactive CLI menu

### Phase 2 — More Character Sets
- [ ] Add more spinner sets (dots, arrows, etc.)
- [ ] Add more progress styles
- [ ] Color/styling options

### Phase 3 — Grid Engine
- [ ] Cell wrapper (bounded rendering)
- [ ] Grid layout (rows x cols)
- [ ] Agent simulator (random states)
- [ ] `--grid NxM` CLI mode
- [ ] 100-agent demo

### Phase 4 — Web (future)
- [ ] React + Vite
- [ ] xterm.js terminal emulator
- [ ] Port effects to JS/TS
