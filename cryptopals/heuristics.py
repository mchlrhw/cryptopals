def score_bytes_as_english(text: bytes) -> float:
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
