use core::mem::SizedTypeProperties;
use std::alloc::{Allocator, Global};
use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::{NonNull, drop_in_place, read, without_provenance_mut};

#[flutter_rust_bridge::frb(ignore)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PrevIter<'a, T: 'a, A = Global> {
    ptr: NonNull<T>,
    start: NonNull<T>,
    end: NonNull<T>,
    alloc: Option<ManuallyDrop<A>>,
    _pdata: PhantomData<&'a T>,
}

impl<'a, T, A: Allocator> PrevIter<'a, T, A> {
    #[inline]
    pub const fn new(slice: &'a [T]) -> Self {
        let len = slice.len();
        let ptr: NonNull<T> = NonNull::from_ref(slice).cast();
        let start = NonNull::from_ref(slice).cast();
        let end = unsafe {
            if T::IS_ZST {
                without_provenance_mut(len)
            } else {
                ptr.as_ptr().add(len)
            }
        };

        Self {
            ptr,
            start,
            alloc: None,
            _pdata: PhantomData,
            end: NonNull::new(end).unwrap(),
        }
    }

    #[inline]
    pub fn new_from_vec(v: Vec<T, A>) -> Self {
        let len = v.len();
        let mut me = ManuallyDrop::new(v);
        unsafe {
            let alloc = ManuallyDrop::new(read(me.allocator()));
            let ptr: NonNull<T> = NonNull::new(me.as_mut_ptr()).unwrap();
            let start: NonNull<T> = NonNull::new(me.as_mut_ptr()).unwrap();
            let end = if T::IS_ZST {
                without_provenance_mut(len)
            } else {
                ptr.as_ptr().add(len)
            };

            Self {
                ptr,
                start,
                alloc: Some(alloc),
                _pdata: PhantomData,
                end: NonNull::new(end).unwrap(),
            }
        }
    }

    pub fn prev(&mut self) -> Option<&'a T> {
        self.ptr = unsafe { self.ptr.offset(-1) };
        if self.ptr.gt(&self.end) && self.ptr.lt(&self.start) {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr()) }
        }
    }
}

impl<'a, T> Iterator for PrevIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr = unsafe { self.ptr.offset(1) };
        if self.ptr.gt(&self.end) {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr()) }
        }
    }
}

impl<'a, T> From<&'a [T]> for PrevIter<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self::new(value)
    }
}

impl<'a, T> From<&'a Vec<T>> for PrevIter<'a, T> {
    fn from(value: &'a Vec<T>) -> Self {
        Self::new(value.as_slice())
    }
}

impl<'a, T, A: Allocator> From<Vec<T, A>> for PrevIter<'a, T, A> {
    fn from(value: Vec<T, A>) -> Self {
        Self::new_from_vec(value)
    }
}

impl<'a, T, A> Drop for PrevIter<'a, T, A> {
    // we're just copying the std IntoIter from Vec but this drop part is just
    // me guessing
    fn drop(&mut self) {
        if let Some(alloc) = self.alloc.as_mut() {
            unsafe {
                let len = self.end.offset_from_unsigned(self.start);
                drop_in_place(self.start.as_ptr().cast_slice(len));
                ManuallyDrop::drop(alloc);
                // Welp this just called drop again causing an infinite loop?
                // drop_in_place(self);
            }
        }
    }
}
