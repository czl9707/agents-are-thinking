import random
import sys
import time

import click
from rich import box as rich_box
from rich.panel import Panel
from rich.text import Text
from rich.console import Console

from agents_are_thinking.effects import EFFECTS
from agents_are_thinking.effects.base import WIDTH


def _run_live(render_fn, console, screen=False):
    from rich.live import Live

    with Live(get_renderable=render_fn, refresh_per_second=FPS, console=console, screen=screen):
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            pass


FPS = 16
GAP = 4

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


def _render_grid(instances, console):
    label_w = max(len(ef.name) for ef in EFFECTS)
    cell_w = label_w + 1 + WIDTH
    col_w = cell_w + GAP
    cols = max(1, console.width // col_w)
    rows = []
    for i in range(0, len(instances), cols):
        parts = []
        for ef in instances[i : i + cols]:
            out = next(ef)
            parts.append(f"{ef.name:{label_w}s} {out:<{WIDTH}}")
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
            out = next(ef)
            dots = "." * dot_count
            parts.append(f"{out:<{WIDTH}} {word}{dots:<{word_w + 3 - len(word)}}")
        rows.append((" " * gap).join(parts))
        if i < len(instances) - cols:
            rows.append("")

    content = Text("\n".join(rows))
    return Panel(
        content,
        box=rich_box.ROUNDED,
    )


def cli():
    label_w = max(len(ef.name) for ef in EFFECTS)

    @click.group(invoke_without_command=True)
    @click.pass_context
    @click.option("--effect", "-e", default=None, help="Effect name to run")
    @click.option("--list", "list_effects", is_flag=True, help="List available effects")
    def root(ctx, effect: str, list_effects: bool):
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
                out = next(inst)
                return Text(f"{cls.name:{label_w}s} {out}")

            _run_live(render, console)
        else:
            instances = [ef() for ef in EFFECTS]
            _run_live(lambda: _render_grid(instances, console), console)

    @root.command("list")
    def list_cmd():
        for ef in EFFECTS:
            click.echo(f"  {ef.name:20s} {ef.description}")

    @root.command()
    @click.argument("name")
    def run(name: str):
        cls = next((e for e in EFFECTS if e.name == name), None)
        if not cls:
            click.echo(f"Unknown effect: {name}")
            sys.exit(1)

        console = Console()
        inst = cls()

        def render():
            out = next(inst)
            return Text(f"{cls.name:{label_w}s} {out}")

        _run_live(render, console)

    @root.command()
    def preview():
        console = Console()
        instances = [ef() for ef in EFFECTS]
        frame_num = [0]

        def render():
            frame_num[0] += 1
            return _render_preview(instances, frame_num[0], console)

        _run_live(render, console, screen=True)

    root()


if __name__ == "__main__":
    cli()
