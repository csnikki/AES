// Perform multiplication on GF(2^8).
pub fn gfmul(x: u8, y: u8) -> u8 {
    let mut p = 0x00u8;
    let mut row = y;

    for i in 0..8 {
        if ((x >> i) & 1) == 1 {
            p ^= row;
        }

        if row & 0x80 == 0 {
            row <<= 1;
        } else {
            row = (((row as u32) << 1) ^ 0x11B) as _;
        }
    }

    p
}

// Get the inverse multiplicative on GF(2^8).
pub fn gfinv(b: u8) -> u8 {
    let mut r = b;

    for _ in 0..0xFD {
        r = gfmul(b, r);
    }

    r
}

// Apply the affine transformation.
pub fn affine(b: u8) -> u8 {
    let mut out = 0u8;

    for i in 0..8 {
        if (b >> i) & 1 == 1 {
            out ^= 0x1Fu8.rotate_left(i);
        }
    }

    out ^ 0x63
}

// Apply the inverse affine transformation.
pub fn inv_affine(b: u8) -> u8 {
    let mut out = 0u8;

    for i in 0..8 {
        if (b >> i) & 1 == 1 {
            out ^= 0x4Au8.rotate_left(i);
        }
    }

    out ^ 0x05
}
