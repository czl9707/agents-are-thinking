import math
import random

from src.braille_helper import Frame
from src.effects.base import Effect, WIDTH, HEIGHT


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
        frame = Frame(WIDTH, HEIGHT)
        for cx in range(0, WIDTH * 2, 4):
            for i in range(self.TRAIL):
                dx, dy = self.PATH[(self._frame + i) % 8]
                frame.set(cx + dx, dy)
        self._frame += 1
        return frame.render()


class BrailleWave(Effect):
    name = "braille-wave"
    description = "Braille wave, each col at different phase"

    def step(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
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
        frame = Frame(WIDTH, HEIGHT)
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
        frame = Frame(WIDTH, HEIGHT)
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


class BrailleRipple(Effect):
    name = "braille-ripple"
    description = "Concentric ripple rings pulsing outward from center"

    def step(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        cx, cy = WIDTH, 1.5
        for x in range(2 * WIDTH):
            for y in range(4):
                d = math.sqrt((x - cx) ** 2 + (y - cy) ** 2)
                wave = math.sin(d * 2 - self._frame * 0.5)
                if wave > 0.3:
                    frame.set(x, y)
        self._frame += 1
        return frame.render()


class BrailleBounce(Effect):
    name = "braille-bounce"
    description = "Solid block with gradient edges shifts left to right then back"

    def __init__(self):
        super().__init__()

    def step(self) -> list[str]:
        self.b_frame = Frame(WIDTH, HEIGHT)
        t = self._frame % (WIDTH * 4)
        center = t if t < (WIDTH * 2) else (WIDTH * 4) - t
        rng = random.Random(42)
        for x in range(WIDTH * 2):
            dist = abs(x - center)

            p = max(0, 1.1 - dist / 8)
            for y in range(4):
                if rng.random() < p:
                    self.b_frame.set(x, y)

        self._frame += 1
        return self.b_frame.render()


class BrailleRain(Effect):
    name = "braille-rain"
    description = "Drops fall from top, each column at its own pace"

    def step(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            speed = 1 + (i * 3 + 1) % 3
            offset = (i * 7) % 8
            drop_y = (self._frame * speed + offset) % 7 - 1
            for trail in range(3):
                y = drop_y - trail
                if 0 <= y < 4:
                    frame.set(i * 2 + (trail % 2), y)
        self._frame += 1
        return frame.render()



class BrailleZigzag(Effect):
    name = "braille-zigzag"
    description = "Zigzag line sweeping diagonally across the grid"

    def step(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for x in range(2 * WIDTH):
            phase = (x + self._frame) % 6
            y = phase if phase < 4 else 6 - phase
            frame.set(x, y)
        self._frame += 1
        return frame.render()


class BrailleDissolve(Effect):
    name = "braille-dissolve"
    description = "All dots appear, then randomly dissolve and rebuild"

    def step(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        total = 72
        cycle = 36
        phase = self._frame % cycle
        half = cycle // 2
        rng = random.Random(42)
        order = list(range(total))
        rng.shuffle(order)
        if phase < half:
            remaining = total - (phase * total // half)
        else:
            remaining = (phase - half) * total // half
        for idx in order[: max(0, remaining)]:
            x = idx % (2 * WIDTH)
            y = idx // (2 * WIDTH)
            if y < 4:
                frame.set(x, y)
        self._frame += 1
        return frame.render()
