import math
import random

from agents_are_thinking.helpers.braille_helper import Frame
from agents_are_thinking.effects.base import (
    Effect, WIDTH, HEIGHT,
    TEMPORAL_SPEED, SPATIAL_FREQUENCY, CYCLE_LENGTH, PAUSE, TRAIL, TOGGLE_RATE,
)


_PIXEL_ORDER = [
    (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (0, 3), (1, 3),
]


class BrailleSpin(Effect):
    name = "braille-spin"
    description = "Braille spinner, same char repeated"

    cycle_length = 8
    PATH = [
        (0, 0), (0, 1), (0, 2), (0, 3),
        (1, 3), (1, 2), (1, 1), (1, 0),
    ]
    _TRAIL = TRAIL.SHORT

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for cx in range(0, WIDTH * 2, 4):
            for i in range(self._TRAIL):
                dx, dy = self.PATH[(self._frame + i) % len(self.PATH)]
                frame.set(cx + dx, dy)
        return frame.render()


class BrailleSpin2(Effect):
    name = "braille-spin2"
    description = "Braille spinner, same char repeated"

    cycle_length = 16
    PATH = [
        (0, 0), (0, 1), (0, 2), (0, 3),
        (1, 3), (2, 3), (3, 3), (4, 3),
        (5, 3), (5, 2), (5, 1), (5, 0),
        (4, 0), (3, 0), (2, 0), (1, 0),
    ]
    _TRAIL = TRAIL.SHORT

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        for cx in range(0, WIDTH * 2 - 4, 4):
            for i in range(self._TRAIL):
                dx, dy = self.PATH[(self._frame + i) % len(self.PATH)]
                frame.set(cx + dx, dy)
        return frame.render()


class BrailleWave(Effect):
    name = "braille-wave"
    description = "Smooth sine wave scrolls across as braille dots"

    cycle_length = 8

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

    cycle_length = 36

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        rng = random.Random(self._frame)
        for i in range(WIDTH):
            val = rng.getrandbits(8)
            for b in range(8):
                if val & (1 << b):
                    px, py = _PIXEL_ORDER[b]
                    frame.set(i * 2 + px, py)
        return frame.render()


class BrailleBreathe(Effect):
    name = "braille-breathe"
    description = "Dots breathe outward from center then inward"

    cycle_length = 9

    def _render(self) -> list[str]:
        phase = self._frame % CYCLE_LENGTH.SHORT
        frame = Frame(WIDTH, HEIGHT)
        for i in range(WIDTH):
            dist = abs(i - WIDTH // 2)
            ci = (phase + dist) % CYCLE_LENGTH.SHORT
            count = ci if ci < 5 else CYCLE_LENGTH.SHORT - ci
            n = count * 8 // 5
            for b in range(n):
                px, py = _PIXEL_ORDER[b]
                frame.set(i * 2 + px, py)
        return frame.render()


class BrailleRipple(Effect):
    name = "braille-ripple"
    description = "Concentric rings pulse outward from center"

    cycle_length = 9

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        cx, cy = WIDTH, 1.5
        for x in range(2 * WIDTH):
            for y in range(4):
                d = math.sqrt((x - cx) ** 2 + (y - cy) ** 2)
                wave = math.sin(d * 2 - self._frame * TEMPORAL_SPEED.FAST)
                if wave > 0.3:
                    frame.set(x, y)
        return frame.render()


class BrailleBounce(Effect):
    name = "braille-bounce"
    description = "Solid block bounces left to right with a fading trail"

    cycle_length = 36

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
    description = "Elements fall independently at their own pace"

    cycle_length = 7

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

    cycle_length = 6

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

    cycle_length = 36

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        total = 2 * WIDTH * 4
        cycle = CYCLE_LENGTH.LONG
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
    description = "FASTing flames rising from the bottom"

    cycle_length = 9

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        rng = random.Random(self._frame)
        for x in range(2 * WIDTH):
            for y in range(4):
                row_from_bottom = 3 - y
                decay = row_from_bottom * 0.28
                FAST = math.sin(self._frame * TEMPORAL_SPEED.FAST + x * SPATIAL_FREQUENCY.HIGH) * 0.2
                heat = max(0.0, 1.0 - decay + FAST)
                if rng.random() < heat:
                    frame.set(x, y)
        return frame.render()


class BrailleNoise(Effect):
    name = "braille-noise"
    description = "Organic lava-lamp blobs via noise field"

    cycle_length = 36

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        phase = math.tau * self._frame / self.cycle_length
        for x in range(2 * WIDTH):
            for y in range(4):
                nx = x * SPATIAL_FREQUENCY.LOW
                ny = y * 0.5
                v = (math.sin(nx * 2 + ny * 2 + phase * 2) * 0.5
                    + math.sin(nx - ny - phase) * 0.3
                    + math.sin(nx * 2 + ny + phase * 3) * 0.2)
                if v > 0.1:
                    frame.set(x, y)
        return frame.render()


class BrailleHeartbeat(Effect):
    name = "braille-heartbeat"
    description = "Pulsing rhythm that mimics a heartbeat"

    cycle_length = 36

    _PATTERNS = [
        [0, 0, 0, 0, 1, 2, 1, 0, 2, 4, 3, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 2, 2, 3, 1, 0, 0, 0],
    ]
    _LEN = CYCLE_LENGTH.MEDIUM

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
    description = "Bright line sweeps across with a fading trail"

    cycle_length = 30

    _SCAN_WIDTH = CYCLE_LENGTH.MEDIUM
    _TRAIL = TRAIL.EXTENDED

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

    _TAIL = TRAIL.SHORT
    cycle_length = 36

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        rng = random.Random(42)
        for i in range(2 * WIDTH):
            speed = rng.choice([1, 1, 2])
            init_offset = rng.randint(-8, 0)
            span = 4 + self._TAIL + abs(init_offset)
            steps = self._frame * speed
            head = init_offset + steps % span
            for t in range(self._TAIL):
                y = head - t
                if 0 <= y < 4:
                    frame.set(i, y)
        return frame.render()


class BrailleArrow(Effect):
    name = "braille-arrow"
    description = "Shape slides right, reverses left, pauses, repeats"

    cycle_length = 44

    _W = 8
    _SPEED = 2
    _PAUSE = PAUSE.LONG

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

    cycle_length = 16

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        offset = (self._frame // TOGGLE_RATE.SLOW) % 2
        for x in range(2 * WIDTH):
            for y in range(4):
                if (x + y + offset) % 2 == 0:
                    frame.set(x, y)
        return frame.render()


class BrailleCheckerboard2x2(Effect):
    name = "braille-checkerboard2x2"
    description = "2x2 block checkerboard that shifts each frame"

    cycle_length = 16

    def _render(self) -> list[str]:
        frame = Frame(WIDTH, HEIGHT)
        offset = (self._frame // TOGGLE_RATE.SLOW) % 2
        for x in range(2 * WIDTH):
            for y in range(4):
                if ((x // 2 + y // 2) + offset) % 2 == 0:
                    frame.set(x, y)
        return frame.render()
