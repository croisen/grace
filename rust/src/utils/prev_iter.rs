use std::marker::PhantomData;
use std::ptr::NonNull;

pub(crate) struct PrevIter<'a, T>
where
    T: 'a,
{
    ptr: NonNull<T>,
    index: usize,
    len: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> PrevIter<'a, T>
where
    T: 'a,
{
    #[inline]
    pub const fn new(s: &'a [T]) -> Self {
        let ptr = NonNull::from_ref(s).cast::<T>();
        let index = 0usize.wrapping_sub(1);
        let len = s.len();
        Self { ptr, index, len, _phantom: PhantomData }
    }

    #[inline]
    pub fn new_from_box_slice(s: Box<[T]>) -> Self {
        let len = s.len();
        let ptr = NonNull::from_ref(&*s).cast::<T>();
        let index = 0usize.wrapping_sub(1);
        Self { ptr, index, len, _phantom: PhantomData }
    }

    #[inline]
    pub fn next(&mut self) -> Option<&T> {
        self.index += 1;
        self.current()
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        if !(self.index >= self.len) {
            unsafe { Some(self.ptr.add(self.index).as_ref()) }
        } else {
            None
        }
    }

    #[inline]
    // now that I've added peek, prev kinda loses it's purpose
    pub fn prev(&mut self) -> Option<&T> {
        self.index -= 1;
        self.current()
    }

    #[inline]
    pub fn current(&self) -> Option<&T> {
        if !(self.index >= self.len) {
            unsafe { Some(self.ptr.add(self.index).as_ref()) }
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
        self.seek(self.len);
    }

    #[inline]
    pub fn seek(&mut self, pos: usize) {
        self.index = pos;
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn skip_while(&mut self, mut predicate: impl FnMut(&T) -> bool) {
        if size_of::<T>() == 0 {
            return;
        }

        self.index += 1;
        while !(self.index >= self.len) {
            if !predicate(unsafe { &*self.ptr.as_ptr() }) {
                self.ptr = unsafe { self.ptr.sub(1) };
                break;
            }

            self.index += 1;
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
        Self::new_from_box_slice(value.into_boxed_slice())
    }
}
