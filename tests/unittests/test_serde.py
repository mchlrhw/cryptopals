from hypothesis import given
from hypothesis.strategies import sampled_from, text

from cryptopals.serde import from_base64, from_hex, to_base64, to_hex

base64_string = text(
    alphabet=sampled_from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    )
).filter(lambda s: len(s) % 4 == 0)

hex_string = text(alphabet=sampled_from("0123456789abcdef")).filter(
    lambda s: len(s) % 2 == 0
)


@given(hex_string)
def test_hex_round_trip(expected):
    deserialized = from_hex(expected)
    serialized = to_hex(deserialized)

    assert serialized == expected


@given(base64_string)
def test_base64_round_trip(expected):
    deserialized = from_base64(expected)
    serialized = to_base64(deserialized)

    assert serialized == expected
