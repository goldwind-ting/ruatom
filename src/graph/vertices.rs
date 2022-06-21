use std::fmt::Debug;

#[derive(Debug)]
pub struct VertexIter<'a, T: Iterator<Item = &'a u8> + Debug>(T);

impl<'a, T: Iterator<Item = &'a u8> + Debug> VertexIter<'a, T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}

impl<'a, T: Iterator<Item = &'a u8> + Debug> Iterator for VertexIter<'a, T> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
