use num_enum::{FromPrimitive, IntoPrimitive};

pub const FREE_BLOCK_TYPE: u32 = 0;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum BlockType {
    #[default]
    Free = FREE_BLOCK_TYPE,
    Super = 1,
    Directory = 2,
    Regular = 3,
    Chunk = 4,
    Name = 5,
}
