pub mod readers;

#[derive(Copy, Clone)]
pub struct FieldSegment {
    pub start: usize,
    pub stop: usize,
}

pub trait Reader {
    fn peek(&self, data: &[u8]) -> Option<u8>;
    fn pop(&mut self, data: &[u8]) -> Option<u8>;
    fn putback(&mut self);
    fn mark_start(&mut self);
    fn mark_stop(&mut self);
    fn segment(&self) -> FieldSegment;
}

enum ParseStatus {
    HasMoreFields,
    EndRecord,
    EndDocument
}

fn next_field(data: &[u8], reader: &mut impl Reader) -> ParseStatus {
    let mut inside_dquote = false;
    let mut escaped_field = false;
    let dquote:u8 = 34;
    let comma:u8 = 44;
    let cr:u8 = 13;
    let lf:u8 = 10;

    reader.mark_start();
    
    loop {
        if let Some(ch) = reader.pop(data) {
            if ch == dquote {
                if !inside_dquote {
                    inside_dquote = true;
                    escaped_field = true;
                    
                    reader.mark_start();
                } else {
                    match reader.peek(data) {
                        Some(ch2) => {
                            if ch2 == dquote {
                                //Still inside dquote
                                reader.pop(data);
                            } else {
                                //We are out of dquote
                                inside_dquote = false;
                                
                                reader.mark_stop();
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
                    reader.mark_stop();
                }
                
                //field = reader.segment();

                return ParseStatus::HasMoreFields;
            }
            
            if ch == cr {
                if !escaped_field {
                    reader.mark_stop();
                }
                
                // field = reader.segment();
                
                reader.pop(data); //Read the LF \n
                
                return ParseStatus::EndRecord;
            }
            
            /*
             * Non-standard end of line with just a LF \n
             */
            if ch == lf {
                if !escaped_field {
                    reader.mark_stop();
                }
    
                // field = reader.segment();
                
                return ParseStatus::EndRecord;
            }
        } else {
            return ParseStatus::EndDocument;
        }
    }
}

fn parse_record<'a>(data: &[u8], reader: &'a mut impl Reader, storage: &'a mut [FieldSegment]) -> Option<usize> {
    let mut field_index : usize  = 0;

    loop {
        let status = next_field(data, reader);

        match status {
            ParseStatus::HasMoreFields => {
                if field_index < storage.len() {
                    storage[field_index] = reader.segment();
        
                    field_index += 1;
                }
            },
            ParseStatus::EndRecord => {
                if field_index < storage.len() {
                    storage[field_index] = reader.segment();
        
                    field_index += 1;
                }

                return Some(field_index);
            },
            ParseStatus::EndDocument => {
                return None;
            }
        }
    }
}

pub fn parse<const N: usize>(data: &[u8], reader: &mut impl Reader, consumer: impl Fn(usize, &[FieldSegment])) 
{
    //Statically allocate memory for the fields of a record (line in CSV).
    let mut storage: [FieldSegment; N] = [FieldSegment {start:0, stop: 0}; N];
    let mut field_list: [&[u8]; N] = [&[]; N];
    let mut index: usize = 0;
    
    while let Some(field_count) = parse_record(data, reader, &mut storage) {
        consumer(index, &storage[0..field_count]);

        index += 1;
    }
}