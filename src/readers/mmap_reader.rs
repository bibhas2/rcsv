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
    }

    impl <'a> Drop for FileMapper<'a> {
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
    let bytes;

    {
        let mapper = match FileMapper::map(file_name) {
            Ok(m) => m,
            Err(msg) => {
                println!("{}", msg);
    
                return;
            }
        };
    
        bytes = mapper.get_bytes();
    }

    let str = std::str::from_utf8(bytes).unwrap();

    println!("{}", str);
}