mod buffer_reader;
mod file_mapper;
pub use buffer_reader::BufferReader;
#[cfg(unix)]
pub use file_mapper::unix_map::FileMapper;