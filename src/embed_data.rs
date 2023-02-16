use data::Data;
use rust_embed::{EmbeddedFile, RustEmbed};
use std::fmt::{Debug, Formatter};
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tantivy::directory::error::{DeleteError, OpenReadError, OpenWriteError};
use tantivy::directory::{
    AntiCallToken, FileHandle, FileSlice, TerminatingWrite, WatchCallback, WatchCallbackList,
    WatchHandle, WritePtr,
};
use tantivy::{Directory, TantivyError};

#[derive(RustEmbed)]
#[folder = "data/"]
#[prefix = "data/"]
pub struct EmbedData;

#[derive(RustEmbed)]
#[folder = "embed/traits/"]
pub struct EmbedTraits;

impl EmbedData {
    pub fn load() -> Data {
        let file = EmbedData::get("data/creatures.json").unwrap();
        Data {
            creatures: serde_json::from_slice(file.data.as_ref()).unwrap(),
        }
    }
}

#[derive(Clone, Default)]
pub struct EmbedDirectory {
    watch_router: Arc<WatchCallbackList>,
}

struct NullWriter {}
impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl TerminatingWrite for NullWriter {
    fn terminate_ref(&mut self, _: AntiCallToken) -> std::io::Result<()> {
        todo!()
    }
}

impl Debug for EmbedDirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmbedDirectory")
    }
}

impl Directory for EmbedDirectory {
    fn get_file_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>, OpenReadError> {
        todo!()
    }

    fn open_read(&self, path: &Path) -> Result<FileSlice, OpenReadError> {
        match EmbedTraits::get(path.to_string_lossy().deref()) {
            None => Err(OpenReadError::FileDoesNotExist(PathBuf::from(path))),
            Some(d) => Ok(FileSlice::from(d.data.to_vec())),
        }
        //.ok_or_else(|| )
        //.map(Clone::clone)
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        Ok(())
        // Err(DeleteError::IoError {
        //     filepath: path.to_path_buf(),
        //     io_error: Arc::new(std::io::Error::from(std::io::ErrorKind::Other)),
        // })
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        Ok(EmbedTraits::get(path.to_string_lossy().deref()).is_some())
    }

    fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
        Ok(BufWriter::new(Box::new(NullWriter {})))
        // Err(OpenWriteError::IoError {
        //     filepath: path.to_path_buf(),
        //     io_error: Arc::new(std::io::Error::from(std::io::ErrorKind::Other)),
        // })
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        Ok(EmbedTraits::get(path.to_string_lossy().deref())
            .unwrap()
            .data
            .to_vec())
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        Ok(())
        //Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists))
    }

    fn sync_directory(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn watch(&self, watch_callback: WatchCallback) -> tantivy::Result<WatchHandle> {
        Ok(self.watch_router.subscribe(watch_callback))
        // Err(TantivyError::IoError(Arc::new(std::io::Error::from(
        //     std::io::ErrorKind::AlreadyExists,
        // ))))
    }
}
