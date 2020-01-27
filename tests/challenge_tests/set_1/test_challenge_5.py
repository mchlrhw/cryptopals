from cryptopals.ciphers import repeating_key_xor
from cryptopals.serde import to_hex


def test_repeating_key_xor():
    plaintext = b"""Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"""
    key = b"ICE"
    expected = (
        "0b3637272a2b2e63622c2e69692a2369"
        "3a2a3c6324202d623d63343c2a262263"
        "24272765272a282b2f20430a652e2c65"
        "2a3124333a653e2b2027630c692b2028"
        "3165286326302e27282f"
    )

    ciphertext = repeating_key_xor(plaintext, key)
    ciphertext_hex = to_hex(ciphertext)

    assert ciphertext_hex == expected
