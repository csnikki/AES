use crate::def::*;
use crate::pass::*;

// Globals

const RCON: [u8; 10] = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tables/rcon.tbl"));

// Helpers

fn words_for_mode(mode: Mode) -> usize {
    match mode {
        Mode::ECB_128 | Mode::CBC_128 | Mode::CTR_128 => 4,
        Mode::ECB_192 | Mode::CBC_192 | Mode::CTR_192 => 6,
        Mode::ECB_256 | Mode::CBC_256 | Mode::CTR_256 => 8,
    }
}

fn get_word(buffer: &[u8], index: usize) -> Word {
    [
        buffer[index * WORD_BYTES],
        buffer[(index * WORD_BYTES) + 1],
        buffer[(index * WORD_BYTES) + 2],
        buffer[(index * WORD_BYTES) + 3],
    ]
}

fn set_word(buffer: &mut [u8], index: usize, w: &Word) {
    buffer[index * WORD_BYTES] = w[0];
    buffer[(index * WORD_BYTES) + 1] = w[1];
    buffer[(index * WORD_BYTES) + 2] = w[2];
    buffer[(index * WORD_BYTES) + 3] = w[3];
}

fn xor_word(w: &Word, with: &Word) -> Word {
    let mut xor = w.clone();

    xor[0] ^= with[0];
    xor[1] ^= with[1];
    xor[2] ^= with[2];
    xor[3] ^= with[3];

    xor
}

pub(crate) fn rcon(index: usize) -> Word {
    [RCON[index - 1], 0x00, 0x00, 0x00]
}

pub(crate) fn sub_word(w: &Word) -> Word {
    let mut s = *w;

    for i in 0..4 {
        sub_byte(&mut s[i]);
    }

    s
}

// Subkeys

pub(crate) fn rounds_for_mode(mode: Mode) -> usize {
    match mode {
        Mode::ECB_128 | Mode::CBC_128 | Mode::CTR_128 => 11,
        Mode::ECB_192 | Mode::CBC_192 | Mode::CTR_192 => 13,
        Mode::ECB_256 | Mode::CBC_256 | Mode::CTR_256 => 15,
    }
}

pub(crate) fn generate_subkeys(ctx: &mut Context, key: &[u8]) {
    for i in 0..(rounds_for_mode(ctx.mode) * 4) {
        let words = words_for_mode(ctx.mode);
        if i < words {
            let w = get_word(&key, i);
            set_word(&mut ctx.subkeys, i, &w);
        } else {
            let w1 = get_word(&ctx.subkeys, i - 1);
            let w2 = get_word(&ctx.subkeys, i - words);

            if (i % words) == 0 {
                let mut w = w1.clone();
                w.rotate_left(1);
                set_word(
                    &mut ctx.subkeys,
                    i,
                    &xor_word(&xor_word(&w2, &sub_word(&w)), &rcon(i / words)),
                );
            } else if words > 6 && (i % words) == 4 {
                set_word(&mut ctx.subkeys, i, &xor_word(&w2, &sub_word(&w1)));
            } else {
                set_word(&mut ctx.subkeys, i, &xor_word(&w1, &w2));
            }
        }
    }
}
