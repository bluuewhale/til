use std::{
    alloc::{AllocRef, Layout},
    ops::{Deref, DerefMut},
};

use crate::{into_iter::IntoIter, unique::Unique};

/// See `https://doc.rust-lang.org/nomicon/vec-alloc.html` for details

/// Vec<T>
/// Vec<T> is a Heap-allocated dynamically sized array.
/// This is done by dynamically growing memory allocation of Vec<T> by double
/// as the numober of elements in Vec<T> increases.
pub struct Vec<T> {
    ptr: Unique<T>,
    cap: usize,
    len: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        assert!(
            std::mem::size_of::<T>() != 0,
            "We are not ready to handle ZSTs"
        );
        Self {
            ptr: Unique::empty(),
            len: 0,
            cap: 0,
        }
    }
    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Vec<T> {
    /// Returns bytes length of Element T
    fn elem_size(&self) -> usize {
        std::mem::size_of::<T>()
    }

    /// Allocate memory for Vec<T> as it grows
    #[inline]
    fn allocate(&mut self) {
        unsafe {
            let (new_cap, result) = if self.cap == 0 {
                // Allocate memory for Vec<T> for the first time
                let new_cap = 1;
                let result =
                    std::alloc::Global.alloc(std::alloc::Layout::array::<T>(new_cap).unwrap());

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

    /// Append element at the back of Vec<T>
    /// If the capacity lacks, Vec<T> will re-allocate more memory.
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.allocate()
        }

        unsafe {
            let offset = self.ptr.as_ptr().offset(self.len as isize);
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
        unsafe { Some(std::ptr::read(self.ptr.as_ptr().offset(self.len as isize))) }
    }

    /// Insert an element to given index by moving all the other elements to the right
    /// by one
    pub fn insert(&mut self, index: usize, elem: T) {
        // assert!(index <= self.len, "index out of bounds");
        if self.cap == self.len() {
            self.allocate()
        }

        unsafe {
            if index < self.len {
                std::ptr::copy(
                    self.ptr.as_ptr().offset(index as isize),
                    self.ptr.as_ptr().offset(index as isize + 1),
                    self.len - index,
                )
            }

            std::ptr::write(self.ptr.as_ptr().offset(index as isize), elem);
            self.len += 1;
        }
    }

    /// Remove an element from Vec<T> and move other elements to the left by one
    pub fn remove(&mut self, index: usize) -> T {
        // assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = std::ptr::read(self.ptr.as_ptr().offset(index as isize));
            std::ptr::copy(
                self.ptr.as_ptr().offset(index as isize + 1),
                self.ptr.as_ptr().offset(index as isize),
                self.len - index,
            );
            result
        }
    }
}

// IntoIter
impl<T> Vec<T> {
    /// Consume Vec<T> and return IntoIter<T> which is DoubleEncodedIterator of [T]
    pub fn into_iter(self) -> IntoIter<T> {
        let ptr = Unique::new_unchecked(self.ptr.as_ptr());
        let cap = self.cap;
        let len = self.len;

        std::mem::forget(self);

        unsafe {
            IntoIter {
                start: ptr.as_ptr(),
                end: if cap == 0 {
                    ptr.as_ptr()
                } else {
                    ptr.as_ptr().offset(len as isize)
                },
                ptr,
                cap,
            }
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap == 0 {
            return;
        }

        while let Some(_) = self.pop() {}
        unsafe {
            std::alloc::Global.dealloc(
                self.ptr.as_nonnull().cast(),
                Layout::array::<T>(self.cap).unwrap(),
            )
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
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

        let p = vec.ptr.as_ptr();
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
