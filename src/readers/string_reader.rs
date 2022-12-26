use crate::Reader;

pub struct StringReader<'a> {
    start: usize,
    stop: usize,
    position: usize,
    data: &'a [u8],
}

impl <'a> StringReader<'a> {
    pub fn new(data: &'a str) -> StringReader<'a> {
        StringReader {
            start: 0,
            stop: 0,
            position: 0,
            data: data.as_bytes(),
        }
    }
}

impl <'a> Reader<'a> for StringReader<'a> {
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