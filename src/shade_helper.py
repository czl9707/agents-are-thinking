from src.effects.base import WIDTH, HEIGHT

_SHADES = "░▒▓█"


class ShadeFrame:
    def __init__(self, width: int = WIDTH, height: int = HEIGHT) -> None:
        self.width = width
        self.height = height
        self._cells: list[list[float]] = [[0.0] * width for _ in range(height)]

    def set(self, x: int, y: int, density: float = 1.0) -> None:
        if 0 <= x < self.width and 0 <= y < self.height:
            self._cells[y][x] = max(0.0, min(1.0, density))

    def add(self, x: int, y: int, density: float = 1.0) -> None:
        if 0 <= x < self.width and 0 <= y < self.height:
            self._cells[y][x] = max(0.0, min(1.0, self._cells[y][x] + density))

    def clear(self) -> None:
        self._cells = [[0.0] * self.width for _ in range(self.height)]

    def render(self) -> list[str]:
        rows = []
        for r in range(self.height):
            row = []
            for c in range(self.width):
                idx = min(int(self._cells[r][c] * len(_SHADES)), len(_SHADES) - 1)
                row.append(_SHADES[idx])
            rows.append("".join(row))
        return rows
