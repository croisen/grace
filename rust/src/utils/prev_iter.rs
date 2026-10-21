use std::marker::PhantomData;
use std::ptr::{NonNull, without_provenance_mut};

use crate::utils::macros::any;

#[flutter_rust_bridge::frb(ignore)]
pub struct PrevIter<'a, T>
where
    T: 'a,
{
    ptr: NonNull<T>,
    start: NonNull<T>,
    end: NonNull<T>,
    capacity: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> PrevIter<'a, T>
where
    T: 'a,
{
    #[inline]
    pub const fn new(s: &'a [T]) -> Self {
        let len = s.len();

        let start = NonNull::from_ref(s).cast::<T>();
        let ptr = unsafe { start.sub(1) };
        let end = unsafe {
            if size_of::<T>() == 0 {
                NonNull::new_unchecked(without_provenance_mut(len))
            } else {
                start.add(len)
            }
        };

        Self { ptr, start, end, capacity: 0, _phantom: PhantomData }
    }

    #[inline]
    pub fn new_from_vec(s: Vec<T>) -> Self {
        let (ptr, len, capacity) = s.into_raw_parts();

        let start = unsafe { NonNull::new(ptr).unwrap_unchecked() };
        let ptr = unsafe { start.sub(1) };
        let end = unsafe {
            if size_of::<T>() == 0 {
                NonNull::new_unchecked(without_provenance_mut(len))
            } else {
                start.add(len)
            }
        };

        Self { ptr, start, end, capacity, _phantom: PhantomData }
    }

    #[inline]
    pub fn next(&mut self) -> Option<&T> {
        self.ptr = unsafe { self.ptr.add(1) };
        self.current()
    }

    #[inline]
    pub fn prev(&mut self) -> Option<&T> {
        self.ptr = unsafe { self.ptr.sub(1) };
        self.current()
    }

    #[inline]
    pub fn current(&self) -> Option<&T> {
        if !any!(self.ptr.ge(&self.end), self.ptr.lt(&self.start)) {
            unsafe { Some(&*self.ptr.as_ptr()) }
        } else {
            None
        }
    }

    #[inline]
    pub fn seek_start(&mut self) {
        self.seek(0);
    }

    #[inline]
    pub fn seek_end(&mut self) {
        self.seek(unsafe { self.end.offset_from(self.start) });
    }

    #[inline]
    pub fn seek(&mut self, pos: isize) {
        self.ptr = unsafe { self.start.offset(pos) };
    }

    #[inline]
    pub fn pos(&self) -> isize {
        unsafe { self.ptr.offset_from(self.start) }
    }

    #[inline]
    pub fn skip_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(&T) -> bool,
    {
        if size_of::<T>() == 0 {
            return;
        }

        self.ptr = unsafe { self.ptr.add(1) };
        while !any!(self.ptr.ge(&self.end), self.ptr.lt(&self.start)) {
            if !predicate(unsafe { &*self.ptr.as_ptr() }) {
                self.ptr = unsafe { self.ptr.sub(1) };
                break;
            }

            self.ptr = unsafe { self.ptr.add(1) };
        }
    }

    #[inline]
    pub fn collect_while<P>(&mut self, mut predicate: P) -> Vec<&T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut v = vec![];

        self.ptr = unsafe { self.ptr.add(1) };
        while !any!(self.ptr.ge(&self.end), self.ptr.lt(&self.start)) {
            if !predicate(unsafe { &*self.ptr.as_ptr() }) {
                self.ptr = unsafe { self.ptr.sub(1) };
                break;
            }

            v.push(unsafe { &*self.ptr.as_ptr() });
            self.ptr = unsafe { self.ptr.add(1) };
        }

        v
    }
}

impl<'a, T> Drop for PrevIter<'a, T>
where
    T: 'a,
{
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                let len = self.end.offset_from(self.start) as usize;
                let ptr = self.start.as_ptr();
                let _ = Vec::from_raw_parts(ptr, len, self.capacity);
            }
        }
    }
}

impl<'a, T> From<&'a [T]> for PrevIter<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self::new(value)
    }
}

impl<'a, T> From<Vec<T>> for PrevIter<'a, T> {
    fn from(value: Vec<T>) -> Self {
        Self::new_from_vec(value)
    }
}
