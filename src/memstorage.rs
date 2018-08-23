use storage::{MemoryDescriptor, ResourceStorage, Stream};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Cursor};
use std::mem;
use std::path;
use std::rc::Rc;

struct BytesCursor {
    inner: Cursor<Vec<u8>>,
}

impl BytesCursor {
    pub fn new() -> Self {
        Self {
            inner: Cursor::new(Vec::new()),
        }
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        self.inner.get_mut()
    }
}

impl io::Seek for BytesCursor {
    fn seek(&mut self, from: io::SeekFrom) -> io::Result<u64> {
        self.inner.seek(from)
    }
}

impl io::Write for BytesCursor {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.inner.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl AsMut<[u8]> for BytesCursor {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.get_mut()
    }
}

/// Internal storage of data in memory.
#[derive(Default)]
struct MemoryStorage {
    // Streams of resources that were written.
    streams: BTreeMap<path::PathBuf, Rc<RefCell<BytesCursor>>>,
    // Data of resources that were opened for reading.
    resources: BTreeMap<path::PathBuf, Rc<Vec<u8>>>,
}

impl fmt::Debug for MemoryStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MemoryStorage {{ num_streams: {}, num_resources: {} }}",
            self.streams.len(),
            self.resources.len(),
        )
    }
}

/// Resource storage in memory.
#[derive(Debug)]
pub struct MemoryResourceStorage {
    storage: MemoryStorage,
    path: path::PathBuf,
}

impl MemoryResourceStorage {
    /// Create an empty memory resource storage.
    pub fn new(path: path::PathBuf) -> Self {
        Self {
            storage: MemoryStorage::default(),
            path,
        }
    }
}

impl Stream for BytesCursor {}

impl ResourceStorage for MemoryResourceStorage {
    fn subdir(&self, dir: &str) -> Rc<RefCell<ResourceStorage>> {
        Rc::new(RefCell::new(Self::new(self.path.join(dir))))
    }

    fn exists(&self, resource_name: &str) -> bool {
        let resource_path = self.path.join(resource_name);
        self.storage.resources.contains_key(&resource_path)
            || self.storage.streams.contains_key(&resource_path)
    }

    fn read_resource(&mut self, resource_name: &str) -> Result<MemoryDescriptor, io::Error> {
        let resource_path = self.path.join(resource_name);
        if !self.storage.resources.contains_key(&resource_path) {
            let stream = self.storage.streams.get(&resource_path);
            match stream {
                Some(stream) => {
                    // Resource is not yet opened, but there is a stream it was written to
                    // => move out the data from the stream as resource data.
                    let mut data = Vec::new();
                    mem::swap(stream.borrow_mut().get_mut(), &mut data);
                    let data = Rc::new(data);
                    self.storage.resources.insert(resource_path.clone(), data);
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        String::from(resource_path.to_str().unwrap_or(resource_name)),
                    ));
                }
            }
        }
        let data: &[u8] = &self.storage.resources[&resource_path];
        Ok(MemoryDescriptor::new(&data[0], data.len()))
    }

    fn read_resource_mut(&self, resource_name: &str) -> io::Result<Rc<RefCell<AsMut<[u8]>>>> {
        let resource_path = self.path.join(resource_name);
        let stream = self.storage.streams.get(&resource_path);
        match stream {
            Some(stream) => {
                if self.storage.resources.contains_key(&resource_path) {
                    // resource is already opened as read-only, so we cannot modify it
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        format!("{}", resource_path.display()),
                    ));
                }

                Ok(stream.clone())
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{}", resource_path.display()),
                ));
            }
        }
    }

    fn create_output_stream(&mut self, resource_name: &str) -> io::Result<Rc<RefCell<Stream>>> {
        let resource_path = self.path.join(resource_name);
        let stream = self
            .storage
            .streams
            .entry(resource_path)
            .or_insert_with(|| Rc::new(RefCell::new(BytesCursor::new())));
        Ok(stream.clone())
    }
}
