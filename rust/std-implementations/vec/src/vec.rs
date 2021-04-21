use std::{
    alloc::{Allocator, Layout},
    ops::{Deref, DerefMut},
};

use crate::unique::Unique;

/// See `https://doc.rust-lang.org/nomicon/vec-alloc.html` for details

/// RawVec<T>
/// RawVec<T> is a Heap-allocated dynamically sized array.
/// RawVec<T> if responsible for specifing buffer and freeing its memory in Vec<T> and IntoIter<T>
/// so that, wrapper of RawVec<T> doesn't need to care about these jobs.
pub struct RawVec<T> {
    ptr: Unique<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(
            std::mem::size_of::<T>() != 0,
            "We are not ready to handle ZSTs"
        );

        Self {
            ptr: Unique::empty(),
            cap: 0,
        }
    }

    /// Allocate memory for Vec<T> as it grows
    #[inline]
    fn allocate(&mut self) {
        unsafe {
            let (new_cap, result) = if self.cap == 0 {
                // Allocate memory for Vec<T> for the first time
                let new_cap = 1;
                let result =
                    std::alloc::Global.allocate(std::alloc::Layout::array::<T>(new_cap).unwrap());

                (new_cap, result)
            } else {
                // Reallocate memory for Vec<T> as it grows
                // }
                let new_cap = self.cap * 2; // capacity doubles

                // check that the new allocation doesn't exceed `usize::MAX` at all
                // regardless of the actual size of the capacity
                assert!(
                    new_cap * self.elem_size() <= usize::MAX,
                    "capacity overflow"
                );

                let result = std::alloc::Global.grow(
                    self.ptr.as_nonnull().cast(),
                    Layout::array::<T>(self.cap).unwrap(),
                    Layout::array::<T>(new_cap).unwrap(),
                );

                (new_cap, result)
            };

            // Out-of-Memory
            if result.is_err() {
                std::alloc::handle_alloc_error(Layout::from_size_align_unchecked(
                    new_cap * self.elem_size(),
                    std::mem::align_of::<T>(),
                ))
            }

            self.ptr = Unique::new_unchecked(result.unwrap().as_ptr().cast());
            self.cap = new_cap;
        }
    }

    /// Returns bytes length of Element T
    fn elem_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
}
impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            unsafe {
                let c = self.ptr.as_nonnull();
                std::alloc::Global
                    .deallocate(c.cast(), std::alloc::Layout::array::<T>(self.cap).unwrap())
            }
        }
    }
}

/// Vec<T>
/// Vec<T> is a Heap-allocated dynamically sized array.
/// This is done by dynamically growing memory allocation of Vec<T> by double
/// as the numober of elements in Vec<T> increases.
pub struct Vec<T> {
    inner: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Self {
            inner: RawVec::<T>::new(),
            len: 0,
        }
    }
    pub fn capacity(&self) -> usize {
        self.inner.cap
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Vec<T> {
    /// Returns bytes length of Element T
    fn elem_size(&self) -> usize {
        self.inner.elem_size()
    }

    /// Append element at the back of Vec<T>
    /// If the capacity lacks, Vec<T> will re-allocate more memory.
    pub fn push(&mut self, elem: T) {
        if self.len == self.inner.cap {
            self.inner.allocate()
        }

        unsafe {
            let offset = self.inner.ptr.as_ptr().offset(self.len as isize);
            std::ptr::write(offset, elem);
        }

        self.len += 1;
    }

    /// Pop element from the back of Vec<T> or return `None` if Vec<T> is empty
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe {
            Some(std::ptr::read(
                self.inner.ptr.as_ptr().offset(self.len as isize),
            ))
        }
    }

    /// Insert an element to given index by moving all the other elements to the right
    /// by one
    pub fn insert(&mut self, index: usize, elem: T) {
        // assert!(index <= self.len, "index out of bounds");
        if self.inner.cap == self.len() {
            self.inner.allocate()
        }

        unsafe {
            if index < self.len {
                std::ptr::copy(
                    self.inner.ptr.as_ptr().offset(index as isize),
                    self.inner.ptr.as_ptr().offset(index as isize + 1),
                    self.len - index,
                )
            }

            std::ptr::write(self.inner.ptr.as_ptr().offset(index as isize), elem);
            self.len += 1;
        }
    }

    /// Remove an element from Vec<T> and move other elements to the left by one
    pub fn remove(&mut self, index: usize) -> T {
        // assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = std::ptr::read(self.inner.ptr.as_ptr().offset(index as isize));
            std::ptr::copy(
                self.inner.ptr.as_ptr().offset(index as isize + 1),
                self.inner.ptr.as_ptr().offset(index as isize),
                self.len - index,
            );
            result
        }
    }
}

// IntoIter
pub struct IntoIter<T> {
    inner: RawVec<T>,
    start: *const T,
    end: *const T,
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

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.inner.cap != 0 {
            for _ in &mut *self {} // # drop any remaining element
        }
        // deallocation is handled by RawVec
    }
}
impl<T> Vec<T> {
    /// Consume Vec<T> and return IntoIter<T> which is DoubleEncodedIterator of [T]
    pub fn into_iter(self) -> IntoIter<T> {
        let inner = RawVec {
            ptr: Unique::new_unchecked(self.inner.ptr.as_ptr()),
            cap: self.inner.cap,
        };
        let len = self.len;
        let start = inner.ptr.as_ptr();
        let end = if inner.cap == 0 {
            inner.ptr.as_ptr() // no allocation yet
        } else {
            unsafe { inner.ptr.as_ptr().offset(len as isize) }
        };

        std::mem::forget(self);

        IntoIter { inner, start, end }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVec
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.inner.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.inner.ptr.as_ptr(), self.len) }
    }
}

#[cfg(test)]
mod test {
    use super::Vec;

    #[test]
    fn test_alloc() {
        let mut vec = Vec::<i32>::new();
        vec.push(1);

        assert_eq!(vec.capacity(), 1);
        assert_eq!(vec.len(), 1);

        vec.push(2);
        assert_eq!(vec.capacity(), 2);
        assert_eq!(vec.len(), 2);

        vec.push(3);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);

        vec.push(4);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_pop() {
        let mut vec = Vec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        assert_eq!(Some(4), vec.pop());
        assert_eq!(Some(3), vec.pop());
        assert_eq!(Some(2), vec.pop());
        assert_eq!(Some(1), vec.pop());
        assert_eq!(None, vec.pop());

        assert_eq!(0, vec.len());
        assert_eq!(4, vec.capacity());
    }

    #[test]
    fn test_overwrite() {
        let mut vec = Vec::<i32>::new();
        vec.push(1);
        vec.push(2);

        let p = vec.inner.ptr.as_ptr();
        assert_eq!(unsafe { std::ptr::read(p.offset(1)) }, 2);

        vec.pop();
        vec.push(3);

        assert_eq!(unsafe { std::ptr::read(p.offset(1)) }, 3);
        assert_eq!(2, vec.len());
        assert_eq!(2, vec.capacity());
    }

    #[test]
    fn test_insert() {
        let mut vec = Vec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.insert(1, 3);

        assert_eq!(3, vec.len());
        assert_eq!(Some(2), vec.pop());
        assert_eq!(Some(3), vec.pop());
        assert_eq!(Some(1), vec.pop());
        assert_eq!(None, vec.pop());
    }
    #[test]
    fn test_remove() {
        let mut vec = Vec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(3, vec.len());
        assert_eq!(Some(3), vec.pop());
        assert_eq!(Some(2), vec.pop());
        assert_eq!(Some(1), vec.pop());
        assert_eq!(None, vec.pop());
    }
    #[test]
    fn test_deref() {
        let mut vec = Vec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!([1, 2, 3], *vec);
    }

    #[test]
    fn test_into_iter() {
        let mut vec = Vec::<usize>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);

        for (i, x) in vec.into_iter().enumerate() {
            assert_eq!(i, x);
        }
    }
    #[test]
    fn test_into_iter_next() {
        let mut vec = Vec::<usize>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);

        let mut into_iter = vec.into_iter();
        assert_eq!(Some(0), into_iter.next());
        assert_eq!(Some(2), into_iter.next_back());
        assert_eq!(Some(1), into_iter.next());
        assert_eq!(None, into_iter.next_back());
    }
}
