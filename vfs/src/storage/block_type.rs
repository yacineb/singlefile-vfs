use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum BlockType {
    #[default]
    Free = 0,
    Super = 1,
    Directory = 2,
    Regular = 3,
    Chunk = 4,
    Name = 5,
}
