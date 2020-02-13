from .rust import fixed_xor


def count_1s(n: int) -> int:
    return bin(n).count("1")


def hamming_distance(text_a: bytes, text_b: bytes) -> int:
    diff = fixed_xor(text_a, text_b)
    return sum(count_1s(b) for b in diff)
