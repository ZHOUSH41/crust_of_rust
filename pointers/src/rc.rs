use crate::cell::Cell;
use std::{ptr::NonNull, marker::PhantomData};
pub struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}
pub struct Rc<T> {
    ptr: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        let c = inner.ref_count.get();
        inner.ref_count.set(c + 1);
        Rc {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        let inner = Box::new(RcInner {
            value,
            ref_count: Cell::new(1),
        });
        Rc {
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.ptr.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let c = unsafe { self.ptr.as_ref() }.ref_count.get();
        if c == 1 {
            drop(c);
            let _ = unsafe {
                Box::from_raw(self.ptr.as_ptr());
            };
        } else {
            unsafe { self.ptr.as_ref() }.ref_count.set(c - 1);
        }
    }
}
