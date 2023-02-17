use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rust_embed::RustEmbed;
use tantivy::directory::error::{DeleteError, OpenReadError, OpenWriteError};
use tantivy::directory::{
    AntiCallToken, FileHandle, FileSlice, TerminatingWrite, WatchCallback, WatchCallbackList,
    WatchHandle, WritePtr,
};
use tantivy::Directory;

struct NullWriter {}

impl Write for NullWriter {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl TerminatingWrite for NullWriter {
    fn terminate_ref(&mut self, _: AntiCallToken) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct EmbedDirectory<T: RustEmbed + Sync> {
    embed: T,
    watch_router: Arc<WatchCallbackList>,
}

impl<T> EmbedDirectory<T>
where
    T: RustEmbed + Sync,
{
    pub fn new(embed: T) -> EmbedDirectory<T> {
        EmbedDirectory {
            embed,
            watch_router: Arc::new(WatchCallbackList::default()),
        }
    }
}

impl<T> Debug for EmbedDirectory<T>
where
    T: RustEmbed + Sync,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmbedDirectory")
    }
}

impl<T> Directory for EmbedDirectory<T>
where
    T: RustEmbed + Sync + Send + Clone + 'static,
{
    fn get_file_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>, OpenReadError> {
        todo!()
    }

    fn open_read(&self, path: &Path) -> Result<FileSlice, OpenReadError> {
        match <T as RustEmbed>::get(path.to_str().unwrap()) {
            None => Err(OpenReadError::FileDoesNotExist(PathBuf::from(path))),
            Some(d) => Ok(FileSlice::from(d.data.to_vec())),
        }
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        Ok(())
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        Ok(<T as RustEmbed>::get(path.to_str().unwrap()).is_some())
    }

    fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
        Ok(BufWriter::new(Box::new(NullWriter {})))
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        Ok(<T as RustEmbed>::get(path.to_str().unwrap())
            .unwrap()
            .data
            .to_vec())
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        Ok(())
    }

    fn sync_directory(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn watch(&self, watch_callback: WatchCallback) -> tantivy::Result<WatchHandle> {
        Ok(self.watch_router.subscribe(watch_callback))
    }
}
