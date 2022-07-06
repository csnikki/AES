use crate::def::*;
use crate::pass::*;
use crate::subkeys::*;

// Helpers

fn encrypt_update(ctx: &Context, chunk: &mut Chunk, round: usize) {
    let rounds = rounds_for_mode(ctx.mode);

    if round > 0 {
        sub_bytes(chunk);
        shift_rows(chunk);

        if round < (rounds - 1) {
            mix_columns(chunk);
        }
    }

    add_round_key(chunk, &ctx.subkeys, round);
}

fn encrypt_chunk(ctx: &Context, chunk: &mut Chunk) {
    let rounds = rounds_for_mode(ctx.mode);

    for round in 0..rounds {
        encrypt_update(ctx, chunk, round);
    }
}

// Encrypt

impl Encrypt for Context {
    fn encrypt(&self, buffer: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut last_chunk = self.iv.clone();

        // Encryption rounds
        for i in 0..buffer.len() / BLOCK_BYTES {
            let mut chunk = Chunk::from_buffer(buffer, i).unwrap();

            // CBC: xor with last chunk.
            if self.is_cbc() {
                for i in 0..BLOCK_BYTES {
                    chunk.as_mut_block()[i] ^= last_chunk[i];
                }
            }

            // Run encryption steps.
            encrypt_chunk(&self, &mut chunk);

            // Flush output.
            out.append(&mut chunk.as_mut_block().to_vec());

            // CBC: save encrypted chunk.
            if self.is_cbc() {
                last_chunk = chunk.as_mut_block().clone();
            }
        }

        out
    }
}

#[test]
fn test() {
    let c = Context::new(Mode::CBC_256, &[
        0x46,0x45,0x44,0x43,0x42,0x41,0x39,0x38,0x37,0x36,0x35,0x34,0x33,0x32,0x31,0x30,
        0x46,0x45,0x44,0x43,0x42,0x41,0x39,0x38,0x37,0x36,0x35,0x34,0x33,0x32,0x31,0x30],
        &[
            0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35,
            0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35,
        ]);

    let encrypted = c.encrypt(&[
        0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x61,0x62,0x63,0x64,0x65,0x66,
        0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x61,0x62,0x63,0x64,0x65,0x66,
        0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x61,0x62,0x63,0x64,0x65,0x66,
        0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x61,0x62,0x63,0x64,0x65,0x66,
    ]);

    println!("Encrypted: {:x?}", encrypted);
}