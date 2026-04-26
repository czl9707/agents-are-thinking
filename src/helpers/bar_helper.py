_DEFAULT_W = 9
_DEFAULT_H = 1

_BARS = " ▁▂▃▄▅▆▇█"


class BarFrame:
    def __init__(self, width: int = _DEFAULT_W, height: int = _DEFAULT_H) -> None:
        self.width = width
        self.height = height
        self._cells: list[list[float]] = [[0.0] * width for _ in range(height)]

    def set(self, x: int, y: int, level: float = 1.0) -> None:
        if 0 <= x < self.width and 0 <= y < self.height:
            self._cells[y][x] = max(0.0, min(1.0, level))

    def clear(self) -> None:
        self._cells = [[0.0] * self.width for _ in range(self.height)]

    def render(self) -> list[str]:
        rows = []
        for r in range(self.height):
            row = []
            for c in range(self.width):
                idx = min(int(self._cells[r][c] * (len(_BARS) - 1)), len(_BARS) - 1)
                row.append(_BARS[idx])
            rows.append("".join(row))
        return rows
