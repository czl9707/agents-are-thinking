import math
import random

from src.helpers.dot_helper import DotFrame
from src.effects.base import Effect, WIDTH


class DotWave(Effect):
    name = "dot-wave"
    description = "Sine wave where dot size varies instead of shade"

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * 0.6) + 1) / 2
            frame.set(i, v)
        return frame.render()


class DotHeartbeat(Effect):
    name = "dot-heartbeat"
    description = "Heartbeat throb, all dots size up then snap back"

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        phase = self._frame % 16
        if phase < 5:
            v = phase / 5
        elif phase < 8:
            v = 1.0 - (phase - 5) / 6
        elif phase < 10:
            v = (phase - 8) / 4
        else:
            v = max(0.0, 1.0 - (phase - 10) / 8)
        for i in range(WIDTH):
            frame.set(i, v)
        return frame.render()


class DotPulse(Effect):
    name = "dot-pulse"
    description = "Dots expand from center then shrink back"

    _FULL_CYCLE = WIDTH * 2

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        cx = WIDTH / 2
        f = self._frame % self._FULL_CYCLE
        if f < (self._FULL_CYCLE // 2):
            ring = f
        else:
            ring = self._FULL_CYCLE - f - 1
        for i in range(WIDTH):
            dist = abs(i - cx + 0.5)
            v = max(0.0, min(1.0, (ring - dist + 1) / 3))
            frame.set(i, v)
        self._frame += 1
        return frame.render()


class DotArrow(Effect):
    name = "dot-arrow"
    description = "Two solid dots and one outline slide and reverse"

    _SPEED = 1
    _PAUSE = 12
    _LENGTH = 5

    def __init__(self):
        super().__init__()
        self._travel = WIDTH + 2 * (self._LENGTH - 1)
        self._cycle = self._travel * 2 + self._PAUSE

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        f = self._frame % self._cycle
        if f < self._travel:
            head = -(self._LENGTH - 1) + f * self._SPEED
            direction = 1
        elif f < self._travel * 2:
            head = WIDTH - 1 + (self._LENGTH - 1) - (f - self._travel) * self._SPEED
            direction = -1
        else:
            return frame.render()
        for t in range(self._LENGTH):
            pos = head - direction * t
            if 0 <= pos < WIDTH:
                v = (self._LENGTH - t) / self._LENGTH
                frame.set(pos, v)
        return frame.render()


class DotBounce(Effect):
    name = "dot-bounce"
    description = "Random dots bounce between sizes at different speeds"

    def __init__(self):
        super().__init__()
        self._phases = [random.uniform(0, math.tau) for _ in range(WIDTH)]
        self._speeds = [random.uniform(0.15, 0.5) for _ in range(WIDTH)]

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin(self._phases[i] + self._frame * self._speeds[i]) + 1) / 2
            frame.set(i, v)
        return frame.render()
