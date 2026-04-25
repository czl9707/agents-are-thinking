from abc import ABC, abstractmethod

WIDTH = 9
HEIGHT = 1


class Effect(ABC):
    name: str = ""
    description: str = ""

    def __init__(self):
        self._frame = 0

    @abstractmethod
    def step(self) -> list[str]:
        ...
