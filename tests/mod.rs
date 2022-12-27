use rcsv::*;
use rcsv::readers::*;

#[test]
fn test_string_reader() {
    let mut reader = BufferReader::from_str("aa,bb,cc\r\n");

    reader.mark_start();

    assert_eq!(97, reader.pop().unwrap());
    assert_eq!(97, reader.pop().unwrap());
    assert_eq!(44, reader.pop().unwrap());

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

    let mut reader = BufferReader::new(mapper.get_bytes());

    reader.mark_start();

    assert_eq!(97, reader.pop().unwrap());
    assert_eq!(97, reader.pop().unwrap());
    assert_eq!(44, reader.pop().unwrap());

    reader.mark_stop();

    assert_eq!(reader.segment().start, 0);
    assert_eq!(reader.segment().stop, 2);
}

#[test]
fn test_basic_parse() {
    let mut reader = BufferReader::from_str("aa,bb,cc\r\ndd,ee,ff,gg\r\n");

    parse::<3>(&mut reader, |index, fields| {
        println!("Line {}", index);

        for field in fields {
            println!("\t{} {}", field.start, field.stop);
        }
    });
}