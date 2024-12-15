
use super::fist_impl::_Fist;
use std::marker::Unsize;
use std::ops::{Deref, DerefMut};

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
        const { assert!(std::mem::size_of::<V>() <= SIZE) }
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
