import math
import random

from src.helpers.shade_helper import ShadeFrame
from src.effects.base import Effect, WIDTH


class ShadeWave(Effect):
    name = "shade-wave"
    description = "Sine wave scrolls across with density gradient"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * 0.6) + 1) / 2
            frame.set(i, v)
        return frame.render()


class ShadeScanner(Effect):
    name = "shade-scanner"
    description = "Bright line sweeps with smooth falloff trail"

    _SCAN_RANGE = WIDTH + 6

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        pos = self._frame % self._SCAN_RANGE
        for i in range(WIDTH):
            dist = abs(i - pos)
            density = max(0.0, 1.0 - dist / 4)
            frame.set(i, density)
        return frame.render()


class ShadeFire(Effect):
    name = "shade-fire"
    description = "Flames with flickering heat columns"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        for x in range(WIDTH):
            v = (math.sin(self._frame * 0.8 + x * 2.1) * 0.3
                 + math.sin(self._frame * 1.3 + x * 0.7) * 0.2
                 + 0.5)
            frame.set(x, max(0.0, min(1.0, v)))
        return frame.render()


class ShadePulse(Effect):
    name = "shade-pulse"
    description = "Concentric rings radiating from center"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        cx = WIDTH / 2
        for i in range(WIDTH):
            dist = abs(i - cx) / cx
            wave = (math.sin(dist * 6 - self._frame * 0.4) + 1) / 2
            density = max(0.0, wave - dist * 0.5)
            frame.set(i, density)
        return frame.render()


class ShadeBreathe(Effect):
    name = "shade-breathe"
    description = "Entire bar inhales and exhales"

    def _render(self) -> list[str]:
        phase = self._frame % 20
        v = (math.sin(phase * math.pi / 10) + 1) / 2
        frame = ShadeFrame(WIDTH)
        for i in range(WIDTH):
            frame.set(i, v)
        return frame.render()


class ShadeSeeSaw(Effect):
    name = "shade-seesaw"
    description = "Left and right halves alternate brightness"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        phase = (math.sin(self._frame * 0.3) + 1) / 2
        for i in range(WIDTH):
            t = i / (WIDTH - 1)
            raw = (math.cos(t * math.pi) + 1) / 2
            blend = 0.5 + (raw - 0.5) * 2
            density = phase * blend + (1.0 - phase) * (1.0 - blend)
            frame.set(i, density)
        return frame.render()


class ShadeBlink(Effect):
    name = "shade-blink"
    description = "Cells breathe in three randomly staggered tiers"

    _SPEED = 0.05

    def __init__(self) -> None:
        super().__init__()
        self._tiers: list[int] = [random.randint(0, 2) for _ in range(WIDTH)]

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        for i in range(WIDTH):
            offset = self._tiers[i] / 3.0
            v = (math.sin((self._frame * self._SPEED + offset) * math.pi * 2) + 1) / 2
            frame.set(i, v)
        return frame.render()


class ShadeLayers(Effect):
    name = "shade-layers"
    description = "Two sine waves add together creating interference moire"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        for i in range(WIDTH):
            w1 = (math.sin(self._frame * 0.4 + i * 0.8) + 1) / 2
            w2 = (math.sin(self._frame * 0.7 + i * 1.2) + 1) / 2
            frame.add(i, w1 * 0.5)
            frame.add(i, w2 * 0.5)
        return frame.render()


class ShadePinch(Effect):
    name = "shade-pinch"
    description = "Bright edges alternate between left and right sides"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        cx = (WIDTH - 1) / 2
        phase = (math.sin(self._frame * 0.25) + 1) / 2
        for i in range(WIDTH):
            edge_dist = abs(i - cx) / cx
            side_phase = phase if i <= cx else 1.0 - phase
            density = edge_dist * (0.3 + 0.7 * side_phase)
            frame.set(i, density)
        return frame.render()


class ShadeStaircase(Effect):
    name = "shade-staircase"
    description = "Gradient staircase that rotates one cell per frame"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        shift = self._frame % WIDTH
        for i in range(WIDTH):
            density = ((i + shift) % WIDTH) / (WIDTH - 1)
            frame.set(i, density)
        return frame.render()


class ShadeTide(Effect):
    name = "shade-tide"
    description = "Brightness fills from left then drains from right"

    _CYCLE = WIDTH * 2

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        t = self._frame % self._CYCLE
        fill = t if t < WIDTH else self._CYCLE - t
        for i in range(WIDTH):
            if i < fill:
                frame.set(i, (i + 1) / fill)
            else:
                frame.set(i, 0.0)
        return frame.render()


class ShadeGrow(Effect):
    name = "shade-grow"
    description = "Bright island expands from center then shrinks back"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        cx = (WIDTH - 1) / 2
        t = (math.sin(self._frame * 0.2) + 1) / 2
        radius = t * cx
        for i in range(WIDTH):
            dist = abs(i - cx)
            if dist <= radius:
                density = 1.0 - (dist / max(radius, 0.01)) * 0.5
            else:
                density = max(0.0, 1.0 - (dist - radius) / 2) * 0.3
            frame.set(i, density)
        return frame.render()
