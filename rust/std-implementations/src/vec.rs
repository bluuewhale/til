use std::{
    alloc::{AllocRef, Layout},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// See `https://doc.rust-lang.org/nomicon/vec-alloc.html` for details

/// 1. Unique<T>
/// Unique<T> is a wrapper around a raw non-null *mut T that indicates that
/// the possessor of this wrapper owns the referent. Useful for building abstractions
/// like Box<T>, Vec<T>, String and HashMap<K, V>
///
/// Unlike `*mut T`, `Unique<T>` is covariant over `T`.
/// Unline `*mut T`, the pointer must always be non-null, even if the pointer is never
/// dereferenced. This is so that enums may use this forbidden value as a discriminant

pub struct Unique<T: ?Sized> {
    ptr: *const T,
    _marker: PhantomData<T>,
}
unsafe impl<T: Send> Send for Unique<T> {}
unsafe impl<T: Sync> Sync for Unique<T> {}

impl<T: ?Sized> Unique<T> {
    /// Create a new `Unique`
    /// # Safety
    /// `ptr` must be non-null
    pub fn new_unchecked(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a new `Uniqe<T>` if `ptr` is non-null
    pub fn new(ptr: *mut T) -> Option<Self> {
        if 0 == ptr as *mut () as usize {
            None
        } else {
            Some(Self::new_unchecked(ptr))
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr as _
    }

    pub fn as_nonnull(&self) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(self.ptr as _) }
    }
}

impl<T: Sized> Unique<T> {
    /// Creates a new `Unique` that is dangling, but well-aligned
    /// This is useful for initializing types which lazily allocate, like `Vec`
    fn empty() -> Self {
        let ptr = std::mem::align_of::<T>() as *mut T;
        Self::new_unchecked(ptr)
    }
}

/// 2. Vec<T>
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
}
