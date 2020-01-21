from .cryptopals import fixed_xor as _fixed_xor
from .cryptopals import single_byte_xor as _single_byte_xor


# TODO: Find a way of not having to wrap in bytes
#       i.e. allocate and return the bytes from Rust
def fixed_xor(*args, **kwargs) -> bytes:
    return bytes(_fixed_xor(*args, **kwargs))


# TODO: Find a way of not having to wrap in bytes
#       i.e. allocate and return the bytes from Rust
def single_byte_xor(*args, **kwargs) -> bytes:
    return bytes(_single_byte_xor(*args, **kwargs))
