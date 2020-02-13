import os
from typing import Tuple

from cryptopals.ciphers import repeating_key_xor
from cryptopals.heuristics import score_bytes_as_english
from cryptopals.iter import blocks, transpose
from cryptopals.serde import from_base64
from cryptopals.utils import break_single_byte_xor, get_keysize_candidates


def break_repeating_key_xor(ciphertext: bytes, keysize: int) -> Tuple[bytes, bytes]:
    key_parts = []
    for block in transpose(blocks(ciphertext, keysize)):
        res = break_single_byte_xor([block])
        _, key_part = res
        key_parts.append(key_part)

    key = bytes(key_parts)
    plaintext = repeating_key_xor(ciphertext, key)

    return key, plaintext


def test_break_repeating_key_xor():
    test_data_path = os.path.join(os.path.dirname(__file__), "challenge-6-data.txt")
    with open(test_data_path) as test_data:
        ciphertext_base64 = "".join(l.strip() for l in test_data.readlines())
    ciphertext = from_base64(ciphertext_base64)

    candidates = get_keysize_candidates(ciphertext, block_count=3, n_candidates=5)

    best_score = 0
    found_key, found_plaintext = b"", b""
    for keysize in candidates:
        key, plaintext = break_repeating_key_xor(ciphertext, keysize)
        score = score_bytes_as_english(plaintext)
        if score >= best_score:
            best_score = score
            found_key, found_plaintext = key, plaintext

    assert found_key == b"Terminator X: Bring the noise"
    assert found_plaintext == (
        b"I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly g"
        b"irls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin"
        b"' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on t"
        b"he mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my"
        b" mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's "
        b"to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I wil"
        b"l be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get "
        b"off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my ow"
        b"n phase \nThe girlies sa y they love me and that is ok \nAnd I can dance b"
        b"etter than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to "
        b"\nIt's off my head so let the beat play through \nSo I can funk it up and "
        b"make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I lik"
        b"e my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect"
        b" and that you can bet \nI can take a fly girl and make her wet. \n\nI'm lik"
        b"e Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \n"
        b"But you'll keep tryin' to get my style \nOver and over, practice makes pe"
        b"rfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no ti"
        b"me, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti w"
        b"ith a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin'"
        b" hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks st"
        b"op trying and girl stop cryin' \nVanilla Ice is sellin' and you people ar"
        b"e buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and "
        b"groovin' trying to sing along \nAll through the ghetto groovin' this here"
        b" song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a Ger"
        b"man Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on m"
        b"ine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanati"
        b"c \nYou trapped me once and I thought that \nYou might have it \nSo step do"
        b"wn and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're w"
        b"eakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I "
        b"can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong"
        b" to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and en"
        b"dure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close "
        b"and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thou"
        b"ght that I was weak, Boy, you're dead wrong \nSo come on, everybody and s"
        b"ing this song \n\nSay -- Play that funky music Say, go white boy, go white"
        b" boy go \nplay that funky music Go white boy, go white boy, go \nLay down "
        b"and boogie and play that funky music till you die. \n\nPlay that funky mus"
        b"ic Come on, Come on, let me hear \nPlay that funky music white boy you sa"
        b"y it, say it \nPlay that funky music A little louder now \nPlay that funky"
        b" music, white boy Come on, Come on, Come on \nPlay that funky music \n"
    )
