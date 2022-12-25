use crate::Reader;
use std::{fs::File, os::fd::AsRawFd};

pub fn test_map(file_name : &str) {
    println!("Mapping {}", file_name);

    let file = File::open(file_name).unwrap();

    unsafe {
        let mut sbuf : libc::stat = std::mem::zeroed();

        if libc::fstat(file.as_raw_fd(), &mut sbuf) < 0 {
            println!("stat failed {}", file_name);

            return;
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
            println!("Map failed {}", file_name);

            return
        }

        let bytes: &[u8] = std::slice::from_raw_parts(ptr as *const u8, file_size);
        let str = std::str::from_utf8(bytes).unwrap();

        println!("{}", str);

        libc::munmap(ptr, file_size);
    }
}