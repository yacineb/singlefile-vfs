use alloc::{string::String, vec::Vec};

type Result<T> = anyhow::Result<T>;

pub trait Filesystem {
    /// Moves a file or directory to a new location.
    fn move_path(&self, old_path: &str, new_path: &str) -> Result<()>;

    /// Creates a new directory.
    fn mkdir(&self, dir: &str) -> Result<()>;

    /// Removes an empty directory.
    fn rmdir(&self, dir: &str) -> Result<()>;

    /// Reads the contents of a directory.
    fn readdir(&self, dir: &str) -> Result<Vec<String>>;

    /// Deletes a file.
    fn delete(&self, file: &str) -> Result<()>;
}
