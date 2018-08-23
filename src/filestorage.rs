use storage::{MemoryDescriptor, ResourceStorage, Stream};

use memmap::{Mmap, MmapMut};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::path;
use std::rc::Rc;

/// Internal storage of data as files.
#[derive(Debug, Default)]
struct MemoryMappedFileStorage {
    maps: BTreeMap<path::PathBuf, Mmap>,
}

impl MemoryMappedFileStorage {
    pub fn read(&mut self, path: path::PathBuf) -> io::Result<MemoryDescriptor> {
        if let Some(mapping) = self.maps.get(&path) {
            return Ok(MemoryDescriptor::new(mapping.as_ptr(), mapping.len()));
        }

        let file = File::open(&path)?;
        let file_mmap = unsafe { Mmap::map(&file)? };

        let mem_descr = MemoryDescriptor::new(file_mmap.as_ptr(), file_mmap.len());
        self.maps.insert(path, file_mmap);

        Ok(mem_descr)
    }

    pub fn read_mut(&self, path: &path::Path) -> io::Result<Rc<RefCell<AsMut<[u8]>>>> {
        if self.maps.contains_key(path) {
            // resource is already mmapped read-only, so we cannot mmap it as mut
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("{}", path.display()),
            ));
        }

        let file = File::open(path)?;
        let file_mmap_mut = unsafe { MmapMut::map_mut(&file)? };
        Ok(Rc::new(RefCell::new(file_mmap_mut)))
    }
}

impl Stream for File {}

/// Resource storage on disk using memory mapped files.
#[derive(Debug)]
pub struct FileResourceStorage {
    storage: MemoryMappedFileStorage,
    path: path::PathBuf,
}

impl FileResourceStorage {
    /// Create an empty memory mapped file storage.
    pub fn new(path: path::PathBuf) -> Self {
        Self {
            storage: MemoryMappedFileStorage::default(),
            path,
        }
    }
}

impl ResourceStorage for FileResourceStorage {
    fn subdir(&self, dir: &str) -> Rc<RefCell<ResourceStorage>> {
        Rc::new(RefCell::new(Self::new(self.path.join(dir))))
    }

    fn exists(&self, resource_name: &str) -> bool {
        self.path.join(resource_name).exists()
    }

    fn read_resource(&mut self, resource_name: &str) -> io::Result<MemoryDescriptor> {
        let resource_path = self.path.join(resource_name);
        if !resource_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                String::from(resource_path.to_str().unwrap_or(resource_name)),
            ));
        }

        self.storage.read(resource_path)
    }

    fn read_resource_mut(&self, resource_name: &str) -> io::Result<Rc<RefCell<AsMut<[u8]>>>> {
        let resource_path = self.path.join(resource_name);
        if !resource_path.exists() {
            // file does not exists
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{}", resource_path.display()),
            ));
        }

        self.storage.read_mut(&resource_path)
    }

    fn create_output_stream(
        &mut self,
        resource_name: &str,
    ) -> Result<Rc<RefCell<Stream>>, io::Error> {
        if !self.path.exists() {
            fs::create_dir_all(self.path.clone())?;
        }
        let resource_path = self.path.join(resource_name);
        let file = File::create(resource_path)?;
        Ok(Rc::new(RefCell::new(file)))
    }
}
