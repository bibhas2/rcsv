pub mod mmap;
enum ParseStatus {
    HasMoreFields,
    EndRecord,
    EndDocument,
}

///A non-allocating parser of CSV data. 
///It is compliant with RFC 4180. Due to its non-allocating nature, it can
/// parse very large CSV files with a constant memory cost.
pub struct Parser {
    start: usize,
    stop: usize,
    position: usize,
}

impl Parser {
    /// Creates a new parser.
    pub fn new() -> Parser {
        Parser {
            start: 0,
            stop: 0,
            position: 0,
        }
    }

    fn peek(&self, data: &[u8]) -> Option<u8> {
        if self.position < data.len() {
            Some(data[self.position])
        } else {
            None
        }
    }

    fn pop(&mut self, data: &[u8]) -> Option<u8> {
        if self.position < data.len() {
            self.position += 1;

            Some(data[self.position - 1])
        } else {
            None
        }
    }

    fn putback(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    fn mark_start(&mut self) {
        self.start = self.position;
    }

    fn mark_stop(&mut self) {
        self.stop = if self.position > 0 {
            self.position - 1
        } else {
            0
        };
    }

    fn field<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        &data[self.start..self.stop]
    }

    fn next_field(&mut self, data: &[u8]) -> ParseStatus {
        let mut inside_dquote = false;
        let mut escaped_field = false;
        let dquote: u8 = 34;
        let comma: u8 = 44;
        let cr: u8 = 13;
        let lf: u8 = 10;

        self.mark_start();

        loop {
            if let Some(ch) = self.pop(data) {
                if ch == dquote {
                    if !inside_dquote {
                        inside_dquote = true;
                        escaped_field = true;

                        self.mark_start();
                    } else {
                        match self.peek(data) {
                            Some(ch2) => {
                                if ch2 == dquote {
                                    //Still inside dquote
                                    self.pop(data);
                                } else {
                                    //We are out of dquote
                                    inside_dquote = false;

                                    self.mark_stop();
                                }
                            }
                            None => {
                                return ParseStatus::EndDocument;
                            }
                        }
                    }

                    continue;
                }

                if inside_dquote {
                    continue;
                }

                if ch == comma {
                    if !escaped_field {
                        self.mark_stop();
                    }

                    return ParseStatus::HasMoreFields;
                }

                if ch == cr {
                    if !escaped_field {
                        self.mark_stop();
                    }

                    self.pop(data); //Read the LF \n

                    return ParseStatus::EndRecord;
                }

                /*
                 * Non-standard end of line with just a LF \n
                 */
                if ch == lf {
                    if !escaped_field {
                        self.mark_stop();
                    }

                    return ParseStatus::EndRecord;
                }
            } else {
                return ParseStatus::EndDocument;
            }
        }
    }

    fn parse_record<'a>(&mut self, data: &'a [u8], fields: &mut [&'a [u8]]) -> Option<usize> {
        let mut field_index: usize = 0;

        loop {
            let status = self.next_field(data);

            match status {
                ParseStatus::HasMoreFields => {
                    if field_index < fields.len() {
                        fields[field_index] = self.field(data);

                        field_index += 1;
                    }
                }
                ParseStatus::EndRecord => {
                    if field_index < fields.len() {
                        fields[field_index] = self.field(data);

                        field_index += 1;
                    }

                    return Some(field_index);
                }
                ParseStatus::EndDocument => {
                    return None;
                }
            }
        }
    }

    /// Begins parsing CSV ``data``. For every record (line in CSV), the ``consumer`` closure is called.
    /// The generic parameter ``N`` determines the maximum number of fields (columns) that will be passed
    /// to the closure. If the record has more fields then the excess fields are silently ignored.
    /// It is always safer to err on the side of caution and set ``N`` larger than you think you need.
    /// 
    /// The closure receives two paremeters:
    /// - The index of the record. The first line has an index of 0.
    /// - An array slice of fields ``&[ &[u8] ]``. Each field is an array of unsigned bytes ``&[u8]``.
    /// 
    /// # Example
    ///  ```
    /// fn test_uneven() {
    ///     let str =
    /// "aa,bb,cc,dd\r\n\
    /// ee,ff,gg\r\n\
    /// hh,ii\r\n";
    /// 
    ///     let mut parser = rcsv::Parser::new();
    ///
    ///     parser.parse::<3>(str.as_bytes(), |index, fields| {
    ///             assert!(index < 3);
    ///     
    ///             if index == 0 {
    ///                 //We have more fields (4) than the maximum set (3).
    ///                 assert!(fields.len() == 3);
    ///               
    ///                 assert!(fields[0] == "aa".as_bytes());
    ///                 assert!(fields[2] == "cc".as_bytes());
    ///             } else if index == 1 {
    ///                 assert!(fields.len() == 3);
    ///             
    ///                 assert!(fields[0] == "ee".as_bytes());
    ///                 assert!(fields[1] == "ff".as_bytes());
    ///             } else {
    ///                 //We have fewer fields (2) than the maximum set (3).
    ///                 assert!(fields.len() == 2);
    ///                
    ///                 assert!(fields[0] == "hh".as_bytes());
    ///                 assert!(fields[1] == "ii".as_bytes());
    ///             }
    ///         });
    /// }
    /// ```
    pub fn parse<const N: usize>(&mut self, data: &[u8], mut consumer: impl FnMut(usize, &[&[u8]])) {
        //Statically allocate memory for the fields of a record (line in CSV).
        let mut fields: [&[u8]; N] = [&[]; N];
        let mut index: usize = 0;

        while let Some(field_count) = self.parse_record(data, &mut fields) {
            consumer(index, &fields[0..field_count]);

            index += 1;
        }
    }
}

///Utility function that parses the ``bytes`` array slice to a number ``n``.
///It returns true if the conversion is successful.
/// 
/// Parsing is done by the ``std::str::parse()`` method.
/// 
/// As an optimization, the ``bytes`` array slice ``&[u8]`` is converted to ``&str`` without
/// validating for UTF-8. This is OK to do since the number parser does its own validation.
/// 
/// Spaces around the string are trimmed to avoid errors from the number parser.
///  
/// # Example
/// ```
/// fn test_parse_number() {
/// let str =
/// "0.025717778,  -2.3285230e-002  ,-1.4653762e-002\r\n\
/// 0.036717778,-3.4285230e-002,-10.4653762e-002\r\n";
///     
///     let mut parser = rcsv::Parser::new();
///     let mut total = 0.0f64;
/// 
///     parser.parse::<3>(str.as_bytes(), |index, fields| {
///             assert!(index < 2);
/// 
///             let mut n1:f64 = 0.0;
///             let mut n2:f64 = 0.0;
///             let mut n3:f64 = 0.0;
/// 
///             assert!(rcsv::parse_number(fields[0], &mut n1));
///             assert!(rcsv::parse_number(fields[1], &mut n2));
///             assert!(rcsv::parse_number(fields[2], &mut n3));
/// 
///             total += n1 + n2 + n3;
///     });
/// 
///     assert!(f64::abs(total - (-0.114442428)) < 0.0001);
/// }
/// ```
pub fn parse_number<T: std::str::FromStr>(bytes:&[u8], n: &mut T) -> bool {
    unsafe {
        match std::str::from_utf8_unchecked(bytes).trim()
            .parse::<T>() {
            Ok(v) => {
                *n = v;

                true
            },
            Err(_) => false
        }
    }
}