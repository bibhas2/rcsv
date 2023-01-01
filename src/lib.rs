pub mod mmap;
enum ParseStatus {
    HasMoreFields,
    EndRecord,
    EndDocument,
}

pub struct Parser {
    start: usize,
    stop: usize,
    position: usize,
}

impl Parser {
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

pub fn parse_number<T: std::str::FromStr>(bytes:&[u8], n: &mut T) -> bool {
    unsafe {
        match std::str::from_utf8_unchecked(bytes)
            .parse::<T>() {
            Ok(v) => {
                *n = v;

                true
            },
            Err(_) => false
        }
    }
}