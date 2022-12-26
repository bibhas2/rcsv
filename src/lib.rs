pub mod readers;

pub trait Reader<'a> {
    fn peek(&self) -> Option<u8>;
    fn pop(&mut self) -> Option<u8>;
    fn putback(&mut self);
    fn mark_start(&mut self);
    fn mark_stop(&mut self);
    fn segment(&'a self) -> &'a [u8];
}
