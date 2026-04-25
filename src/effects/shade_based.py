import math

from src.shade_helper import ShadeFrame
from src.effects.base import Effect, WIDTH


class ShadeWave(Effect):
    name = "shade-wave"
    description = "Sine wave scrolls across with density gradient"

    def step(self) -> list[str]:
        frame = ShadeFrame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            v = (math.sin((i + self._frame) * 0.6) + 1) / 2
            frame.set(i, 0, v)
        self._frame += 1
        return frame.render()


class ShadeScanner(Effect):
    name = "shade-scanner"
    description = "Bright line sweeps with smooth falloff trail"

    _SCAN_RANGE = WIDTH + 6

    def step(self) -> list[str]:
        frame = ShadeFrame(WIDTH, HEIGHT)
        pos = self._frame % self._SCAN_RANGE
        for i in range(WIDTH):
            dist = abs(i - pos)
            density = max(0.0, 1.0 - dist / 4)
            frame.set(i, 0, density)
        self._frame += 1
        return frame.render()


class ShadeFire(Effect):
    name = "shade-fire"
    description = "Flames with heat gradient fading upward"

    def step(self) -> list[str]:
        frame = ShadeFrame(WIDTH, HEIGHT)
        for x in range(WIDTH):
            flicker = math.sin(self._frame * 0.7 + x * 1.3) * 0.15
            heat = max(0.0, min(1.0, 1.0 + flicker))
            frame.set(x, 0, heat)
        self._frame += 1
        return frame.render()


class ShadePulse(Effect):
    name = "shade-pulse"
    description = "Concentric rings radiating from center"

    def step(self) -> list[str]:
        frame = ShadeFrame(WIDTH, HEIGHT)
        cx = WIDTH / 2
        for i in range(WIDTH):
            dist = abs(i - cx) / cx
            wave = (math.sin(dist * 6 - self._frame * 0.4) + 1) / 2
            density = max(0.0, wave - dist * 0.5)
            frame.set(i, 0, density)
        self._frame += 1
        return frame.render()


class ShadeBreathe(Effect):
    name = "shade-breathe"
    description = "Entire bar inhales and exhales"

    def step(self) -> list[str]:
        phase = self._frame % 20
        v = (math.sin(phase * math.pi / 10) + 1) / 2
        frame = ShadeFrame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            frame.set(i, 0, v)
        self._frame += 1
        return frame.render()
