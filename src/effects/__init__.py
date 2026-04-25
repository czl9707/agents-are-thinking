from src.effects.base import Effect
from src.effects.braille_based import (
    BrailleSpin,
    BrailleSpin2,
    BrailleWave,
    BrailleRandom,
    BrailleBreathe,
    BrailleRipple,
    BrailleBounce,
    BrailleRain,
    BrailleZigzag,
    BrailleDissolve,
    BrailleFire,
    BrailleNoise,
    BrailleHeartbeat,
    BrailleArrow,
    BrailleScanner,
    BrailleMatrix,
)
from src.effects.shade_based import (
    ShadeWave,
    ShadeScanner,
    ShadeFire,
    ShadeNoise,
    ShadePulse,
    ShadeBreathe,
)
from src.effects.bar_based import (
    BarEqualizer,
    BarWave,
    BarRain,
)
from src.effects.progress import BlockProgress

EFFECTS: list[type[Effect]] = [
    # braille
    BrailleSpin,
    BrailleSpin2,
    BrailleWave,
    BrailleRandom,
    BrailleBreathe,
    BrailleRipple,
    BrailleBounce,
    BrailleRain,
    BrailleZigzag,
    BrailleDissolve,
    BrailleFire,
    BrailleNoise,
    BrailleHeartbeat,
    BrailleArrow,
    BrailleScanner,
    BrailleMatrix,
    # shade
    ShadeWave,
    ShadeScanner,
    ShadeFire,
    ShadeNoise,
    ShadePulse,
    ShadeBreathe,
    # bar
    BarEqualizer,
    BarWave,
    BarRain,
    # block progress
    BlockProgress,
]
