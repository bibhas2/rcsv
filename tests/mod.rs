use rcsv::*;
use rcsv::readers::*;
#[test]
fn test_string_reader() {
    let data = "aa,bb,cc\r\n".as_bytes();
    let mut reader = BufferReader::new();

    reader.mark_start();

    assert_eq!(97, reader.pop(data).unwrap());
    assert_eq!(97, reader.pop(data).unwrap());
    assert_eq!(44, reader.pop(data).unwrap());

    reader.mark_stop();

    assert_eq!(reader.field(data), "aa".as_bytes());
}

#[test]
fn test_mmap() {
    let path = env!("CARGO_MANIFEST_DIR");
    let resource = format!("{path}/resources/test1.csv");

    let mapper = match FileMapper::new(&resource) {
        Ok(r) => r,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let data = mapper.get_bytes();
    let mut reader = BufferReader::new();

    reader.mark_start();

    assert_eq!(97, reader.pop(data).unwrap());
    assert_eq!(97, reader.pop(data).unwrap());
    assert_eq!(44, reader.pop(data).unwrap());

    reader.mark_stop();

    assert_eq!(reader.field(data), "aa".as_bytes());
}

#[test]
fn test_basic_parse() {
    let data = "aa,bb,cc\r\ndd,ee,ff,gg\r\n".as_bytes();
    let mut reader = BufferReader::new();

    parse::<3>(data, &mut reader, |index, fields| {
        println!("Line {}", index);

        for field in fields {
            println!("{}", std::str::from_utf8(field).unwrap());
        }
    });
}