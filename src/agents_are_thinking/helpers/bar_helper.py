_DEFAULT_W = 9

_BARS = " ▁▂▃▄▅▆▇█"


class BarFrame:
    def __init__(self, width: int = _DEFAULT_W) -> None:
        self.width = width
        self._cells: list[float] = [0.0] * width

    def set(self, x: int, level: float = 1.0) -> None:
        if 0 <= x < self.width:
            self._cells[x] = max(0.0, min(1.0, level))

    def clear(self) -> None:
        self._cells = [0.0] * self.width

    def render(self) -> list[str]:
        row = []
        for c in range(self.width):
            idx = min(round(self._cells[c] * (len(_BARS) - 1)), len(_BARS) - 1)
            row.append(_BARS[idx])
        return ["".join(row)]
