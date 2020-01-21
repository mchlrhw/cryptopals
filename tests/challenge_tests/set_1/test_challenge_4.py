import os

from cryptopals import single_byte_xor
from cryptopals.heuristics import looks_like_english
from cryptopals.serde import from_hex


def test_detect_single_byte_xor():
    test_data_path = os.path.join(os.path.dirname(__file__), "challenge-4-data.txt")
    with open(test_data_path) as test_data:
        lines = [l.strip() for l in test_data.readlines()]

    found = False
    for line in lines:
        if found:
            break
        for key in range(256):
            ciphertext = from_hex(line)
            plaintext = single_byte_xor(ciphertext, key)
            if looks_like_english(plaintext):
                found = True
                break

    assert plaintext == b"Now that the party is jumping\n"
