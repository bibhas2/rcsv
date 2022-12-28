use crate::{Reader};

pub struct BufferReader {
    start: usize,
    stop: usize,
    position: usize,
}

impl BufferReader {
    pub fn new() -> BufferReader {
        BufferReader {
            start: 0,
            stop: 0,
            position: 0,
        }
    }
}

impl Reader for BufferReader {
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
        self.stop = if self.position > 0  {self.position - 1} else {0};
    }

    fn field<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        &data[self.start..self.stop]
    }
}