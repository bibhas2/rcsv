mod file_mapper;

#[cfg(unix)]
pub use file_mapper::unix_map::FileMapper;

#[cfg(windows)]
pub use file_mapper::windows_map::FileMapper;