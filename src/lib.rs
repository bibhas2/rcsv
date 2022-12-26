pub mod readers;

pub trait Reader {
    fn peek(&self) -> Option<u8>;
    fn pop(&mut self) -> Option<u8>;
    fn putback(&mut self);
    fn mark_start(&mut self);
    fn mark_stop(&mut self);
    fn segment(&self) -> &[u8];
}

pub fn parse<const N: usize, F>(reader: &mut impl Reader, consumer: F) 
    where F: Fn(&[&[u8]])
{
    //Statically allocate memory for the fields of a record (line in CSV).
    let mut storage: [&[u8]; N] = [&[]; N];
    
    reader.mark_start();
    reader.pop();
    reader.pop();
    reader.pop();
    reader.mark_stop();

    storage[0] = reader.segment();

    println!("Segment: {}", std::str::from_utf8(reader.segment()).unwrap());

    consumer(&storage[0..1]);
}