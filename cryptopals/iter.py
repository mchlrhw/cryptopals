from typing import Iterable, Optional


def blocks(
    text: bytes, blocksize: int, block_count: Optional[int] = None
) -> Iterable[bytes]:
    for n, i in enumerate(range(0, len(text), blocksize), start=1):
        if block_count and n > block_count:
            break
        block = text[i : i + blocksize]
        if len(block) != blocksize:
            continue
        yield block


def transpose(blocks: Iterable[bytes]) -> Iterable[bytes]:
    return (bytes(block) for block in zip(*blocks))
