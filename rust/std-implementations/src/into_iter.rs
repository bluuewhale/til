use crate::unique::Unique;

pub struct IntoIter<T> {
    pub(crate) ptr: Unique<T>,
    pub(crate) cap: usize,
    pub(crate) start: *const T,
    pub(crate) end: *const T,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let val = std::ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(val)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / std::mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(std::ptr::read(self.end))
            }
        }
    }
}
