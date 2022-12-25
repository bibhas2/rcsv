use crate::Reader;

#[cfg(unix)]
mod unix_map {
    use std::{fs::File, os::fd::AsRawFd};

    pub struct FileMapper {
        file_size: libc::size_t,
        ptr: *mut libc::c_void,
        file: File,
    }

    impl FileMapper {
        pub fn map(file_name: &str) -> Result<FileMapper, &str> {
            println!("Mapping file {}.", file_name);

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
            unsafe {
                std::slice::from_raw_parts(self.ptr as *const u8, self.file_size)
            }
        }
    }

    impl Drop for FileMapper {
        fn drop(&mut self) {
            println!("Unmapping file {}.", self.file.as_raw_fd());

            unsafe {
                libc::munmap(self.ptr, self.file_size);
            }
        }
    }
}

#[cfg(unix)]
use unix_map::FileMapper;

pub fn test_map(file_name : &str) {
    let mapper = match FileMapper::map(file_name) {
        Ok(m) => m,
        Err(msg) => {
            println!("{}", msg);

            return;
        }
    };

    let bytes: &[u8] = mapper.get_bytes();
    let str = std::str::from_utf8(bytes).unwrap();

    println!("{}", str);
}