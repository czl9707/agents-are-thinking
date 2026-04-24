from src.effects.base import Effect


class BrailleSpinner(Effect):
    name = "braille"
    description = "Braille dot spinner"

    FRAMES = ["\u2807", "\u2819", "\u2839", "\u2838", "\u283c", "\u2834", "\u2826", "\u2827", "\u2807", "\u280f"]

    def tick(self, frame: int) -> list[str]:
        char = self.FRAMES[frame % len(self.FRAMES)]
        lines = [""] * self.height
        mid = self.height // 2
        lines[mid] = f"  {char} Thinking..."
        return lines
