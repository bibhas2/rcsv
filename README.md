# Non-Allocating CSV Parser in Rust
Key features of this library.

- Follows [RFC 4180](https://www.rfc-editor.org/rfc/rfc4180).
- Does not allocate any memory on the heap.
- Doesn't throw.

## Quick Example
```rust
fn test_record() {
    let str =
"aa,bb,cc,dd\r\n\
ee,ff,gg,hh\r\n";
    
    let mut parser = rcsv::Parser::new();

    parser.parse::<10>(str.as_bytes(), |index, fields| {
        assert!(index < 2);
        
        if index == 0 {
            assert!(fields[0] == "aa".as_bytes());
            assert!(fields[3] == "dd".as_bytes());
        } else {
            assert!(fields[0] == "ee".as_bytes());
            assert!(fields[3] == "hh".as_bytes());
        }
    });
}
```

The ``Parser::parse::<10>()`` call statically allocates enough space for 10 fields per line. Any excess fileds are discarded and does not cause any errors.


# User Guide
## Parsing a String

The ``Parser::parse()`` method parses CSV data supplied as an array of unsigned bytes ``&[u8]``. 

You need to have an estimate for how many fields are expected per line. This is needed to statically allocate space at compile time. You can always err on the side of caution. For example, if you expect 4 fields per line, you can configure the parser with 10 fields.

```rust
fn test_record() {
    let str =
"aa,bb,cc,dd\r\n\
ee,ff,gg,hh\r\n";
    
    let mut parser = rcsv::Parser::new();

    parser.parse::<10>(str.as_bytes(), |index, fields| {
        println!("Record no: {}", index);
        println!("Field count: {}", fields.len());
    });
}
```

Should print:

```
Record no: 0
Field count: 4
Record no: 1
Field count: 4
```

The ``parse()`` method receives two parameters.

1. The ``&[u8]`` data to parse.
2. A closure that is called for each record (line in CSV).

The closure receives two parameters:

1. The index of the record. The first line has an index of 0.
2. An array of fields. Each field is an array of unsigned bytes ``&[u8]``. 

If a record has more fields than the parser was configured for then the excess fields are discarded and not reported to the lambda.

```rust
fn test_uneven() {
    let str =
"aa,bb,cc,dd\r\n\
ee,ff,gg\r\n\
hh,ii\r\n";
    
    let mut parser = rcsv::Parser::new();

    parser.parse::<3>(str.as_bytes(), |index, fields| {
            assert!(index < 3);
        
            if index == 0 {
                assert!(fields.len() == 3);
                
                assert!(fields[0] == "aa".as_bytes());
                assert!(fields[2] == "cc".as_bytes());
            } else if index == 1 {
                assert!(fields.len() == 3);
                
                assert!(fields[0] == "ee".as_bytes());
                assert!(fields[1] == "ff".as_bytes());
            } else {
                assert!(fields.len() == 2);
                
                assert!(fields[0] == "hh".as_bytes());
                assert!(fields[1] == "ii".as_bytes());
            }
        });
}

```

## Parsing a CSV File
Memory mapping is used to read from a CSV file.

Let's suppose you have a file called ``test.csv`` as follows.

```
aa,bb,cc
dd,ee,ff
gg,hh,ii
```

We can read the file like this.

```rust
fn test_memory_map_reader() {
    let mapper = match rcsv::mmap::FileMapper::new("test.csv") {
        Ok(r) => r,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let data = mapper.get_bytes();
    let mut parser = rcsv::Parser::new();

    parser.parse::<3>(data, |index, fields| {
        assert!(index < 3);
            
        if index == 0 {
            assert!(fields.len() == 3);
            
            assert!(fields[0] == "aa".as_bytes());
            assert!(fields[2] == "cc".as_bytes());
        } else if index == 1 {
            assert!(fields.len() == 3);
            
            assert!(fields[0] == "dd".as_bytes());
            assert!(fields[1] == "ee".as_bytes());
        } else {
            assert!(fields.len() == 3);
            
            assert!(fields[0] == "gg".as_bytes());
            assert!(fields[1] == "hh".as_bytes());
        }
    });
}
```

# Standard Conformance
The library conforms to RFC 4180. It relaxes the standard a bit to be more flexible. These departures are discussed below.

## UNIX Newline
RFC 4180 requires each line to be ended by CRLF (``\r\n``). It is common in Linux and macOS for files to end with just a LF. The library tolerates such files.

## Spaces Around Escaped Fields
The RFC makes it clear that spaces are a part of the fields. They should not be ignored. However, it's not clear what happens to the spaces before or after the double quotes of an escaped field. The ABNF grammer appears to indicate that there should be no spaces. The parser discards spaces before and after the double quotes around an escaped field.

In the example below the unescaped fields ``aa`` and ``cc`` have spaces around them. These spaces are preserved. However, for the escaped fields such as ``"bb"`` the spaces outside the double quotes are ignored.

```
 aa, "bb",  cc ,
   " dd "  , " ee "
```

```rust
fn test_space() {
    let str = r#" aa, "bb",  cc ,
  " dd ", " ee "
"#;
    let mut parser = rcsv::Parser::new();

    parser.parse::<10>(str.as_bytes(), |index, fields| {
        assert!(index < 2);
        
        if index == 0 {
            assert!(fields[0] == " aa".as_bytes());
            assert!(fields[1] == "bb".as_bytes());
            assert!(fields[2] == "  cc ".as_bytes());
        } else {
            assert!(fields[0] == " dd ".as_bytes());
            assert!(fields[1] == " ee ".as_bytes());
        }
    });
}
```

## Un-Escaping Double Quotes

Escaped double quotes are not unescaped by the parser. I found no simple way of doing that without allocating. In the example below the field ``"b""b"`` is reported to the lambda without unescaping the double quote.

```
aa,"b""b",cc,"d,d"
ee,ff,"g
g",hh
```

```rust
fn test_basic_escape() {
    let str = r#"aa,"b""b",cc,"d,d"
"ee",ff,"g
g",hh
"#;
    let mut parser = rcsv::Parser::new();

    parser.parse::<10>(str.as_bytes(), |index, fields| {
        assert!(index < 2);
        
        if index == 0 {
            assert!(fields[1] == "b\"\"b".as_bytes());
            assert!(fields[3] == "d,d".as_bytes());
        } else {
            assert!(fields[0] == "ee".as_bytes());
            assert!(fields[2] == "g\ng".as_bytes());
            assert!(fields[3] == "hh".as_bytes());
        }
    });
}
```

# Memory Safety
In Rust the array index operator ``[index]`` does bounds checking. The slicing operator ``[start..stop]`` does the same. The library should be memory safe in that regard.

