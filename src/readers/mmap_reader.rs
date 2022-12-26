use crate::Reader;

#[cfg(unix)]
mod unix_map {
    use std::{fs::File, os::fd::AsRawFd};

    pub struct FileMapper<'a> {
        file_size: libc::size_t,
        ptr: *mut libc::c_void,
        file: File,
        bytes: &'a [u8],
    }

    impl <'a> FileMapper<'a> {
        pub fn map(file_name: &str) -> Result<FileMapper<'a>, &str> {

            let file = match File::open(file_name) {
                Ok(f) => f,
                Err(_) => return Err("Failed to open file in readonly mode.")
            };
    
            unsafe {
                let mut sbuf : libc::stat = std::mem::zeroed();
    
                if libc::fstat(file.as_raw_fd(), &mut sbuf) < 0 {
                    return Err("Failed to get file size. fstat() failed.");
                }
    
                let file_size = sbuf.st_size as libc::size_t;
    
                let ptr = libc::mmap(
                    std::ptr::null_mut(),
                    file_size,
                    libc::PROT_READ,
                    libc::MAP_FILE | libc::MAP_SHARED,
                    file.as_raw_fd(),
                    0,
                );    
        
                if ptr == libc::MAP_FAILED {
                    return Err("Failed to map file. mmap() failed.");
                }
    
                let bytes = std::slice::from_raw_parts(ptr as *const u8, file_size);

                Ok(
                    FileMapper {
                        file_size,
                        ptr,
                        file, 
                        bytes,
                    }
                )
            }
        }

        pub fn get_bytes(&'a self) -> &'a [u8] {
            self.bytes
        }
        pub fn size(&self) -> usize {
            self.file_size
        }
    }

    impl <'a> Drop for FileMapper<'a> {
        fn drop(&mut self) {
            unsafe {
                libc::munmap(self.ptr, self.file_size);
            }
        }
    }
}

#[cfg(unix)]
use unix_map::FileMapper;

pub struct MemoryMappedReader<'a> {
    start: usize,
    stop: usize,
    position: usize,
    mapper: FileMapper<'a>,
}

impl <'a> MemoryMappedReader<'a> {
    pub fn new(file_name: &str) -> Result<MemoryMappedReader<'a>, &str> {
        FileMapper::map(file_name).map(|mapper| MemoryMappedReader {
            start: 0,
            stop: 0,
            position: 0,
            mapper: mapper})
    }
}

impl <'a> Reader<'a> for MemoryMappedReader<'a> {
    fn peek(&self) -> Option<u8> {
        if self.position < self.mapper.size() {
            Some(self.mapper.get_bytes()[self.position])
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.position < self.mapper.size() {
            self.position += 1;

            Some(self.mapper.get_bytes()[self.position - 1])
        } else {
            None
        }
    }

    fn putback(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    fn mark_start(&mut self) {
        self.start = self.position;
    }

    fn mark_stop(&mut self) {
        self.stop = if self.position > 0  {self.position - 1} else {0};
    }

    fn segment(&'a self) -> &'a [u8] {
        &self.mapper.get_bytes()[self.start..self.stop]
    }
}