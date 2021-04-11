use std::ptr::NonNull;

pub struct Vec<T> {
    ptr: Option<NonNull<T>>,
    cap: usize,
    len: usize,
}
