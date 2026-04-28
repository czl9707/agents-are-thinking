import math

from agents_are_thinking.helpers.bar_helper import BarFrame
from agents_are_thinking.effects.base import Effect, WIDTH, TEMPORAL_SPEED, SPATIAL_FREQUENCY


class BarBounce(Effect):
    name = "bar-bounce"
    description = "Each bar bounces independently at its own speed"

    cycle_length = 25

    def __init__(self):
        super().__init__()
        self._phases = [self._rng.uniform(0, math.tau) for _ in range(WIDTH)]
        self._speeds = [self._rng.uniform(TEMPORAL_SPEED.GENTLE, TEMPORAL_SPEED.FAST) for _ in range(WIDTH)]

    def _render(self) -> list[str]:
        frame = BarFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin(self._phases[i] + self._frame * self._speeds[i]) + 1) / 2
            frame.set(i, v)
        return frame.render()


class BarWave(Effect):
    name = "bar-wave"
    description = "Smooth sine wave scrolls across as bar heights"

    cycle_length = 25

    def _render(self) -> list[str]:
        frame = BarFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i * SPATIAL_FREQUENCY.LOW) + self._frame * TEMPORAL_SPEED.GENTLE) + 1) / 2
            frame.set(i, v)
        return frame.render()


class BarSeeSaw(Effect):
    name = "bar-seesaw"
    description = "Left and right halves alternate like a seesaw"

    cycle_length = 25

    def _render(self) -> list[str]:
        frame = BarFrame(WIDTH)
        t = (math.sin(self._frame * TEMPORAL_SPEED.GENTLE) + 1) / 2
        for i in range(WIDTH):
            ratio = i / (WIDTH - 1) if WIDTH > 1 else 0.5
            v = t * (1 - ratio) + (1 - t) * ratio
            frame.set(i, v)
        return frame.render()
