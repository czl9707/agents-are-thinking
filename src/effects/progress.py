from src.effects.base import Effect


class BlockProgress(Effect):
    name = "progress"
    description = "Block progress bar with spinner"

    SPINNER = ["\u2807", "\u2819", "\u2839", "\u2838", "\u283c", "\u2834", "\u2826", "\u2827", "\u2807", "\u280f"]
    FILLED = "\u2588"
    EMPTY = "\u2591"
    TOTAL_FRAMES = 100

    def tick(self, frame: int) -> list[str]:
        progress = frame % (self.TOTAL_FRAMES + 1)
        pct = progress / self.TOTAL_FRAMES
        bar_width = self.width - 8
        filled = int(bar_width * pct)
        bar = self.FILLED * filled + self.EMPTY * (bar_width - filled)
        spinner = self.SPINNER[frame % len(self.SPINNER)]
        lines = [""] * self.height
        mid = self.height // 2
        lines[mid] = f"  {spinner} [{bar}] {int(pct * 100):3d}%"
        return lines
