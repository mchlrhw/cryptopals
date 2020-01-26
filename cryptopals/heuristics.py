from .rust import score_bytes_as_english


class InputTooShort(Exception):
    ...


def looks_like_english(text: bytes, threshold: float = 0.3) -> bool:
    if len(text) < 10:
        raise InputTooShort

    score = score_bytes_as_english(text)
    if score > threshold:
        return True
    return False
