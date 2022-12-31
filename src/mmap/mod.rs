mod file_mapper;

#[cfg(unix)]
pub use file_mapper::unix_map::FileMapper;