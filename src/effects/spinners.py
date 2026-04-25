import random

from src.braille import Frame
from src.effects.base import Effect, WIDTH


_PIXEL_ORDER = [
    (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (0, 3), (1, 3),
]


class BrailleSpin(Effect):
    name = "braille-spin"
    description = "Braille spinner, same char repeated"

    PATH = [
        (0, 0), (0, 1), (0, 2), (0, 3),
        (1, 3), (1, 2), (1, 1), (1, 0),
    ]
    TRAIL = 3

    def step(self) -> list[str]:
        frame = Frame()
        for cx in range(0, WIDTH * 2, 2):
            for i in range(self.TRAIL):
                dx, dy = self.PATH[(self._frame + i) % 8]
                frame.set(cx + dx, dy)
        self._frame += 1
        return frame.render()


class BrailleWave(Effect):
    name = "braille-wave"
    description = "Braille wave, each col at different phase"

    def step(self) -> list[str]:
        frame = Frame()
        for i in range(WIDTH):
            phase = (self._frame + i) % 8
            y = phase if phase < 4 else 7 - phase
            frame.set(i * 2, y)
        self._frame += 1
        return frame.render()
        

class BrailleRandom(Effect):
    name = "braille-random"
    description = "Braille random, each col picks random dots"

    def step(self) -> list[str]:
        frame = Frame()
        for i in range(WIDTH):
            val = random.getrandbits(8)
            for b in range(8):
                if val & (1 << b):
                    px, py = _PIXEL_ORDER[b]
                    frame.set(i * 2 + px, py)
        self._frame += 1
        return frame.render()


class BrailleBreathe(Effect):
    name = "braille-breathe"
    description = "Braille breathe, grows from center then shrinks"

    def step(self) -> list[str]:
        phase = self._frame % 10
        frame = Frame()
        for i in range(WIDTH):
            dist = abs(i - WIDTH // 2)
            ci = (phase + dist) % 10
            count = ci if ci < 5 else 10 - ci
            n = count * 8 // 5
            for b in range(n):
                px, py = _PIXEL_ORDER[b]
                frame.set(i * 2 + px, py)
        self._frame += 1
        return frame.render()
