use crate::storage::{block::Block, block_type::FREE_BLOCK_TYPE};

///  Abstraction that represents low level underlying file storage connection.
pub trait StorageConnection {
    /// Total size in bytes of the storage
    fn size(&self) -> u64;

    /// resizes the storage
    fn set_size(&self, new_size: u64) -> anyhow::Result<()>;

    fn read_u32(&self, pos: u64) -> anyhow::Result<u32>;
    fn read_u64(&self, pos: u64) -> anyhow::Result<u64>;
    fn read(&self, pos: u64, data: &mut [u8]) -> anyhow::Result<usize>;

    fn write_u32(&mut self, pos: u64, v: u32) -> anyhow::Result<()>;
    fn write_u64(&mut self, pos: u64, v: u64) -> anyhow::Result<()>;
    fn write(&mut self, pos: u64, buf: &[u8]) -> anyhow::Result<()>;

    /// frees the block at a given position
    fn free(&mut self, offset: u64) -> anyhow::Result<()> {
        assert_ne!(self.read_u32(offset)?, FREE_BLOCK_TYPE);
        let block_size = self.read_u32(offset + 4)?;

        // shrink the unnderlying storage of the freed block is the last
        if Block::block_end(offset, block_size.into()) == self.size() {
            return self.set_size(offset);
        }

        // set the block type back to free
        self.write_u32(offset, FREE_BLOCK_TYPE)?;

        // set next free block reference
        let first_free = self.read_u64(16)?;
        self.write_u64(offset + 8, first_free)?;

        // first free block reference
        self.write_u64(16, offset / 16)
    }
}
