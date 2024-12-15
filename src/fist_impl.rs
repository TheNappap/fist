use std::marker::{PhantomData, Unsize};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice::from_raw_parts;

pub struct _Fist<T: ?Sized, const SIZE: usize> {
    data: [u8; SIZE],
    vtable: *mut (),
    _p: PhantomData<T>,
}

impl<T: ?Sized, const SIZE: usize> _Fist<T, SIZE> {
    pub fn new<V: Unsize<T>>(v: V) -> _Fist<T, SIZE> {
        let value_size = mem::size_of::<V>();
        assert!(value_size <= SIZE);
        let r: &T = &v;
        unsafe {
            let r: (*mut u8, *mut ()) = mem::transmute_copy(&r);
            let value_data: &[u8] = from_raw_parts( r.0, value_size);
            let mut data = [0_u8; SIZE];
            data[..value_size].copy_from_slice(value_data);
            mem::forget(v);
            _Fist {
                data,
                vtable: r.1,
                _p: PhantomData,
            }
        }
    }

    unsafe fn ptr(&self) -> *mut T {
        mem::transmute_copy::<(*mut (), *mut ()), *mut T>(&(
            &self.data as *const _ as *mut (),
            self.vtable,
        ))
    }
}

impl<T: ?Sized, const SIZE: usize> Drop for _Fist<T, SIZE> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place::<T>(self.ptr());
        }
    }
}

impl<T: ?Sized, const SIZE: usize> Deref for _Fist<T, SIZE> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr() }
    }
}

impl<T: ?Sized, const SIZE: usize> DerefMut for _Fist<T, SIZE> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr() }
    }
}

