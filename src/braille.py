BRAILLE = [chr(0x2800 + i) for i in range(256)]

DOT = {
    (0, 0): 0,
    (1, 0): 1,
    (2, 0): 2,
    (3, 0): 6,
    (0, 1): 3,
    (1, 1): 4,
    (2, 1): 5,
    (3, 1): 7,
}


def dot(row: int, col: int) -> int:
    return 1 << DOT[(row, col)]


def encode(bits: int) -> str:
    return BRAILLE[bits & 0xFF]


ALL_DOTS = sum(1 << d for d in DOT.values())
