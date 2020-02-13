import os

from cryptopals.serde import from_hex
from cryptopals.utils import break_single_byte_xor


def test_detect_single_byte_xor():
    test_data_path = os.path.join(os.path.dirname(__file__), "challenge-4-data.txt")
    with open(test_data_path) as test_data:
        lines = [from_hex(l.strip()) for l in test_data.readlines()]

    plaintext, _ = break_single_byte_xor(lines)

    assert plaintext == b"Now that the party is jumping\n"
