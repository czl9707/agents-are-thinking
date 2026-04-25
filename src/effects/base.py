from abc import ABC, abstractmethod

from src.braille import WIDTH


class Effect(ABC):
    name: str = ""
    description: str = ""

    def __init__(self):
        self._frame = 0

    @abstractmethod
    def step(self) -> list[str]:
        ...
