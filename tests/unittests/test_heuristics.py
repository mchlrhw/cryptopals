import pytest
from hypothesis import given
from hypothesis.strategies import binary

from cryptopals import score_bytes_as_english
from cryptopals.heuristics import InputTooShort, looks_like_english


def test_score_bytes_as_english_empty_input():
    assert score_bytes_as_english(b"") == -1.0


@pytest.mark.parametrize(
    "text",
    [
        b"Hello, World!",
        b"Cooking MC's like a pound of bacon",
        b"It was the best of times, it was the worst of times",
        b"Stoke me a clipper, I'll be back for Christmas.",
    ],
)
def test_looks_like_english_with_english(text):
    assert looks_like_english(text)


@given(binary(min_size=10))
def test_looks_like_english_with_random_bytes(text):
    assert not looks_like_english(text)


@given(binary(max_size=9))
def test_looks_like_english_input_too_short(text):
    with pytest.raises(InputTooShort):
        looks_like_english(text)
