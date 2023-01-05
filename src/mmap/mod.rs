//!Provides a cross platform way to get the bytes in a CSV file using memory mapping. Currently Linux, macOS and Windows are supported.

mod file_mapper;

#[cfg(unix)]
pub use file_mapper::unix_map::FileMapper;

#[cfg(windows)]
pub use file_mapper::windows_map::FileMapper;