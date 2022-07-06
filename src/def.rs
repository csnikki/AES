use std::ops::{Index, IndexMut};

// Constants

pub(crate) const MAX_ROUNDS: usize = 14;
pub(crate) const WORD_BYTES: usize = 4;
pub(crate) const BLOCK_BYTES: usize = 16;
pub(crate) const SUBKEY_BYTES: usize = (MAX_ROUNDS + 1) * WORD_BYTES * 4;

// Types

pub(crate) type Word = [u8; 4];
pub(crate) type Block = [u8; BLOCK_BYTES];

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Mode {
    ECB_128,
    ECB_192,
    ECB_256,
    CBC_128,
    CBC_192,
    CBC_256,
    CTR_128,
    CTR_192,
    CTR_256,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Columns {
    column0: Word,
    column1: Word,
    column2: Word,
    column3: Word,
}

#[repr(C)]
pub(crate) union Chunk {
    columns: Columns,
    block: Block,
}

#[repr(C)]
pub(crate) struct Context {
    pub(crate) mode: Mode,
    pub(crate) iv: [u8; BLOCK_BYTES],
    pub(crate) subkeys: [u8; SUBKEY_BYTES],
}

pub trait Encrypt {
    fn encrypt(&self, buffer: &[u8]) -> Vec<u8>;
}

pub trait Decrypt {
    fn decrypt(&self, buffer: &[u8]) -> Vec<u8>;
}

// Chunk

impl Chunk {
    pub(crate) fn from_buffer(buffer: &[u8], index: usize) -> Option<Chunk> {
        let mut block = [0u8; 16];

        if buffer.len() < BLOCK_BYTES {
            return None;
        }

        for i in 0..BLOCK_BYTES {
            block[i] = buffer[i];
        }

        Some(Chunk {
            block,
        })
    }

    pub(crate) fn as_row(&mut self, index: usize) -> Option<Word> {
        unsafe {
            if index < 4 {
                return Some([
                    self.columns.column0[index],
                    self.columns.column1[index],
                    self.columns.column2[index],
                    self.columns.column3[index],
                ]);
            }

            None
        }
    }

    pub(crate) fn set_row(&mut self, index: usize, row: Word) {
        unsafe {
            if index < 4 {
                self.columns.column0[index] = row[0];
                self.columns.column1[index] = row[1];
                self.columns.column2[index] = row[2];
                self.columns.column3[index] = row[3];
            }
        }
    }

    pub(crate) fn as_column(&self, index: usize) -> Option<Word> {
        unsafe {
            match index {
                0 => Some(self.columns.column0),
                1 => Some(self.columns.column1),
                2 => Some(self.columns.column2),
                3 => Some(self.columns.column3),
                _ => None,
            }
        }
    }

    pub(crate) fn set_column(&mut self, index: usize, column: Word) {
        unsafe {
            match index {
                0 => self.columns.column0.copy_from_slice(&column),
                1 => self.columns.column1.copy_from_slice(&column),
                2 => self.columns.column2.copy_from_slice(&column),
                3 => self.columns.column3.copy_from_slice(&column),
                _ => (),
            }
        }
    }


    pub(crate) fn as_mut_block(&mut self) -> &mut Block {
        unsafe { &mut self.block }
    }
}

impl Index<usize> for Chunk {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.block[index] }
    }
}

impl IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.block[index] }
    }
}
