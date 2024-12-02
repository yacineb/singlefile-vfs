use anyhow::Ok;

use crate::storage::block::Block;

use super::{block_type::BlockType, storage_writer::StorageWriter};

///  Abstraction that represents low level underlying file storage connection.
pub trait StorageConnection {
    /// Total size in bytes of the storage
    fn size(&self) -> u64;

    /// resizes the storage
    fn set_size(&self, new_size: u64) -> anyhow::Result<()>;

    fn read_u32(&self, pos: u64) -> anyhow::Result<u32>;
    fn read_u64(&self, pos: u64) -> anyhow::Result<u64>;
    fn read(&self, pos: u64, data: &mut [u8]) -> anyhow::Result<usize>;

    fn write_u32(&self, pos: u64, v: u32) -> anyhow::Result<()>;
    fn write_u64(&self, pos: u64, v: u64) -> anyhow::Result<()>;
    fn write(&self, pos: u64, buf: &[u8]) -> anyhow::Result<()>;

    /// frees the block at a given position
    fn free(&self, block: &Block) -> anyhow::Result<()> {
        assert_ne!(block.block_type, BlockType::Free);

        let offset = block.offset;

        // shrink the unnderlying storage of the freed block is the last
        if Block::block_end(offset, block.size.into()) == self.size() {
            return self.set_size(offset);
        }

        // set the block type back to free
        self.write_u32(offset, BlockType::Free.into())?;

        // set next free block reference
        let first_free = self.read_u64(16)?;
        self.write_u64(offset + 8, first_free)?;

        // first free block reference
        self.write_u64(16, offset / 16)
    }

    fn init(&self) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        let mut writer = StorageWriter::new(self, 0);

        const ROOT_DIR: u64 = 2;
        writer.write_u32(BlockType::Super.into())?;
        writer.write_u32(16)?;
        writer.write_u64(ROOT_DIR)?;
        writer.write_u64(0)?; //first free
        writer.write_padding()?;

        const ENTRIES: u32 = 4;
        writer.write_u32(BlockType::Directory.into())?;
        writer.write_u32(8 + 16 * ENTRIES)?;
        writer.write_u64(ROOT_DIR)?; // parent node

        for _ in 0..ENTRIES {
            writer.write_u64(0)?; // name
            writer.write_u64(0)?; // object
        }

        writer.write_padding()?;
        Ok(())
    }
}
