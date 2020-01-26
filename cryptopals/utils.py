from typing import List, Optional

from .rust import detect_single_byte_xor as _detect_single_byte_xor


def detect_single_byte_xor(lines: List[bytes]) -> Optional[bytes]:
    result = _detect_single_byte_xor(lines)
    if result:
        return bytes(result)
    return None
