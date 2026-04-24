from src.effects.base import Effect
from src.effects.spinners import BrailleSpinner

EFFECTS: list[type[Effect]] = [
    BrailleSpinner,
]
