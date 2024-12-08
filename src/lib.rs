//! fist crate

#![warn(missing_docs)]
#![allow(incomplete_features)]
#![feature(unsize)]

#[cfg(test)]
mod tests;

use std::marker::{PhantomData, Unsize};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice::from_raw_parts;

struct _Fist<T: ?Sized, const SIZE: usize> {
    data: [u8; SIZE],
    vtable: *mut (),
    _p: PhantomData<T>,
}

impl<T: ?Sized, const SIZE: usize> _Fist<T, SIZE> {
    pub fn new<V: Unsize<T>>(v: V) -> _Fist<T, SIZE> {
        let r: &T = &v;
        unsafe {
            let r: (*mut u8, *mut ()) = mem::transmute_copy(&r);
            let data: &[u8] = from_raw_parts( r.0, mem::size_of::<V>());
            mem::forget(v);
            _Fist {
                data: data.try_into().expect("Value is too big in size to fit in the Fist"),
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


/// Fixed Sized Trait (Object) (FiST)
pub struct Fist<T: ?Sized, const SIZE: usize>(_Fist<T, SIZE>);

impl<T: ?Sized, const SIZE: usize> Fist<T, SIZE> {
    /// Creates a new fist
    ///
    /// # Examples
    ///
    /// ```
    /// # use fist::Fist;
    /// # use std::fmt::Display;
    /// let fist = Fist::<dyn Display, 4>::new(0_i32);
    /// ```
    /// ```compile_fail
    /// # use fist::Fist;
    /// # use std::fmt::Display;
    /// let fist = Fist::<dyn Display, 3>::new(0_i32);
    /// ```
    pub fn new<V: Unsize<T>>(v: V) -> Fist<T, SIZE> {
        const { assert!(mem::size_of::<V>() <= SIZE) }
        Fist(_Fist::new(v))
    }
}

impl<T: ?Sized, const SIZE: usize> Deref for Fist<T, SIZE> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized, const SIZE: usize> DerefMut for Fist<T, SIZE> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Dynamic Fist
pub enum DynFist<T: ?Sized, const SIZE: usize> {
    /// Stack Type
    Fist(_Fist<T, SIZE>),
    /// Heap Type
    Box(Box<T>),
}

impl<T: ?Sized, const SIZE: usize> DynFist<T, SIZE> {
    /// Creates a new dynamic fist
    ///
    /// # Examples
    ///
    /// ```
    /// # use fist::DynFist;
    /// # use std::fmt::Display;
    /// let dynfist_stack : fist::DynFist<dyn Display, 4> = fist::DynFist::new(0_i32);
    /// let dynfist_heap : fist::DynFist<dyn Display, 3> = fist::DynFist::new(0_i32);
    /// ```
    pub fn new<V: Unsize<T>>(v: V) -> DynFist<T, SIZE> {
        if mem::size_of::<V>() <= SIZE {
            DynFist::Fist(_Fist::new(v))
        } else {
            DynFist::Box(Box::<V>::new(v))
        }
    }

    /// Returns `true` if the owned value is on stack (if self is on stack).
    ///
    /// # Examples
    ///
    /// ```
    /// use fist::DynFist;
    /// use std::fmt::Display;
    ///
    /// let dynfist_stack : fist::DynFist<dyn Display, 4> = fist::DynFist::new(0_i32);
    /// assert!(dynfist_stack.on_stack());
    /// ```
    pub fn on_stack(&self) -> bool {
        matches!(self, DynFist::Fist(_))
    }

    /// Returns `true` if the owned value is on heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use fist::DynFist;
    /// use std::fmt::Display;
    ///
    /// let dynfist_heap : fist::DynFist<dyn Display, 3> = fist::DynFist::new(0_i32);
    /// assert!(dynfist_heap.on_heap());
    /// ```
    pub fn on_heap(&self) -> bool {
        matches!(self, DynFist::Box(_))
    }
}

impl<T: ?Sized, const SIZE: usize> Deref for DynFist<T, SIZE> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            DynFist::Fist(ref f) => f.deref(),
            DynFist::Box(ref b) => b.deref(),
        }
    }
}

impl<T: ?Sized, const SIZE: usize> DerefMut for DynFist<T, SIZE> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            DynFist::Fist(ref mut f) => f.deref_mut(),
            DynFist::Box(ref mut b) => b.deref_mut(),
        }
    }
}
