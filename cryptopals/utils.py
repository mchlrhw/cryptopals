from itertools import combinations
from operator import itemgetter
from typing import List, Tuple

from .analysis import hamming_distance
from .iter import blocks
from .rust import break_single_byte_xor as _break_single_byte_xor


def break_single_byte_xor(lines: List[bytes]) -> Tuple[bytes, int]:
    result = _break_single_byte_xor(lines)
    # TODO: Treat this the same as the cipher functions
    #       i.e. create the bytes object in Rust
    return bytes(result[0]), result[1]


def get_keysize_candidates(
    ciphertext: bytes, block_count: int, n_candidates: int
) -> List[int]:

    distances = dict()
    for keysize in range(2, 41):

        total_distance: float = 0
        for (block_a, block_b) in combinations(
            blocks(ciphertext, keysize, block_count), 2
        ):
            distance = hamming_distance(block_a, block_b)
            norm_distance = distance / keysize
            total_distance += norm_distance

        average_distance = total_distance / block_count
        distances[keysize] = average_distance

    top_n_candidates = list(sorted(distances.items(), key=itemgetter(1)))[:n_candidates]

    return [c for (c, _) in top_n_candidates]
