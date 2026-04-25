from src.effects.base import Effect
from src.effects.spinners import (
    BrailleSpin,
    BrailleWave,
    BrailleCascade,
    BrailleRandom,
    BrailleBreathe,
)
from src.effects.progress import BlockProgress

EFFECTS: list[type[Effect]] = [
    BrailleSpin,
    BrailleWave,
    BrailleCascade,
    BrailleRandom,
    BrailleBreathe,
    BlockProgress,
]
