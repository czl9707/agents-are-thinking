from src.effects.base import Effect, WIDTH


class BlockProgress(Effect):
    name = "block"
    description = "Block fill progress"

    FILLED = "█"
    EMPTY = "░"
    TOTAL_FRAMES = 100

    def step(self) -> list[str]:
        progress = self._frame % (self.TOTAL_FRAMES + 1)
        pct = progress / self.TOTAL_FRAMES
        filled = int(WIDTH * pct)
        bar = self.FILLED * filled + self.EMPTY * (WIDTH - filled)
        self._frame += 1
        return [bar]
