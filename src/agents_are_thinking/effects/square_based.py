import math

from agents_are_thinking.helpers.square_helper import SquareFrame
from agents_are_thinking.effects.base import Effect, WIDTH, CYCLE_LENGTH, PAUSE


class SquarePulse(Effect):
    name = "square-pulse"
    description = "Expanding ring radiates from center then contracts back"

    _FULL_CYCLE = WIDTH * 2

    def _render(self) -> list[str]:
        frame = SquareFrame(WIDTH)
        cx = WIDTH / 2
        f = self._frame % self._FULL_CYCLE
        if f < (self._FULL_CYCLE // 2):
            ring = f
        else:
            ring = self._FULL_CYCLE - f - 1
        for i in range(WIDTH):
            dist = abs(i - cx + 0.5)
            v = max(0.0, min(1.0, (ring - dist + 1) / 2))
            frame.set(i, v)
        self._frame += 1
        return frame.render()


class SquareFill(Effect):
    name = "square-fill"
    description = "Fills progressively from left to right, then resets"

    _SPEED = 3
    _CYCLE = WIDTH * _SPEED + 8

    def _render(self) -> list[str]:
        frame = SquareFrame(WIDTH)
        pos = self._frame % self._CYCLE
        for i in range(WIDTH):
            target = i * self._SPEED
            elapsed = pos - target
            if elapsed < 0:
                frame.set(i, 0.0)
            elif elapsed < self._SPEED:
                frame.set(i, 0.5)
            else:
                frame.set(i, 1.0)
        return frame.render()


class SquareBlink(Effect):
    name = "square-blink"
    description = "Pattern alternates on a fixed cycle"

    _PERIOD = CYCLE_LENGTH.SHORT

    def _render(self) -> list[str]:
        frame = SquareFrame(WIDTH)
        offset = (self._frame // self._PERIOD) % 3
        for i in range(WIDTH):
            level = (i + offset) % 3
            frame.set(i, level / 2.0)
        return frame.render()


class SquareArrow(Effect):
    name = "square-arrow"
    description = "Shape slides right, reverses left, pauses, repeats"

    _SPEED = 1
    _PAUSE = PAUSE.MEDIUM

    def __init__(self):
        super().__init__()
        self._travel = WIDTH + 2
        self._cycle = self._travel * 2 + self._PAUSE

    def _render(self) -> list[str]:
        frame = SquareFrame(WIDTH)
        f = self._frame % self._cycle
        if f < self._travel:
            head = -2 + f * self._SPEED
            direction = 1
        elif f < self._travel * 2:
            head = WIDTH - 1 - (f - self._travel) * self._SPEED
            direction = -1
        else:
            return frame.render()
        empty = head
        solid1 = head + direction
        solid2 = head + direction * 2
        for x in (solid1, solid2):
            if 0 <= x < WIDTH:
                frame.set(x, 1.0)
        if 0 <= empty < WIDTH:
            frame.set(empty, 0.5)
        return frame.render()
