import pytest

from cryptopals.iter import blocks, transpose


@pytest.mark.parametrize(
    "blocksize, block_count, expected",
    [
        (2, None, [b"ab", b"cd", b"ef", b"gh", b"ij", b"kl"]),
        (2, 5, [b"ab", b"cd", b"ef", b"gh", b"ij"]),
        (3, None, [b"abc", b"def", b"ghi", b"jkl"]),
        (4, None, [b"abcd", b"efgh", b"ijkl"]),
        (5, None, [b"abcde", b"fghij"]),
    ],
)
def test_blocks(blocksize, block_count, expected):
    text = b"abcdefghijkl"

    block_list = list(blocks(text, blocksize, block_count))

    assert block_list == expected


@pytest.mark.parametrize(
    "blocksize, expected",
    [
        (2, [b"acegik", b"bdfhjl"]),
        (3, [b"adgj", b"behk", b"cfil"]),
        (4, [b"aei", b"bfj", b"cgk", b"dhl"]),
        (5, [b"af", b"bg", b"ch", b"di", b"ej"]),
    ],
)
def test_transpose(blocksize, expected):
    text = b"abcdefghijkl"
    transposed = transpose(blocks(text, blocksize))

    assert list(transposed) == expected
