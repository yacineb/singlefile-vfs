use super::block::Block;

///  Abstraction that represents low level underlying file storage connection.
pub trait StorageConnection {
    fn read_u16(&self, pos: u64) -> anyhow::Result<u16>;
    fn read_u32(&self, pos: u64, v: i32) -> anyhow::Result<u32>;
    fn read_u8(&self, pos: u64) -> anyhow::Result<u8>;
    fn read(&self, pos: u64, data: &mut [u8]) -> anyhow::Result<usize>;

    fn write_i32(&mut self, pos: u64, v: i32) -> anyhow::Result<()>;
    fn write_u64(&mut self, pos: u64, v: u64) -> anyhow::Result<()>;
    fn write(&mut self, pos: u64, buf: &[u8]) -> anyhow::Result<()>;

    /// frees the block at a given position
    fn free(&mut self, offset: u64);

    /// returns the block occupied size
    fn get_size(&self, block: &Block) -> anyhow::Result<u16> {
        self.read_u16(block.offset + 1)
    }

    fn write_to_block(&mut self, pos: u64, block: &Block, buf: &[u8]) -> anyhow::Result<()> {
        self.write(block.data_offset(pos), buf)
    }
}
