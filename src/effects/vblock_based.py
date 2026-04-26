import math

from src.helpers.vblock_helper import VBlockFrame
from src.effects.base import Effect, WIDTH


class VBlockWave(Effect):
    name = "vblock-wave"
    description = "Sine wave scrolls, each cell fills to wave height"

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * 0.6) + 1) / 2
            frame.set(i, v)
        return frame.render()


class VBlockFill(Effect):
    name = "vblock-fill"
    description = "Loading bar sweeps left to right, resets"

    _CYCLE = WIDTH + 4

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        pos = self._frame % self._CYCLE
        for i in range(WIDTH):
            if i < pos:
                dist = pos - i
                v = max(0.0, 1.0 - (dist - 1) / (WIDTH - 1))
                frame.set(i, v)
        return frame.render()


class VBlockTide(Effect):
    name = "vblock-tide"
    description = "Left fills while right drains, then reverses"

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        phase = (math.sin(self._frame * 0.15) + 1) / 2
        for i in range(WIDTH):
            t = i / (WIDTH - 1)
            density = phase * (1 - t) + (1 - phase) * t
            frame.set(i, density)
        return frame.render()


class VBlockBreathe(Effect):
    name = "vblock-breathe"
    description = "All cells fill and empty in unison"

    def _render(self) -> list[str]:
        phase = self._frame % 20
        v = (math.sin(phase * math.pi / 10) + 1) / 2
        frame = VBlockFrame(WIDTH)
        for i in range(WIDTH):
            frame.set(i, v)
        return frame.render()
