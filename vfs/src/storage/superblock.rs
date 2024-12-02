use crate::storage::block_type::BlockType;

use super::{block::Block, storage_connection::StorageConnection};

/// Block 0 is the superblock, which contains basic information about the filesystem.
/// It has the following on-disk structure:
/// ~~~text
/// offset   type          content
///      0   u8           magic BlockType::Super
///      1   u64          root directory
///      9   u64          first free block
/// ~~~
/// Since no other block needs to refer to the superblock,
/// the block number 0 means an absent block, for example in lists of directory entries.
pub struct SuperBlock(Block);

impl SuperBlock {
    pub fn new<S: StorageConnection>(storage: &S) -> anyhow::Result<Self> {
        let inner = Block::new(storage, 0)?;
        assert_eq!(inner.block_type, BlockType::Super);
        Ok(Self(inner))
    }

    pub fn get_rootdir_ref<S: StorageConnection>(&self, storage: &S) -> anyhow::Result<u64> {
        self.0.read_u64(0, storage)
    }

    pub fn set_root_dir<S: StorageConnection>(
        &self,
        storage: &S,
        obj: &Block,
    ) -> anyhow::Result<()> {
        self.0.write_u64(0, storage, obj.offset / 16)
    }
}
