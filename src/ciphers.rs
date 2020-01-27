pub(crate) fn fixed_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext
        .iter()
        .zip(key.iter())
        .map(|(p, k)| p ^ k)
        .collect()
}

pub(crate) fn single_byte_xor(plaintext: &[u8], key: u8) -> Vec<u8> {
    plaintext.iter().map(|p| p ^ key).collect()
}

pub(crate) fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext
        .iter()
        .zip(key.iter().cycle())
        .map(|(p, k)| p ^ k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let plaintext = [1, 2, 3, 4, 5, 6];
        let key = [1, 1, 1, 1, 1, 1];
        let expected = [0, 3, 2, 5, 4, 7];

        let ciphertext = fixed_xor(&plaintext, &key);

        assert_eq!(ciphertext, expected);
    }

    #[test]
    fn test_single_byte_xor() {
        let plaintext = [1, 2, 3, 4, 5, 6];
        let key = 1;
        let expected = [0, 3, 2, 5, 4, 7];

        let ciphertext = single_byte_xor(&plaintext, key);

        assert_eq!(ciphertext, expected);
    }

    #[test]
    fn test_repeating_key_xor() {
        let plaintext = [1, 2, 3, 4, 5, 6];
        let key = [1, 1];
        let expected = [0, 3, 2, 5, 4, 7];

        let ciphertext = repeating_key_xor(&plaintext, &key);

        assert_eq!(ciphertext, expected);
    }
}
