from abc import ABC, abstractmethod

WIDTH = 9
HEIGHT = 1


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
