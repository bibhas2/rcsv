#[cfg(unix)]
pub mod unix_map {
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


#[cfg(windows)]
pub mod windows_map {
    use std::{fs::File};
    use std::os::windows::io::{AsRawHandle};
    use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
    use winapi::um::fileapi::{GetFileSize};
    use winapi::um::memoryapi::{
        CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ,
    };
    use winapi::um::winnt::PAGE_READONLY;

    pub struct FileMapper {
        file_size: usize,
        map_handle: *mut winapi::ctypes::c_void,
        ptr: *mut winapi::ctypes::c_void,
        file: File,
    }

    impl FileMapper {
        pub fn new(file_name: &str) -> Result<FileMapper, &str> {

            let file = match File::open(file_name) {
                Ok(f) => f,
                Err(_) => return Err("Failed to open file in readonly mode.")
            };
    
            unsafe {
                let map_handle = CreateFileMappingW(
                    file.as_raw_handle(),
                    std::ptr::null_mut(),
                    PAGE_READONLY,
                    0,
                    0,
                    std::ptr::null(),
                );

                if map_handle == INVALID_HANDLE_VALUE {
                    return Err("Failed to map file. CreateFileMappingW failed.");
                }

                let ptr = MapViewOfFile(
                    map_handle,
                    FILE_MAP_READ,
                    0,
                    0,
                    0
                );

                if ptr == std::ptr::null_mut() {
                    return Err("Failed to get a mapped view of file. MapViewOfFile() failed.");
                }

                let file_size = GetFileSize(file.as_raw_handle(), std::ptr::null_mut()) as usize;
    
                Ok(
                    FileMapper {
                        file_size,
                        map_handle,
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
                UnmapViewOfFile(self.ptr);
                CloseHandle(self.map_handle);
            }
        }
    }
}
