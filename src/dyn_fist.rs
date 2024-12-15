
use super::fist_impl::_Fist;
use std::marker::Unsize;
use std::ops::{Deref, DerefMut};

/// Dynamic Fist
pub struct DynFist<T: ?Sized, const SIZE: usize>(_DynFist<T, SIZE>);

enum _DynFist<T: ?Sized, const SIZE: usize> {
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
        if std::mem::size_of::<V>() <= SIZE {
            DynFist(_DynFist::Fist(_Fist::new(v)))
        } else {
            DynFist(_DynFist::Box(Box::<V>::new(v)))
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
        matches!(self.0, _DynFist::Fist(_))
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
        matches!(self.0, _DynFist::Box(_))
    }
}

impl<T: ?Sized, const SIZE: usize> Deref for DynFist<T, SIZE> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.0 {
            _DynFist::Fist(ref f) => f.deref(),
            _DynFist::Box(ref b) => b.deref(),
        }
    }
}

impl<T: ?Sized, const SIZE: usize> DerefMut for DynFist<T, SIZE> {
    fn deref_mut(&mut self) -> &mut T {
        match self.0 {
            _DynFist::Fist(ref mut f) => f.deref_mut(),
            _DynFist::Box(ref mut b) => b.deref_mut(),
        }
    }
}