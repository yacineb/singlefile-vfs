#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockType {
    Super,
    Directory,
    Regular,
    Chunk,
    Free,
    Name,
}

impl From<BlockType> for u8 {
    fn from(value: BlockType) -> Self {
        match value {
            BlockType::Super => 1,
            BlockType::Directory => 2,
            BlockType::Regular => 4,
            BlockType::Chunk => 8,
            BlockType::Free => 16,
            BlockType::Name => 32,
        }
    }
}
