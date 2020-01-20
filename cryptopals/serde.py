import base64
from typing import Union


def from_hex(string: str) -> bytes:
    return bytes.fromhex(string)


def to_hex(buf: Union[bytes, bytearray]) -> str:
    return buf.hex()


def from_base64(string: str) -> bytes:
    return base64.b64decode(string.encode())


def to_base64(buf: Union[bytes, bytearray]) -> str:
    return base64.b64encode(buf).decode()
