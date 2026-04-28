_DEFAULT_W = 9

_SHADES = "░▒▓█"


class ShadeFrame:
    def __init__(self, width: int = _DEFAULT_W) -> None:
        self.width = width
        self._cells: list[float] = [0.0] * width

    def set(self, x: int, density: float = 1.0) -> None:
        if 0 <= x < self.width:
            self._cells[x] = max(0.0, min(1.0, density))

    def add(self, x: int, density: float = 1.0) -> None:
        if 0 <= x < self.width:
            self._cells[x] = max(0.0, min(1.0, self._cells[x] + density))

    def clear(self) -> None:
        self._cells = [0.0] * self.width

    def render(self) -> list[str]:
        row = []
        for c in range(self.width):
            idx = min(round(self._cells[c] * (len(_SHADES) - 1)), len(_SHADES) - 1)
            row.append(_SHADES[idx])
        return ["".join(row)]
