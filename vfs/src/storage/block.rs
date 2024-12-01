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
    pub const fn data_offset(&self, position: u64) -> u64 {
        self.offset + position + 3
    }

    /// returns the block occupied size
    pub fn get_size<S: StorageConnection>(&self, storage: &S) -> anyhow::Result<u16> {
        storage.read_u16(self.offset + 1)
    }

    pub fn read_u64<S: StorageConnection>(&self, pos: u64, storage: &S) -> anyhow::Result<u64> {
        storage.read_u64(self.data_offset(pos))
    }

    pub fn write_u64<S: StorageConnection>(
        &self,
        pos: u64,
        storage: &mut S,
        value: u64,
    ) -> anyhow::Result<()> {
        storage.write_u64(self.data_offset(pos), value)
    }

    pub fn write<S: StorageConnection>(
        &self,
        pos: u64,
        storage: &mut S,
        buf: &[u8],
    ) -> anyhow::Result<()> {
        storage.write(self.data_offset(pos), buf)
    }
}
