from cryptopals.analysis import hamming_distance


def test_hamming_distance():
    text_a = b"this is a test"
    text_b = b"wokka wokka!!!"
    expected = 37

    distance = hamming_distance(text_a, text_b)

    assert distance == expected
