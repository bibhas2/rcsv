#[cfg(unix)]
pub mod unix_map {
    use std::{fs::File, os::fd::AsRawFd};

    ///Provides a cross platform way to get the bytes in a file using file mapping. Currently Linux, macOS and Windows are supported.
    pub struct FileMapper {
        file_size: libc::size_t,
        ptr: *mut libc::c_void,
        file: File,
    }

    impl FileMapper {
        ///Creates a new ``FileMapper`` that maps the file pointed to by ``file_name``. The file is mapped in read only mode.
        /// 
        /// # Example
        /// ```
        ///fn test_memory_map_reader() {
        ///    let mapper = match rcsv::mmap::FileMapper::new("test.csv") {
        ///        Ok(r) => r,
        ///        Err(e) => {
        ///            panic!("Failed to map file. Error: {}.", e);
        ///        }
        ///    };
        ///
        ///    let data = mapper.get_bytes();
        ///    let mut parser = rcsv::Parser::new();
        ///
        ///    parser.parse::<3>(data, |index, fields| {
        ///
        ///    });
        ///}
        ///```
        ///
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

        ///Returns all the data in the file as byte array slice ``&[u8]``.
        pub fn get_bytes(&self) -> &[u8] {
            unsafe {std::slice::from_raw_parts(self.ptr as *const u8, self.file_size)}
        }

        ///Returns the size of the file. This is same as the length of the array slice returned by ``get_bytes()``.
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
    use winapi::um::winnt::HANDLE;
    use winapi::um::fileapi::{GetFileSize};
    use winapi::um::memoryapi::{
        CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ,
    };
    use winapi::um::winnt::PAGE_READONLY;

    pub struct FileMapper {
        file_size: usize,
        map_handle: HANDLE,
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
