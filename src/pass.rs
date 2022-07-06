use crate::def::*;
use crate::support::*;

// Globals

const SBOX: [u8; 0x100] = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tables/sbox.tbl"));
const INV_SBOX: [u8; 0x100] = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tables/inv_sbox.tbl"));

#[rustfmt::skip]
const MC: [u8; 16] = [
    0x02, 0x03, 0x01, 0x01,
    0x01, 0x02, 0x03, 0x01,
    0x01, 0x01, 0x02, 0x03,
    0x03, 0x01, 0x01, 0x02,
];

#[rustfmt::skip]
const INV_MC: [u8; 16] = [
    0x0E, 0x0B, 0x0D, 0x09,
    0x09, 0x0E, 0x0B, 0x0D,
    0x0D, 0x09, 0x0E, 0x0B,
    0x0B, 0x0D, 0x09, 0x0E,
];

// Helpers

fn mix_columns_impl(chunk: &mut Chunk, matrix: &[u8; 16]) {
    let mut buffer = [0u8; 4];

    for i in 0..4 {
        let column = chunk.as_column(i).unwrap();

        for j in 0..4 {
            buffer[j] = gfmul(column[0], matrix[j * 4])
                ^ gfmul(column[1], matrix[1 + (j * 4)])
                ^ gfmul(column[2], matrix[2 + (j * 4)])
                ^ gfmul(column[3], matrix[3 + (j * 4)]);
        }

        chunk.set_column(i, buffer);
    }
}

// Pass

pub(crate) fn sub_byte(b: &mut u8) {
    *b = SBOX[*b as usize];
}

pub(crate) fn sub_bytes(chunk: &mut Chunk) {
    for i in 0..BLOCK_BYTES {
        sub_byte(&mut chunk[i]);
    }
}

pub(crate) fn shift_rows(chunk: &mut Chunk) {
    for i in 1..4 {
        let mut row = chunk.as_row(i).unwrap();
        row.rotate_left(i);
        chunk.set_row(i, row);
    }
}

pub(crate) fn mix_columns(chunk: &mut Chunk) {
    mix_columns_impl(chunk, &MC)
}

pub(crate) fn add_round_key(chunk: &mut Chunk, subkeys: &[u8], round: usize) {
    let subkey = &subkeys[round * BLOCK_BYTES..(round * BLOCK_BYTES) + BLOCK_BYTES];
    for i in 0..BLOCK_BYTES {
        chunk.as_mut_block()[i] ^= subkey[i];
    }
}

pub(crate) fn inv_sub_bytes(chunk: &mut Chunk) {
    for i in 0..BLOCK_BYTES {
        chunk[i] = INV_SBOX[chunk[i] as usize]
    }
}

pub(crate) fn inv_shift_rows(chunk: &mut Chunk) {
    for i in 1..4 {
        let mut row = chunk.as_row(i).unwrap();
        row.rotate_right(i);
        chunk.set_row(i, row);
    }
}

pub(crate) fn inv_mix_columns(chunk: &mut Chunk) {
    mix_columns_impl(chunk, &INV_MC)
}
