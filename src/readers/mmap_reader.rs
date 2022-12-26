#[cfg(unix)]
mod unix_map {
    use std::{fs::File, os::fd::AsRawFd};

    pub struct FileMapper {
        file_size: libc::size_t,
        ptr: *mut libc::c_void,
        file: File,
    }

    impl FileMapper {
        pub fn new(file_name: &str) -> Result<FileMapper, &str> {

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
    
                Ok(
                    FileMapper {
                        file_size,
                        ptr,
                        file, 
                    }
                )
            }
        }

        pub fn get_bytes(&self) -> &[u8] {
            unsafe {std::slice::from_raw_parts(self.ptr as *const u8, self.file_size)}
        }

        pub fn size(&self) -> usize {
            self.file_size
        }
    }

    impl <'a> Drop for FileMapper {
        fn drop(&mut self) {
            unsafe {
                libc::munmap(self.ptr, self.file_size);
            }
        }
    }
}

#[cfg(unix)]
pub use unix_map::FileMapper;
