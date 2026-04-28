import math

from agents_are_thinking.helpers.dot_helper import DotFrame
from agents_are_thinking.effects.base import (
    Effect, WIDTH,
    TEMPORAL_SPEED, SPATIAL_FREQUENCY, CYCLE_LENGTH, PAUSE,
)


class DotWave(Effect):
    name = "dot-wave"
    description = "Smooth sine wave scrolls across as dot sizes"

    cycle_length = 10

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * SPATIAL_FREQUENCY.LOW) + 1) / 2
            frame.set(i, v)
        return frame.render()


class DotHeartbeat(Effect):
    name = "dot-heartbeat"
    description = "Pulsing rhythm that mimics a heartbeat"

    cycle_length = 18

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        phase = self._frame % CYCLE_LENGTH.MEDIUM
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
    description = "Expanding ring radiates from center then contracts back"

    _FULL_CYCLE = WIDTH * 2
    cycle_length = 18

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
        return frame.render()


class DotArrow(Effect):
    name = "dot-arrow"
    description = "Shape slides right, reverses left, pauses, repeats"

    _SPEED = 1
    _PAUSE = PAUSE.MEDIUM
    _LENGTH = 5
    cycle_length = 46

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
    description = "Each dot bounces independently at its own speed"

    cycle_length = 25

    def __init__(self):
        super().__init__()
        self._phases = [self._rng.uniform(0, math.tau) for _ in range(WIDTH)]
        self._speeds = [self._rng.uniform(TEMPORAL_SPEED.GENTLE, TEMPORAL_SPEED.FAST) for _ in range(WIDTH)]

    def _render(self) -> list[str]:
        frame = DotFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin(self._phases[i] + self._frame * self._speeds[i]) + 1) / 2
            frame.set(i, v)
        return frame.render()
