use std::env;

fn main() {
    let file_name = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Usage: sum csv_file_name");

            return;
        }
    };

    println!("File: {}", file_name);


    let mapper = match rcsv::mmap::FileMapper::new(&file_name) {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to open file {}. {}.", file_name, e);
        }
    };

    let mut parser = rcsv::Parser::new();
    let mut max_index: usize = 0;

    parser.parse::<256>(mapper.get_bytes(), |index, _fields| {
            max_index = index;
    });

    println!("Parsed {} bytes and {} lines.", mapper.get_bytes().len(), max_index + 1);
}