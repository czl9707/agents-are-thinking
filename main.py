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


if __name__ == "__main__":
    main()
