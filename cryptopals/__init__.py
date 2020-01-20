from .cryptopals import fixed_xor as _fixed_xor


# TODO: Find a way of not having to wrap in bytes
#       i.e. allocate and return the bytes from Rust
def fixed_xor(*args, **kwargs):
    return bytes(_fixed_xor(*args, **kwargs))
