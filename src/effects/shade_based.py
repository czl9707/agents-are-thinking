import math

from src.helpers.shade_helper import ShadeFrame
from src.effects.base import (
    Effect, WIDTH,
    TEMPORAL_SPEED, SPATIAL_FREQUENCY, CYCLE_LENGTH,
)


class ShadeWave(Effect):
    name = "shade-wave"
    description = "Smooth sine wave scrolls across as shade density"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * SPATIAL_FREQUENCY.LOW) + 1) / 2
            frame.set(i, v)
        return frame.render()


class ShadeScanner(Effect):
    name = "shade-scanner"
    description = "Bright line sweeps across with a fading trail"

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
    description = "FASTing flames rising from the bottom"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        for x in range(WIDTH):
            v = (math.sin(self._frame * TEMPORAL_SPEED.FAST + x * SPATIAL_FREQUENCY.DENSE) * 0.3
                 + math.sin(self._frame * TEMPORAL_SPEED.INTENSE + x * SPATIAL_FREQUENCY.HIGH) * 0.2
                 + 0.5)
            frame.set(x, max(0.0, min(1.0, v)))
        return frame.render()


class ShadeRipple(Effect):
    name = "shade-ripple"
    description = "Concentric rings pulse outward from center"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        cx = (WIDTH - 1) / 2
        for i in range(WIDTH):
            dist = abs(i - cx) / cx
            wave = (math.sin(dist * SPATIAL_FREQUENCY.EXTRA_DENSE - self._frame * TEMPORAL_SPEED.MODERATE) + 1) / 2
            density = max(0.0, wave - dist * 0.5)
            frame.set(i, density)
        return frame.render()


class ShadeBreathe(Effect):
    name = "shade-breathe"
    description = "All columns breathe in and out in unison"

    def _render(self) -> list[str]:
        phase = self._frame % CYCLE_LENGTH.MEDIUM
        v = (math.sin(phase * math.pi / 10) + 1) / 2
        frame = ShadeFrame(WIDTH)
        for i in range(WIDTH):
            frame.set(i, v)
        return frame.render()


class ShadeSeeSaw(Effect):
    name = "shade-seesaw"
    description = "Left and right halves alternate like a seesaw"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        phase = (math.sin(self._frame * TEMPORAL_SPEED.MODERATE) + 1) / 2
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

    _SPEED = TEMPORAL_SPEED.CRAWL

    def __init__(self) -> None:
        super().__init__(seed=38)
        self._tiers: list[int] = [self._rng.randint(0, 2) for _ in range(WIDTH)]

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
            w1 = (math.sin(self._frame * TEMPORAL_SPEED.MODERATE + i * SPATIAL_FREQUENCY.HIGH) + 1) / 2
            w2 = (math.sin(self._frame * TEMPORAL_SPEED.FAST + i * SPATIAL_FREQUENCY.HIGH) + 1) / 2
            frame.add(i, w1 * 0.5)
            frame.add(i, w2 * 0.5)
        return frame.render()


class ShadePinch(Effect):
    name = "shade-pinch"
    description = "Bright edges alternate between left and right sides"

    def _render(self) -> list[str]:
        frame = ShadeFrame(WIDTH)
        cx = (WIDTH - 1) / 2
        phase = (math.sin(self._frame * TEMPORAL_SPEED.GENTLE) + 1) / 2
        for i in range(WIDTH):
            edge_dist = abs(i - cx) / cx
            side_phase = phase if i <= cx else 1.0 - phase
            density = edge_dist * (0.3 + 0.7 * side_phase)
            frame.set(i, density)
        return frame.render()
