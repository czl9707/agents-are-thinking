import math
import random

from src.helpers.bar_helper import BarFrame
from src.effects.base import Effect, WIDTH, HEIGHT


class BarEqualizer(Effect):
    name = "bar-eq"
    description = "Bars bouncing at different heights like an audio visualizer"

    def __init__(self):
        super().__init__()
        self._phases = [random.uniform(0, math.tau) for _ in range(WIDTH)]
        self._speeds = [random.uniform(0.2, 0.6) for _ in range(WIDTH)]

    def _render(self) -> list[str]:
        frame = BarFrame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            v = (math.sin(self._phases[i] + self._frame * self._speeds[i]) + 1) / 2
            frame.set(i, 0, v)
        return frame.render()


class BarWave(Effect):
    name = "bar-wave"
    description = "Smooth sine wave rendered as bar heights"

    def _render(self) -> list[str]:
        frame = BarFrame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            v = (math.sin((i * 0.5) + self._frame * 0.15) + 1) / 2
            frame.set(i, 0, v)
        return frame.render()
