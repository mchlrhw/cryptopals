from cryptopals.ciphers import fixed_xor
from cryptopals.serde import from_hex, to_hex


def test_fixed_xor():
    plaintext_hex = "1c0111001f010100061a024b53535009181c"
    key_hex = "686974207468652062756c6c277320657965"

    plaintext = from_hex(plaintext_hex)
    key = from_hex(key_hex)
    expected = "746865206b696420646f6e277420706c6179"

    ciphertext = to_hex(fixed_xor(plaintext, key))

    assert ciphertext == expected
