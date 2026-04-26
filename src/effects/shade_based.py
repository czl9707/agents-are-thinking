import math

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
