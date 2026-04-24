from abc import ABC, abstractmethod


class Effect(ABC):
    name: str = ""
    description: str = ""

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    @abstractmethod
    def tick(self, frame: int) -> list[str]:
        ...
