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

    assert_eq!(std::str::from_utf8(reader.segment()).unwrap(), "aa");

    reader.mark_start();

    assert_eq!(98, reader.pop().unwrap());
    assert_eq!(98, reader.pop().unwrap());
    assert_eq!(44, reader.pop().unwrap());

    reader.mark_stop();

    assert_eq!(std::str::from_utf8(reader.segment()).unwrap(), "bb");
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

    assert_eq!(std::str::from_utf8(reader.segment()).unwrap(), "aa");

    reader.mark_start();

    assert_eq!(98, reader.pop().unwrap());
    assert_eq!(98, reader.pop().unwrap());
    assert_eq!(44, reader.pop().unwrap());

    reader.mark_stop();

    assert_eq!(std::str::from_utf8(reader.segment()).unwrap(), "bb");
}

#[test]
fn test_basic_parse() {
    let mut reader = BufferReader::from_str("aa,bb,cc\r\n");
    let f = |fields: &[&[u8]]| {
        println!("Segment: {}", std::str::from_utf8(fields[0]).unwrap());
    };

    rcsv::parse::<10,_>(&mut reader, |fields| {
        println!("Closure fields[0]: {}", std::str::from_utf8(fields[0]).unwrap());
    });
}