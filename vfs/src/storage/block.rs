use anyhow::Ok;

use super::{block_type::BlockType, storage_connection::StorageConnection};

pub struct Block {
    pub offset: u64,
    pub block_type: BlockType,
}

impl Block {
    pub fn new<S: StorageConnection>(storage: &S, offset: u64) -> anyhow::Result<Self> {
        // read the "magic" byte, the very first one
        let block_type: BlockType = storage.read_u8(offset)?.try_into()?;
        Ok(Self { offset, block_type })
    }

    /// returns the data section offset for the given block
    pub fn data_offset(&self, position: u64) -> u64 {
        self.offset + position + 3
    }
}
