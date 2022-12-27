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

    assert_eq!(reader.segment().start, 0);
    assert_eq!(reader.segment().stop, 2);
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

    assert_eq!(reader.segment().start, 0);
    assert_eq!(reader.segment().stop, 2);
}

#[test]
fn test_basic_parse() {
    let data = "aa,bb,cc\r\ndd,ee,ff,gg\r\n".as_bytes();
    let mut reader = BufferReader::new();

    parse::<3>(data, &mut reader, |index, fields| {
        println!("Line {}", index);

        for field in fields {
            let raw_field = &data[field.start..field.stop];
            println!("\t{} {}", field.start, field.stop);
            println!("{}", std::str::from_utf8(raw_field).unwrap());
        }
    });
}