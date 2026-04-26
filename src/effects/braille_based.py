import math
import random

from src.helpers.braille_helper import Frame
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

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for cx in range(0, WIDTH * 2, 4):
            for i in range(self.TRAIL):
                dx, dy = self.PATH[(self._frame + i) % len(self.PATH)]
                frame.set(cx + dx, dy)
        return frame.render()


class BrailleSpin2(Effect):
    name = "braille-spin2"
    description = "Braille spinner, same char repeated"

    PATH = [
        (0, 0), (0, 1), (0, 2), (0, 3),
        (1, 3), (2, 3), (3, 3), (4, 3),
        (5, 3), (5, 2), (5, 1), (5, 0),
        (4, 0), (3, 0), (2, 0), (1, 0),
    ]
    TRAIL = 4

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for cx in range(0, WIDTH * 2 - 4, 4):
            for i in range(self.TRAIL):
                dx, dy = self.PATH[(self._frame + i) % len(self.PATH)]
                frame.set(cx + dx, dy)
        return frame.render()


class BrailleWave(Effect):
    name = "braille-wave"
    description = "Braille wave, each col at different phase"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            phase = (self._frame + i) % 8
            y = phase if phase < 4 else 7 - phase
            frame.set(i * 2, y)
        return frame.render()


class BrailleRandom(Effect):
    name = "braille-random"
    description = "Braille random, each col picks random dots"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            val = random.getrandbits(8)
            for b in range(8):
                if val & (1 << b):
                    px, py = _PIXEL_ORDER[b]
                    frame.set(i * 2 + px, py)
        return frame.render()


class BrailleBreathe(Effect):
    name = "braille-breathe"
    description = "Braille breathe, grows from center then shrinks"

    def _render(self) -> list[str]:
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
        return frame.render()


class BrailleRipple(Effect):
    name = "braille-ripple"
    description = "Concentric ripple rings pulsing outward from center"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        cx, cy = WIDTH, 1.5
        for x in range(2 * WIDTH):
            for y in range(4):
                d = math.sqrt((x - cx) ** 2 + (y - cy) ** 2)
                wave = math.sin(d * 2 - self._frame * 0.5)
                if wave > 0.3:
                    frame.set(x, y)
        return frame.render()


class BrailleBounce(Effect):
    name = "braille-bounce"
    description = "Solid block with gradient edges shifts left to right then back"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        t = self._frame % (WIDTH * 4)
        center = t if t < (WIDTH * 2) else (WIDTH * 4) - t
        rng = random.Random(42)
        for x in range(WIDTH * 2):
            dist = abs(x - center)

            p = max(0, 1.1 - dist / 8)
            for y in range(4):
                if rng.random() < p:
                    frame.set(x, y)
        return frame.render()


class BrailleRain(Effect):
    name = "braille-rain"
    description = "Drops fall from top, each column at its own pace"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            speed = 1 + (i * 3 + 1) % 3
            offset = (i * 7) % 8
            drop_y = (self._frame * speed + offset) % 7 - 1
            for trail in range(3):
                y = drop_y - trail
                if 0 <= y < 4:
                    frame.set(i * 2 + (trail % 2), y)
        return frame.render()


class BrailleZigzag(Effect):
    name = "braille-zigzag"
    description = "Zigzag line sweeping diagonally across the grid"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for x in range(2 * WIDTH):
            phase = (x + self._frame) % 6
            y = phase if phase < 4 else 6 - phase
            frame.set(x, y)
        return frame.render()


class BrailleDissolve(Effect):
    name = "braille-dissolve"
    description = "All dots appear, then randomly dissolve and rebuild"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        total = 2 * WIDTH * 4
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
        return frame.render()


class BrailleFire(Effect):
    name = "braille-fire"
    description = "Flames rising from bottom with flicker"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        rng = random.Random(self._frame)
        for x in range(2 * WIDTH):
            for y in range(4):
                row_from_bottom = 3 - y
                decay = row_from_bottom * 0.28
                flicker = math.sin(self._frame * 0.7 + x * 1.3) * 0.2
                heat = max(0.0, 1.0 - decay + flicker)
                if rng.random() < heat:
                    frame.set(x, y)
        return frame.render()


class BrailleNoise(Effect):
    name = "braille-noise"
    description = "Organic lava-lamp blobs via noise field"

    @staticmethod
    def _noise2d(x: float, y: float) -> float:
        return (math.sin(x * 1.7 + y * 2.3) * 0.5
                + math.sin(x * 0.9 - y * 1.1) * 0.3
                + math.sin(x * 2.5 + y * 0.7) * 0.2)

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        t = self._frame * 0.12
        for x in range(2 * WIDTH):
            for y in range(4):
                nx = x * 0.35 + t
                ny = y * 0.5 + t * 0.3
                v = self._noise2d(nx, ny)
                if v > 0.1:
                    frame.set(x, y)
        return frame.render()


class BrailleHeartbeat(Effect):
    name = "braille-heartbeat"
    description = "ECG-style pulse line with a contemplative rhythm"

    _PATTERNS = [
        [0, 0, 0, 0, 1, 2, 1, 0, 2, 4, 3, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 2, 2, 3, 1, 0, 0, 0],
    ]
    _LEN = 18

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for x in range(2 * WIDTH):
            gi = self._frame - x
            beat = (gi // self._LEN) % len(self._PATTERNS)
            pat = self._PATTERNS[beat]
            i = gi % self._LEN
            y_off = pat[i]
            cy = 2
            for dy in range(-y_off, y_off + 1):
                ty = cy + dy
                if 0 <= ty < 4:
                    frame.set(x, ty)
        return frame.render()


class BrailleScanner(Effect):
    name = "braille-scanner"
    description = "Bright line sweeping left to right with trailing fade"

    _SCAN_WIDTH = 18
    _TRAIL = 8

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        rng = random.Random(self._frame)
        pos = self._frame % (self._SCAN_WIDTH + self._TRAIL + 4)
        for y in range(4):
            for dx in range(self._TRAIL + 1):
                x = pos - dx
                if 0 <= x < 2 * WIDTH:
                    if dx == 0:
                        frame.set(x, y)
                    elif rng.random() < 1.0 - dx / self._TRAIL:
                        frame.set(x, y)
        return frame.render()


class BrailleMatrix(Effect):
    name = "braille-matrix"
    description = "Digital rain with bright head and fading tail"

    _TAIL = 4

    def __init__(self):
        super().__init__()
        self._columns = [random.randint(-8, 0) for _ in range(2 * WIDTH)]
        self._speeds = [random.choice([1, 1, 2]) for _ in range(2 * WIDTH)]

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for i in range(2 * WIDTH):
            head = self._columns[i]
            for t in range(self._TAIL):
                y = head - t
                if 0 <= y < 4:
                    frame.set(i, y)
            self._columns[i] += self._speeds[i]
            if self._columns[i] - self._TAIL > 4:
                self._columns[i] = random.randint(-6, -1)
                self._speeds[i] = random.choice([1, 1, 2])
        return frame.render()


class BrailleArrow(Effect):
    name = "braille-arrow"
    description = "Chevrons slide right, reverse left, pause, repeat"

    _W = 8
    _SPEED = 2
    _PAUSE = 16

    _RIGHT = [
        (0, 0), (1, 0), (2, 0),                (5, 0), (6, 0), (7, 0),
            (1, 1), (2, 1), (3, 1),                (6, 1), (7, 1), (8, 1),
            (1, 2), (2, 2), (3, 2),                (6, 2), (7, 2), (8, 2),
        (0, 3), (1, 3), (2, 3),                (5, 3), (6, 3), (7, 3),
    ]
    _LEFT = [
        (1, 0), (2, 0), (3, 0),                (6, 0), (7, 0), (8, 0),
            (0, 1), (1, 1), (2, 1),                (5, 1), (6, 1), (7, 1),
            (0, 2), (1, 2), (2, 2),                (5, 2), (6, 2), (7, 2),
        (1, 3), (2, 3), (3, 3),                (6, 3), (7, 3), (8, 3),
    ]

    def __init__(self):
        super().__init__()
        self._travel = (2 * WIDTH + self._W) // self._SPEED + 1
        self._cycle = self._travel * 2 + self._PAUSE

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        f = self._frame % self._cycle
        if f < self._travel:
            base = -self._W + f * self._SPEED
            offsets = self._RIGHT
        elif f < self._travel * 2:
            base = 2 * WIDTH - (f - self._travel) * self._SPEED
            offsets = self._LEFT
        else:
            return frame.render()
        for dx, dy in offsets:
            x = base + dx
            if 0 <= x < 2 * WIDTH:
                frame.set(x, dy)
        return frame.render()


class BrailleCheckerboard(Effect):
    name = "braille-checkerboard"
    description = "Alternating single-dot checkerboard that shifts each frame"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        offset = (self._frame // 8) % 2
        for x in range(2 * WIDTH):
            for y in range(4):
                if (x + y + offset) % 2 == 0:
                    frame.set(x, y)
        return frame.render()


class BrailleCheckerboard2x2(Effect):
    name = "braille-checkerboard2x2"
    description = "2x2 block checkerboard that shifts each frame"

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        offset = (self._frame // 8) % 2
        for x in range(2 * WIDTH):
            for y in range(4):
                if ((x // 2 + y // 2) + offset) % 2 == 0:
                    frame.set(x, y)
        return frame.render()
