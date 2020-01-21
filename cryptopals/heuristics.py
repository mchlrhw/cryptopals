class InputTooShort(Exception):
    ...


def score_bytes_as_english(text: bytes) -> float:
    if not text:
        return -1.0

    etaoin = set(b"etaoin")
    shrdlu = set(b"shrdlu")
    others = set(b"bcfgjkmpqvwxyz .,")

    score = 0
    for char in text:
        if char in etaoin:
            score += 2
        elif char in shrdlu:
            score += 1
        elif char in others:
            continue
        else:
            score -= 1

    return score / (2 * len(text))


def looks_like_english(text: bytes, threshold: float = 0.3) -> bool:
    if len(text) < 10:
        raise InputTooShort

    score = score_bytes_as_english(text)
    if score > threshold:
        return True
    return False
