from src.effects.base import Effect
from src.effects.spinners import BrailleSpinner
from src.effects.progress import BlockProgress

EFFECTS: list[type[Effect]] = [
    BrailleSpinner,
    BlockProgress,
]
