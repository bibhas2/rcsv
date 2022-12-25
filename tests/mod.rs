use rcsv::*;
use rcsv::readers::StringReader;

#[test]
fn test_string_reader() {
    let mut reader = StringReader::new("aa,bb,cc\r\n");

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