use std::marker::PhantomData;

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
