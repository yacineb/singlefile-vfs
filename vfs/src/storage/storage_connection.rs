use alloc::vec;

type Result<T> = anyhow::Result<T>;

///  Abstraction that represents low level underlying file storage connection.
pub trait StorageConnection {
    fn write_i32(&self, pos: u64, v: i32) -> Result<()>;
    fn write_u64(&self, pos: u64, v: u64) -> Result<()>;
    fn write(&self, pos: u64, buf: &[u8]) -> Result<()>;
}

/// Represents a writer for storage, maintaining a position.
pub struct StorageWriter<'a, S> {
    storage: &'a S,
    pos: u64,
}

impl<'a, S> StorageWriter<'a, S>
where
    S: StorageConnection,
{
    /// Creates a new `StorageWriter` with the given storage and position.
    pub fn new(storage: &'a S, pos: u64) -> Self {
        Self { storage, pos }
    }

    /// Writes a 4-byte integer to storage at the current position and advances the position.
    pub fn write_i32(&mut self, v: i32) -> Result<()> {
        self.storage.write_i32(self.pos, v)?;
        self.pos += 4;
        Ok(())
    }

    /// Writes an 8-byte reference to storage at the current position and advances the position.
    pub fn write_u64(&mut self, v: u64) -> Result<()> {
        self.storage.write_u64(self.pos, v)?;
        self.pos += 8;
        Ok(())
    }

    /// Writes a byte slice to storage at the current position and advances the position.
    pub fn write(&mut self, buf: &[u8], off: usize, len: usize) -> Result<()> {
        assert!(
            off + len <= buf.len(),
            "Offset and length exceed buffer size"
        );
        self.storage.write(self.pos, &buf[off..off + len])?;
        self.pos += len as u64;
        Ok(())
    }

    /// Writes padding to align the position to a 16-byte boundary.
    pub fn write_padding(&mut self) -> Result<()> {
        let col = (self.pos & 0x0F) as usize;
        if col != 0 {
            let padding = vec![0; 16 - col];
            self.write(&padding, 0, 16 - col)?;
        }
        Ok(())
    }
}
