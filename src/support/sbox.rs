use crate::support::math;

use std::fmt::Write;
use std::fs;

pub fn gen_sbox(inverse: bool) {
    let mut buffer = String::new();
    let mut sbox = [0u8; 0x100];

    for i in 0..sbox.len() {
        if inverse {
            sbox[i] = math::gfinv(math::inv_affine(i as _));
        } else {
            sbox[i] = math::affine(math::gfinv(i as _));
        }
    }

    write!(&mut buffer, "{:?}", sbox).unwrap();
    fs::create_dir_all("tables").unwrap();
    fs::write(
        if inverse {
            "tables/inv_sbox.tbl"
        } else {
            "tables/sbox.tbl"
        },
        buffer,
    )
    .unwrap();
}
