#[test]
fn test_memory_map_reader() {
    let path = env!("CARGO_MANIFEST_DIR");
    let resource = format!("{path}/resources/test1.csv");

    let mapper = match rcsv::mmap::FileMapper::new(&resource) {
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

#[test]
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


#[test]
fn test_empty_line() {
    let str =
"aa,bb,cc,dd\r\n\
\r\n\
ee,ff,gg,hh\r\n";
    
    let mut parser = rcsv::Parser::new();

    parser.parse::<10>(str.as_bytes(), |index, fields| {
        assert!(index < 3);
        
        if index == 0 {
            assert!(fields[0] == "aa".as_bytes());
            assert!(fields[3] == "dd".as_bytes());
        } else if index == 1 {
            //Empty line
            assert!(fields.len() == 1);
            assert!(fields[0] == "".as_bytes());
        } else {
            assert!(fields[0] == "ee".as_bytes());
            assert!(fields[3] == "hh".as_bytes());
        }
    });
}

#[test]
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

#[test]
/*
 * For unescaped fields, spaces are part of the field and can not be discarded.
 * For escaped fields, spaces before or after "" not allowed in the RFC.
 * But many CSV files may have it. The parser discards these spaces.
 */
fn test_space() {
    let str = r#" aa, "bb",  cc ,
  " cc ", " dd "
"#;
    let mut parser = rcsv::Parser::new();

    parser.parse::<10>(str.as_bytes(), |index, fields| {
        assert!(index < 2);
        
        if index == 0 {
            assert!(fields[0] == " aa".as_bytes());
            assert!(fields[1] == "bb".as_bytes());
            assert!(fields[2] == "  cc ".as_bytes());
        } else {
            assert!(fields[0] == " cc ".as_bytes());
            assert!(fields[1] == " dd ".as_bytes());
        }
    });
}

#[test]
fn test_line_feed() {
    //Non-standard CSV line endings but common in Linux/macOS
    let str =
"aa,bb,cc,dd
ee,ff,gg,hh
";
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

#[test]
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

#[test]
fn test_parse_number() {
    let str =
"0.025717778,-2.3285230e-002,-1.4653762e-002\r\n\
0.036717778,-3.4285230e-002,-10.4653762e-002\r\n";
    
    let mut parser = rcsv::Parser::new();
    let mut total = 0.0f64;

    parser.parse::<3>(str.as_bytes(), |index, fields| {
            assert!(index < 3);

            let mut n1:f64 = 0.0;
            let mut n2:f64 = 0.0;
            let mut n3:f64 = 0.0;

            assert!(rcsv::parse_number(fields[0], &mut n1));
            assert!(rcsv::parse_number(fields[1], &mut n2));
            assert!(rcsv::parse_number(fields[2], &mut n3));

            total += n1 + n2 + n3;
    });

    assert!(f64::abs(total - (-0.114442428)) < 0.0001);
}