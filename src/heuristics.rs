use std::collections::HashSet;
use std::iter::FromIterator;

use lazy_static::lazy_static;

lazy_static! {
    static ref ETAOIN: HashSet<&'static u8> = HashSet::from_iter(b"etaoin");
    static ref SHRDLU: HashSet<&'static u8> = HashSet::from_iter(b"shrdlu");
    static ref OTHERS: HashSet<&'static u8> = HashSet::from_iter(b"bcfgjkmpqvwxyz .,");
}

pub(crate) fn score_bytes_as_english(text: &[u8]) -> f64 {
    let mut score = 0.0;

    if text.is_empty() {
        return -1.0;
    }

    for c in text {
        if ETAOIN.get(c).is_some() {
            score += 2.0;
            continue;
        } else if SHRDLU.get(c).is_some() {
            score += 1.0;
            continue;
        } else if OTHERS.get(c).is_some() {
            continue;
        } else {
            score -= 1.0;
        }
    }

    score / (2.0 * (text.len() as f64))
}
