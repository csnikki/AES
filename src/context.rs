use crate::def::*;
use crate::subkeys::*;

// Context

impl Context {
    pub fn new(mode: Mode, key: &[u8], iv: &[u8]) -> Context {
        let mut c = Context {
            mode,
            iv: [0u8; BLOCK_BYTES],
            subkeys: [0u8; SUBKEY_BYTES],
        };

        c.reset(mode, key, iv);
        c
    }

    pub fn reset(&mut self, mode: Mode, key: &[u8], iv: &[u8]) {
        self.mode = mode;
        self.iv.copy_from_slice(iv);
        generate_subkeys(self, &key);
    }

    pub fn is_ecb(&self) -> bool {
        match self.mode {
            Mode::ECB_128 | Mode::ECB_192 | Mode::ECB_256 => true,
            _ => false,
        }
    }

    pub fn is_cbc(&self) -> bool {
        match self.mode {
            Mode::CBC_128 | Mode::CBC_192 | Mode::CBC_256 => true,
            _ => false,
        }
    }

    pub fn is_ctr(&self) -> bool {
        match self.mode {
            Mode::CTR_128 | Mode::CTR_192 | Mode::CTR_256 => true,
            _ => false,
        }
    }
}