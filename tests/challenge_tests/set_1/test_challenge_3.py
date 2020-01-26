from cryptopals.ciphers import single_byte_xor
from cryptopals.heuristics import score_bytes_as_english
from cryptopals.serde import from_hex


def test_decrypt_message():
    ciphertext_hex = (
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    )
    ciphertext = from_hex(ciphertext_hex)
    expected = b"Cooking MC's like a pound of bacon"

    results = dict()
    for key in range(256):
        plaintext = single_byte_xor(ciphertext, key)
        score = score_bytes_as_english(plaintext)
        results[score] = plaintext

    best = results.get(max(results))

    assert best == expected
