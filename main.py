import random
import sys
import time

import click
from rich import box as rich_box
from rich.console import Console
from rich.live import Live
from rich.panel import Panel
from rich.text import Text

from src.effects import EFFECTS
from src.effects.base import WIDTH
import src.effects as effects

FPS = 16
GAP = 4
LABEL_W = max(len(ef.name) for ef in EFFECTS)
CELL_W = LABEL_W + 1 + WIDTH
COL_W = CELL_W + GAP

_PREVIEW_WORDS = [
    "thinking",
    "analyzing",
    "cooking",
    "brewing",
    "computing",
    "pondering",
    "dreaming",
    "processing",
    "vibing",
    "wondering",
    "imagining",
    "calculating",
    "marinating",
    "reflecting",
    "absorbing",
    "channeling",
    "meditating",
    "manifesting",
    "deciphering",
    "daydreaming",
    "ideating",
    "brainstorming",
    "philosophizing",
    "percolating",
    "simmering",
    "distilling",
    "unraveling",
    "connecting",
    "orchestrating",
    "choreographing",
    "architecting",
    "composing",
    "sculpting",
    "weaving",
    "stewing",
    "fermenting",
    "incubating",
    "crystallizing",
    "galvanizing",
    "reassembling",
    "reconfiguring",
    "calibrating",
    "harmonizing",
    "fractalizing",
    "quantizing",
    "transmuting",
    "metamorphosing",
    "osmosing",
    "photosynthesizing",
    "communing",
]


def _run_live(render_fn, console):
    with Live(render_fn(), refresh_per_second=FPS, console=console) as live:
        try:
            while True:
                live.update(render_fn())
                time.sleep(1 / FPS)
        except KeyboardInterrupt:
            pass


def _render_grid(instances, console):
    cols = max(1, console.width // COL_W)
    rows = []
    for i in range(0, len(instances), cols):
        parts = []
        for ef in instances[i : i + cols]:
            out = "".join(ef.step())
            parts.append(f"{ef.name:{LABEL_W}s} {out:<{WIDTH}}")
        rows.append((" " * GAP).join(parts))
    return Text("\n".join(rows))


def _render_preview(instances, frame_num, console):
    word_w = max(len(w) for w in _PREVIEW_WORDS)
    cell_w = WIDTH + 1 + word_w + 3
    gap = 4
    col_w = cell_w + gap

    cols = max(1, (console.width - 4) // col_w)
    dot_count = (frame_num // FPS) % 4
    rows = []
    for i in range(0, len(instances), cols):
        parts = []
        for j, ef in enumerate(instances[i : i + cols]):
            idx = i + j
            word = _PREVIEW_WORDS[idx % len(_PREVIEW_WORDS)]
            out = "".join(ef.step())
            dots = "." * dot_count
            parts.append(f"{out:<{WIDTH}} {word}{dots:<{word_w + 3 - len(word)}}")
        rows.append((" " * gap).join(parts))
        if i < len(instances) - cols:
            rows.append("")

    content = Text("\n".join(rows))
    return Panel(
        content,
        width=console.width,
        box=rich_box.ROUNDED,
        padding=0,
        expand=True,
    )


@click.group(invoke_without_command=True)
@click.pass_context
@click.option("--effect", "-e", default=None, help="Effect name to run")
@click.option("--list", "list_effects", is_flag=True, help="List available effects")
def cli(ctx, effect: str, list_effects: bool):
    if ctx.invoked_subcommand is not None:
        return
    console = Console()

    if list_effects:
        for ef in EFFECTS:
            click.echo(f"  {ef.name:20s} {ef.description}")
        return

    if effect:
        cls = next((e for e in EFFECTS if e.name == effect), None)
        if not cls:
            click.echo(f"Unknown effect: {effect}")
            sys.exit(1)

        inst = cls()

        def render():
            out = "".join(inst.step())
            return Text(f"{cls.name:{LABEL_W}s} {out}")

        _run_live(render, console)
    else:
        instances = [ef() for ef in EFFECTS]
        _run_live(lambda: _render_grid(instances, console), console)


@cli.command()
def preview():
    console = Console()
    efs = EFFECTS[:18]
    random.shuffle(efs)
    instances = [ef() for ef in efs]
    frame_num = [0]

    def render():
        frame_num[0] += 1
        return _render_preview(instances, frame_num[0], console)

    _run_live(render, console)


if __name__ == "__main__":
    cli()
