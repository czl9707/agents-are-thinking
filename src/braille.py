WIDTH = 9
HEIGHT = 1

_DOT_MAP = {
    (0, 0): 0, (1, 0): 1, (2, 0): 2, (3, 0): 6,
    (0, 1): 3, (1, 1): 4, (2, 1): 5, (3, 1): 7,
}
_BRAILLE = [chr(0x2800 + i) for i in range(256)]


class Frame:
    def __init__(self, width: int = WIDTH, height: int = HEIGHT):
        self.width = width
        self.height = height
        self._cells: list[int] = [0] * (width * height)

    def set(self, x: int, y: int):
        col = x // 2
        dot_col = x % 2
        row = y // 4
        dot_row = y % 4
        idx = row * self.width + col
        if 0 <= idx < len(self._cells):
            self._cells[idx] |= 1 << _DOT_MAP[(dot_row, dot_col)]

    def render(self) -> list[str]:
        rows = []
        for r in range(self.height):
            start = r * self.width
            rows.append("".join(
                _BRAILLE[self._cells[start + c] & 0xFF]
                for c in range(self.width)
            ))
        return rows
