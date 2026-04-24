# Thinking Effects Bootstrap — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Bootstrap the thinking-effects repo with a braille spinner and a block progress bar, runnable from a CLI menu.

**Architecture:** Effect base class with `tick(frame) -> list[str]` protocol. Two concrete effects. Rich Live for rendering. Click for CLI. One entry point: `python main.py`.

**Tech Stack:** Python 3.11+, rich, click

---

### Task 1: Project Scaffold

**Files:**
- Create: `pyproject.toml` (via `uv init`)
- Create: `src/__init__.py`
- Create: `src/effects/__init__.py`

**Step 1: Init project with uv**

Run: `uv init --name thinking-effects --python 3.11`

This creates `pyproject.toml` and `.python-version`.

**Step 2: Add deps with uv**

Run: `uv add rich click`

This updates `pyproject.toml` and creates `uv.lock`.

**Step 3: Create empty __init__.py files**

`src/__init__.py` — empty file.
`src/effects/__init__.py` — empty file (registry comes in Task 3).

**Step 4: Verify**

Run: `uv run python -c "import rich; import click; print('ok')"`
Expected: `ok`

**Step 5: Commit**

```
git init
git add .
git commit -m "init: project scaffold with uv, rich, and click"
```

---

### Task 2: Effect Base Class

**Files:**
- Create: `src/effects/base.py`

**Step 1: Write the Effect ABC**

```python
from abc import ABC, abstractmethod


class Effect(ABC):
    name: str = ""
    description: str = ""

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    @abstractmethod
    def tick(self, frame: int) -> list[str]:
        ...
```

**Step 2: Verify import works**

Run: `uv run python -c "from src.effects.base import Effect; print(Effect.__abstractmethods__)"`
Expected: `frozenset({'tick'})`

**Step 3: Commit**

```
git add src/effects/base.py
git commit -m "feat: add Effect base class"
```

---

### Task 3: Braille Spinner

**Files:**
- Create: `src/effects/spinners.py`
- Modify: `src/effects/__init__.py`

**Step 1: Write BrailleSpinner**

```python
from src.effects.base import Effect


class BrailleSpinner(Effect):
    name = "braille"
    description = "Braille dot spinner"

    FRAMES = ["\u2807", "\u2819", "\u2839", "\u2838", "\u283c", "\u2834", "\u2826", "\u2827", "\u2807", "\u280f"]

    def tick(self, frame: int) -> list[str]:
        char = self.FRAMES[frame % len(self.FRAMES)]
        lines = [""] * self.height
        mid = self.height // 2
        lines[mid] = f"  {char} Thinking..."
        return lines
```

**Step 2: Register in __init__.py**

```python
from src.effects.base import Effect
from src.effects.spinners import BrailleSpinner

EFFECTS: list[type[Effect]] = [
    BrailleSpinner,
]
```

**Step 3: Verify**

Run: `uv run python -c "from src.effects import EFFECTS; e = EFFECTS[0](20, 3); print(e.tick(0)); print(e.tick(1))"`
Expected: lines with braille chars and "Thinking..."

**Step 4: Commit**

```
git add src/effects/
git commit -m "feat: add braille spinner effect"
```

---

### Task 4: Block Progress Bar

**Files:**
- Create: `src/effects/progress.py`
- Modify: `src/effects/__init__.py`

**Step 1: Write BlockProgress**

```python
from src.effects.base import Effect


class BlockProgress(Effect):
    name = "progress"
    description = "Block progress bar with spinner"

    SPINNER = ["\u2807", "\u2819", "\u2839", "\u2838", "\u283c", "\u2834", "\u2826", "\u2827", "\u2807", "\u280f"]
    FILLED = "\u2588"
    EMPTY = "\u2591"
    TOTAL_FRAMES = 100

    def tick(self, frame: int) -> list[str]:
        progress = frame % (self.TOTAL_FRAMES + 1)
        pct = progress / self.TOTAL_FRAMES
        bar_width = self.width - 8
        filled = int(bar_width * pct)
        bar = self.FILLED * filled + self.EMPTY * (bar_width - filled)
        spinner = self.SPINNER[frame % len(self.SPINNER)]
        lines = [""] * self.height
        mid = self.height // 2
        lines[mid] = f"  {spinner} [{bar}] {int(pct * 100):3d}%"
        return lines
```

**Step 2: Register in __init__.py — add BlockProgress to EFFECTS list**

```python
from src.effects.base import Effect
from src.effects.spinners import BrailleSpinner
from src.effects.progress import BlockProgress

EFFECTS: list[type[Effect]] = [
    BrailleSpinner,
    BlockProgress,
]
```

**Step 3: Verify**

Run: `uv run python -c "from src.effects import EFFECTS; e = EFFECTS[1](30, 3); print(e.tick(0)); print(e.tick(50))"`
Expected: progress bar lines at 0% and 50%

**Step 4: Commit**

```
git add src/effects/
git commit -m "feat: add block progress bar effect"
```

---

### Task 5: CLI Entry Point

**Files:**
- Create: `main.py`

**Step 1: Write main.py**

```python
import sys
import click
from rich.live import Live
from rich.text import Text

from src.effects import EFFECTS


@click.command()
@click.option("--effect", "-e", default=None, help="Effect name to run")
@click.option("--list", "list_effects", is_flag=True, help="List available effects")
def main(effect, list_effects):
    if list_effects:
        for ef in EFFECTS:
            click.echo(f"  {ef.name:20s} {ef.description}")
        return

    if effect:
        cls = next((e for e in EFFECTS if e.name == effect), None)
        if not cls:
            click.echo(f"Unknown effect: {effect}")
            sys.exit(1)
    else:
        click.echo("Available effects:\n")
        for i, ef in enumerate(EFFECTS):
            click.echo(f"  [{i}] {ef.name:20s} {ef.description}")
        click.echo()
        choice = click.prompt("Select effect", type=int, default=0)
        cls = EFFECTS[choice]

    instance = cls(width=50, height=3)
    frame = 0

    try:
        with Live(refresh_per_second=12, vertical_overflow="visible"):
            while True:
                lines = instance.tick(frame)
                instance._live.update(Text("\n".join(lines)))
                frame += 1
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
```

**Step 2: Run interactive menu**

Run: `uv run python main.py --list`
Expected: lists `braille` and `progress`

**Step 3: Run braille spinner**

Run: `uv run python main.py --effect braille`
Expected: animated braille spinner, Ctrl+C to exit

**Step 4: Run progress bar**

Run: `uv run python main.py --effect progress`
Expected: animated progress bar cycling 0-100%

**Step 5: Fix any rendering issues**

If `instance._live` doesn't work (private attribute), restructure to use Live properly:

```python
    instance = cls(width=50, height=3)
    frame = 0

    def generate():
        lines = instance.tick(frame)
        return Text("\n".join(lines))

    try:
        with Live(generate(), refresh_per_second=12) as live:
            while True:
                frame += 1
                live.update(generate())
    except KeyboardInterrupt:
        pass
```

**Step 6: Commit**

```
git add main.py
git commit -m "feat: add CLI entry point with interactive menu"
```

---

### Task 6: Verify End-to-End

**Step 1: Run all effects from CLI**

Run each and visually confirm:
- `uv run python main.py --effect braille` — spinning braille dot + "Thinking..."
- `uv run python main.py --effect progress` — progress bar filling and resetting
- `uv run python main.py` — interactive menu, pick each effect

**Step 2: Ctrl+C exits cleanly for each**

No traceback should be visible (or suppress it).

**Step 3: Final commit**

```
git add -A
git commit -m "chore: final bootstrap cleanup"
```
