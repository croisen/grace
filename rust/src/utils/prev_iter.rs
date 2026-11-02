use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::{NonNull, slice_from_raw_parts_mut};

pub(crate) struct PrevIter<'a, T>
where
    T: 'a + Debug,
{
    ptr: NonNull<T>,
    index: usize,
    len: usize,
    owned: bool,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> PrevIter<'a, T>
where
    T: 'a + Debug,
{
    #[inline]
    pub const fn new(s: &'a [T]) -> Self {
        let ptr = NonNull::from_ref(s).cast();
        let index = 0usize.wrapping_sub(1);
        let len = s.len();
        Self { ptr, index, len, owned: false, _phantom: PhantomData }
    }

    #[inline]
    pub fn new_from_box_slice(s: Box<[T]>) -> Self {
        let len = s.len();
        let ptr = NonNull::new(Box::into_raw(s) as *mut T).unwrap();
        let index = 0usize.wrapping_sub(1);
        Self { ptr, index, len, owned: true, _phantom: PhantomData }
    }

    #[inline]
    pub fn next(&mut self) -> Option<&T> {
        self.index = self.index.wrapping_add(1);
        self.at(self.index)
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.at(self.index + 1)
    }

    #[inline]
    pub fn prev(&mut self) -> Option<&T> {
        self.index = self.index.wrapping_sub(1);
        self.at(self.index)
    }

    #[inline]
    pub fn at(&self, idx: usize) -> Option<&T> {
        let c = if !(self.index >= self.len) {
            unsafe { Some(self.ptr.add(idx).as_ref()) }
        } else {
            None
        };

        // println!("{c:?}");
        c
    }

    #[inline]
    pub fn seek_start(&mut self) {
        self.seek(0);
    }

    #[inline]
    pub fn seek_end(&mut self) {
        self.seek(self.len);
    }

    #[inline]
    pub fn seek(&mut self, pos: usize) {
        self.index = pos;
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn skip_while(&mut self, mut predicate: impl FnMut(&T) -> bool) {
        self.index += 1;
        while !(self.index >= self.len) {
            if !predicate(unsafe { &*self.ptr.add(self.index).as_ptr() }) {
                self.ptr = unsafe { self.ptr.sub(1) };
                break;
            }

            self.index += 1;
        }
    }
}

impl<'a, T: Debug> Drop for PrevIter<'a, T> {
    fn drop(&mut self) {
        if self.owned {
            let _ = unsafe {
                Box::from_raw(slice_from_raw_parts_mut(
                    self.ptr.as_ptr(),
                    self.len,
                ))
            };
        }
    }
}

impl<'a, T: Debug> From<&'a [T]> for PrevIter<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self::new(value)
    }
}

impl<'a, T: Debug> From<Vec<T>> for PrevIter<'a, T> {
    fn from(value: Vec<T>) -> Self {
        Self::new_from_box_slice(value.into_boxed_slice())
    }
}
