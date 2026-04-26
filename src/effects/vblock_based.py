import math
import random

from src.helpers.vblock_helper import VBlockFrame
from src.effects.base import (
    Effect, WIDTH,
    TEMPORAL_SPEED, SPATIAL_FREQUENCY, CYCLE_LENGTH,
)


class VBlockWave(Effect):
    name = "vblock-wave"
    description = "Smooth sine wave scrolls across as block heights"

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * SPATIAL_FREQUENCY.LOW) + 1) / 2
            frame.set(i, v)
        return frame.render()


class VBlockFill(Effect):
    name = "vblock-fill"
    description = "Fills progressively from left to right, then resets"

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
    description = "Fills from one side while draining from the other"

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        phase = (math.sin(self._frame * TEMPORAL_SPEED.GENTLE) + 1) / 2
        for i in range(WIDTH):
            t = i / (WIDTH - 1)
            density = phase * (1 - t) + (1 - phase) * t
            frame.set(i, density)
        return frame.render()


class VBlockBreathe(Effect):
    name = "vblock-breathe"
    description = "All columns breathe in and out in unison"

    def _render(self) -> list[str]:
        phase = self._frame % CYCLE_LENGTH.MEDIUM
        v = (math.sin(phase * math.pi / 10) + 1) / 2
        frame = VBlockFrame(WIDTH)
        for i in range(WIDTH):
            frame.set(i, v)
        return frame.render()


class VBlockBounce(Effect):
    name = "vblock-bounce"
    description = "Bright block bounces left to right with a fading trail"

    _PERIOD = (WIDTH - 1) * 2

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        t = self._frame % self._PERIOD
        pos = t if t < WIDTH else self._PERIOD - t
        for i in range(WIDTH):
            dist = abs(pos - i)
            if dist < 4:
                frame.set(i, max(0.0, 1.0 - dist * 0.3))
        return frame.render()


class VBlockPulse(Effect):
    name = "vblock-pulse"
    description = "Expanding ring radiates from center then contracts back"

    _CYCLE = WIDTH + 4

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        center = (WIDTH - 1) / 2
        age = self._frame % self._CYCLE
        intensity = min(1.0, age / (self._CYCLE * 0.6))
        for i in range(WIDTH):
            dist = abs(i - center)
            wave = max(0.0, 1.0 - (dist - age + 2) * 0.25)
            frame.set(i, wave * intensity)
        return frame.render()


class VBlockRipple(Effect):
    name = "vblock-ripple"
    description = "Concentric rings pulse outward from center"

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        cx = (WIDTH - 1) / 2
        for i in range(WIDTH):
            dist = abs(i - cx) / cx
            wave = (math.sin(dist * SPATIAL_FREQUENCY.EXTRA_DENSE - self._frame * TEMPORAL_SPEED.MODERATE) + 1) / 2
            density = max(0.0, wave - dist * 0.5)
            frame.set(i, density)
        return frame.render()


class VBlockRain(Effect):
    name = "vblock-rain"
    description = "Elements fall independently at their own pace"

    def __init__(self) -> None:
        super().__init__()
        self._levels: list[float] = [0.0] * WIDTH

    def _render(self) -> list[str]:
        for i in range(WIDTH):
            if random.random() < 0.12:
                self._levels[i] = min(1.0, self._levels[i] + random.uniform(0.5, 1.0))
            else:
                self._levels[i] *= 0.85
        frame = VBlockFrame(WIDTH)
        for i in range(WIDTH):
            frame.set(i, self._levels[i])
        return frame.render()


class VBlockEq(Effect):
    name = "vblock-eq"
    description = "Audio equalizer bars bouncing independently with gravity"

    def __init__(self) -> None:
        super().__init__()
        self._levels: list[float] = [random.uniform(0.2, 0.8) for _ in range(WIDTH)]
        self._velocity: list[float] = [0.0] * WIDTH

    def _render(self) -> list[str]:
        for i in range(WIDTH):
            if random.random() < 0.15:
                self._velocity[i] = random.uniform(0.08, 0.25)
            self._velocity[i] -= 0.02
            self._levels[i] += self._velocity[i]
            if self._levels[i] <= 0:
                self._levels[i] = 0.0
                self._velocity[i] = abs(self._velocity[i]) * 0.6
            elif self._levels[i] >= 1:
                self._levels[i] = 1.0
                self._velocity[i] = -abs(self._velocity[i]) * 0.6
        frame = VBlockFrame(WIDTH)
        for i in range(WIDTH):
            frame.set(i, self._levels[i])
        return frame.render()


class VBlockCascade(Effect):
    name = "vblock-cascade"
    description = "Full stream with empty gaps flowing rightward like water"

    _GAPS = 3
    _SPEED = TEMPORAL_SPEED.MODERATE
    _SPACING = WIDTH / _GAPS

    def _render(self) -> list[str]:
        frame = VBlockFrame(WIDTH)
        t = self._frame * self._SPEED
        for i in range(WIDTH):
            v = 1.0
            for g in range(self._GAPS):
                pos = (t + g * self._SPACING) % (WIDTH + 4) - 2
                wobble = 0.3 * math.sin(self._frame * TEMPORAL_SPEED.SLOW + g * 2.1)
                half = 1.0 + wobble
                dist = i - pos
                if abs(dist) < half:
                    v = min(v, max(0.0, (abs(dist) - half + 0.6) / 0.6))
            frame.set(i, v)
        return frame.render()
