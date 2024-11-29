/// Enumeration of possible methods to seek within an I/O object.
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),

    /// Sets the offset to the size of this object plus the specified number of bytes.
    /// It is possible to seek beyond the end of an object, but it’s an error to seek before byte 0.
    End(i64),

    /// Sets the offset to the current position plus the specified number of bytes.
    /// It is possible to seek beyond the end of an object, but it’s an error to seek before byte 0.
    Current(i64),
}

/// Seek to an offset, in bytes, in a stream.
pub trait StreamProvider {
    /// Writes the bytes and returns the total read
    fn write(&mut self, buf: &[u8]) -> anyhow::Result<u64>;

    /// Reads the bytes at the given position and returns count of total read
    fn read(&self, buf: &mut [u8], position: u64) -> anyhow::Result<usize>;

    /// Flush the write buffer it is has one
    fn flush(&mut self) -> anyhow::Result<()>;

    fn seek(&mut self, seek: SeekFrom) -> anyhow::Result<()>;
}
