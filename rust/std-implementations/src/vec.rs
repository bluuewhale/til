use std::{
    alloc::{AllocRef, Layout},
    marker::PhantomData,
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

    pub fn as_ptr(self) -> *mut T {
        self.ptr as _
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
                let new_cap = self.cap * 2; // capacity doubles

                // check that the new allocation doesn't exceed `usize::MAX` at all
                // regardless of the actual size of the capacity
                assert!(
                    new_cap * self.elem_size() <= usize::MAX,
                    "capacity overflow"
                );

                let result = std::alloc::Global.grow(
                    NonNull::new(self.ptr.ptr as *mut T).unwrap().cast(),
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
}
