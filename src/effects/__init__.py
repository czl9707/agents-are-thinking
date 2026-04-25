from src.effects.base import Effect
from src.effects.braille_based import (
    BrailleSpin,
    BrailleWave,
    BrailleRandom,
    BrailleBreathe,
    BrailleRipple,
    BrailleBounce,
    BrailleRain,
    BrailleZigzag,
    BrailleDissolve,
)
from src.effects.progress import BlockProgress

EFFECTS: list[type[Effect]] = [
    BrailleSpin,
    BrailleWave,
    BrailleRandom,
    BrailleBreathe,
    BrailleRipple,
    BrailleBounce,
    BrailleRain,
    BrailleZigzag,
    BrailleDissolve,
    BlockProgress,
]
