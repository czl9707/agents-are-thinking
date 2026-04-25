import sys
import time

import click
from rich.console import Console
from rich.live import Live
from rich.text import Text

from src.effects import EFFECTS
from src.effects.base import WIDTH

FPS = 16
GAP = 4
LABEL_W = max(len(ef.name) for ef in EFFECTS)
CELL_W = LABEL_W + 1 + WIDTH
COL_W = CELL_W + GAP


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


@click.command()
@click.option("--effect", "-e", default=None, help="Effect name to run")
@click.option("--list", "list_effects", is_flag=True, help="List available effects")
def main(effect: str, list_effects: bool):
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

        def generate():
            out = "".join(inst.step())
            return Text(f"{cls.name:{LABEL_W}s} {out}")

        with Live(generate(), refresh_per_second=FPS, console=console) as live:
            try:
                while True:
                    live.update(generate())
                    time.sleep(1 / FPS)
            except KeyboardInterrupt:
                pass
    else:
        instances = [ef() for ef in EFFECTS]

        with Live(
            _render_grid(instances, console), refresh_per_second=FPS, console=console
        ) as live:
            try:
                while True:
                    live.update(_render_grid(instances, console))
                    time.sleep(1 / FPS)
            except KeyboardInterrupt:
                pass


if __name__ == "__main__":
    main()
