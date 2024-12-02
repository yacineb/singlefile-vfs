use anyhow::Ok;

use super::{
    block_type::BlockType, storage_connection::StorageConnection, storage_writer::StorageWriter,
};

/// The block is represented as follows:
/// .          | offset | type
/// block_type | 0      | u8
/// size       | 1      | u16
/// data       | 3      | [u8]
pub struct Block {
    pub offset: u64,
    pub block_type: BlockType,
    pub size: u32,
}

const BLOCK_SIZE: u32 = 4096;

impl Block {
    pub fn new<S: StorageConnection>(storage: &S, offset: u64) -> anyhow::Result<Self> {
        // read the "magic" byte, the very first one
        let block_type: BlockType = storage.read_u32(offset)?.try_into()?;
        let size = storage.read_u32(offset + 4)?;
        Ok(Self {
            offset,
            block_type,
            size,
        })
    }

    pub fn alloc_name<S: StorageConnection + Sized>(
        storage: &S,
        name: &str,
    ) -> anyhow::Result<Self> {
        let bytes = name.as_bytes();
        let block = Self::allocate(storage, BlockType::Name, bytes.len() as _)?;
        block.write(0, storage, bytes)?;
        Ok(block)
    }

    pub fn alloc_dir<S: StorageConnection + Sized>(
        storage: &S,
        entries: u32,
        parent: u64,
    ) -> anyhow::Result<Self> {
        let block = Self::allocate(storage, BlockType::Directory, 8 + entries * 16)?;
        block.write_u64(0, storage, parent)?;
        Ok(block)
    }

    pub fn alloc_file<S: StorageConnection + Sized>(storage: &S) -> anyhow::Result<Self> {
        Self::allocate(storage, BlockType::Regular, BLOCK_SIZE)
    }

    pub fn alloc_chunk<S: StorageConnection + Sized>(
        storage: &S,
        size: u32,
    ) -> anyhow::Result<Self> {
        Self::allocate(storage, BlockType::Chunk, size + 8)
    }

    fn allocate<S: StorageConnection + Sized>(
        storage: &S,
        block_type: BlockType,
        size: u32,
    ) -> anyhow::Result<Self> {
        // append the block to the end of the storage
        let offset = storage.size();
        let mut writer = StorageWriter::new(storage, offset);

        writer.write_u32(block_type.into())?;
        writer.write_u32(size)?;

        // extend the underlying storage
        storage.set_size(Self::block_end(offset, size.into()))?;

        Ok(Self {
            offset,
            block_type,
            size,
        })
    }

    /// returns the data section offset for the given block
    pub const fn data_offset(&self, position: u64) -> u64 {
        self.offset + 8 + position
    }

    /// returns the block occupied size
    pub fn get_size<S: StorageConnection>(&self, storage: &S) -> anyhow::Result<u32> {
        storage.read_u32(self.offset + 4)
    }

    pub fn read_u64<S: StorageConnection>(&self, pos: u64, storage: &S) -> anyhow::Result<u64> {
        storage.read_u64(self.data_offset(pos))
    }

    pub fn write_u64<S: StorageConnection>(
        &self,
        pos: u64,
        storage: &S,
        value: u64,
    ) -> anyhow::Result<()> {
        storage.write_u64(self.data_offset(pos), value)
    }

    pub fn write<S: StorageConnection>(
        &self,
        pos: u64,
        storage: &S,
        buf: &[u8],
    ) -> anyhow::Result<()> {
        storage.write(self.data_offset(pos), buf)
    }

    /// gets the padding needed for a correct alignment
    pub fn padding(pos: u64) -> u8 {
        if pos % 16 == 0 {
            0
        } else {
            16 - ((pos & 0b1111) as u8)
        }
    }

    pub fn block_end(offset: u64, data_size: u64) -> u64 {
        let block_size = 8 + data_size;
        offset + block_size + Self::padding(block_size) as u64
    }
}
