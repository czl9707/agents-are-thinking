from src.braille import dot, encode, ALL_DOTS
from src.effects.base import Effect, WIDTH


class BrailleSpin(Effect):
    name = "braille-spin"
    description = "Braille spinner, same char repeated"

    PATH = [
        (0, 0), (1, 0), (2, 0), (3, 0),
        (3, 1), (2, 1), (1, 1), (0, 1),
    ]
    TRAIL = 3

    def step(self) -> list[str]:
        bits = 0
        for i in range(self.TRAIL):
            r, c = self.PATH[(self._frame + i) % len(self.PATH)]
            bits |= dot(r, c)
        self._frame += 1
        char = encode(bits)
        return [" ".join(char * ((WIDTH + 1) // 2))]


class BrailleWave(Effect):
    name = "braille-wave"
    description = "Braille wave, each col at different phase"

    FRAMES = [
        dot(0, 0),
        dot(1, 0),
        dot(2, 0),
        dot(3, 0),
        dot(3, 1),
        dot(2, 1),
        dot(1, 1),
        dot(0, 1),
    ]

    def step(self) -> list[str]:
        cols = []
        for i in range(WIDTH):
            idx = (self._frame + i) % len(self.FRAMES)
            cols.append(encode(self.FRAMES[idx]))
        self._frame += 1
        return ["".join(cols)]


class BrailleCascade(Effect):
    name = "braille-cascade"
    description = "Braille cascade, fills left to right then resets"

    def step(self) -> list[str]:
        idx = self._frame % (WIDTH + 1)
        cols = []
        for i in range(WIDTH):
            if i < idx:
                progress = (self._frame + i * 3) % 8
                bits = 0
                for b in range(8):
                    if b < progress:
                        bits |= 1 << b
                cols.append(encode(bits))
            else:
                cols.append(" ")
        self._frame += 1
        return ["".join(cols)]


class BrailleRandom(Effect):
    name = "braille-random"
    description = "Braille random, each col picks pseudo-random dots"

    def step(self) -> list[str]:
        cols = []
        for i in range(WIDTH):
            seed = (self._frame * 7 + i * 13) % 256
            cols.append(encode(seed))
        self._frame += 1
        return ["".join(cols)]


class BrailleBreathe(Effect):
    name = "braille-breathe"
    description = "Braille breathe, grows from center then shrinks"

    def step(self) -> list[str]:
        phase = self._frame % 10
        cols = []
        for i in range(WIDTH):
            dist = abs(i - WIDTH // 2)
            ci = (phase + dist) % 10
            if ci < 5:
                bits = ALL_DOTS * ci // 5
            else:
                bits = ALL_DOTS * (10 - ci) // 5
            cols.append(encode(bits))
        self._frame += 1
        return ["".join(cols)]
