use crate::Reader;

pub struct BufferReader<'a> {
    start: usize,
    stop: usize,
    position: usize,
    data: &'a [u8],
}

impl <'a> BufferReader<'a> {
    pub fn from_str(str: &'a str) -> BufferReader<'a> {
        BufferReader::new(str.as_bytes())
    }

    pub fn new(data: &'a [u8]) -> BufferReader<'a> {
        BufferReader {
            start: 0,
            stop: 0,
            position: 0,
            data: data,
        }
    }
}

impl <'a> Reader<'a> for BufferReader<'a> {
    fn peek(&self) -> Option<u8> {
        if self.position < self.data.len() {
            Some(self.data[self.position])
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.position < self.data.len() {
            self.position += 1;

            Some(self.data[self.position - 1])
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
        self.stop = if self.position > 0  {self.position - 1} else {0};
    }

    fn segment(&'a self) -> &'a [u8] {
        &self.data[self.start..self.stop]
    }
}