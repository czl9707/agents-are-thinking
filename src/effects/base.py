from abc import ABC, abstractmethod

WIDTH = 9
HEIGHT = 1


class TEMPORAL_SPEED:
    CRAWL = 0.05
    SLOW = 0.12
    GENTLE = 0.25
    MODERATE = 0.4
    FAST = 0.7
    INTENSE = 1.3


class SPATIAL_FREQUENCY:
    LOW = 0.6
    HIGH = 1.2
    DENSE = 2.1
    EXTRA_DENSE = 6.0


class CYCLE_LENGTH:
    SHORT = 9
    MEDIUM = 18
    LONG = 36


class PAUSE:
    SHORT = 8
    MEDIUM = 12
    LONG = 16


class TRAIL:
    SHORT = 3
    LONG = 5
    EXTENDED = 8


class TOGGLE_RATE:
    SLOW = 8


class Effect(ABC):
    name: str = ""
    description: str = ""

    def __init__(self):
        self._frame = 0

    def step(self) -> list[str]:
        result = self._render()
        self._frame += 1
        return result

    @abstractmethod
    def _render(self) -> list[str]:
        ...
