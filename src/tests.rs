use super::{DynFist, Fist};
use std::mem::size_of_val;

trait TestTrait {
    fn test(&self) -> u8;
}

impl TestTrait for u8 {
    fn test(&self) -> u8 {
        *self
    }
}

impl TestTrait for i32 {
    fn test(&self) -> u8 {
        *self as u8
    }
}

impl TestTrait for [u8; 32] {
    fn test(&self) -> u8 {
        self[0]
    }
}

impl TestTrait for [u8; 128] {
    fn test(&self) -> u8 {
        self[0]
    }
}

impl TestTrait for Box<[u8; 128]> {
    fn test(&self) -> u8 {
        self[0]
    }
}

#[test]
fn fist_init() {
    let f = Fist::<dyn TestTrait, 4>::new(255);
    assert_eq!(f.test(), 255);
    let f = Fist::<dyn TestTrait, 8>::new(Box::new([255; 128]));
    assert_eq!(f.test(), 255);
    let f = Fist::<[i32], 128>::new([255; 32]);
    assert_eq!(f[0], 255);
}

#[test]
fn dynfist_init() {
    let f = DynFist::<dyn TestTrait, 4>::new(255);
    assert_eq!(f.test(), 255);
    let f = DynFist::<dyn TestTrait, 3>::new(255_i32);
    assert_eq!(f.test(), 255);
    let f = DynFist::<dyn TestTrait, 4>::new(Box::new([255; 128]));
    assert_eq!(f.test(), 255);
    let f = DynFist::<dyn TestTrait, 8>::new(Box::new([255; 128]));
    assert_eq!(f.test(), 255);
}

#[test]
fn fist_size() {
    let f = Fist::<dyn TestTrait, 1>::new(0_u8);
    assert_eq!(size_of_val(&f), 16);
    let f = Fist::<dyn TestTrait, 4>::new(0_i32);
    assert_eq!(size_of_val(&f), 16);
    let f = Fist::<dyn TestTrait, 32>::new([0; 32]);
    assert_eq!(size_of_val(&f), 40);
    let f = Fist::<dyn TestTrait, 128>::new([0; 128]);
    assert_eq!(size_of_val(&f), 136);
    let f = Fist::<dyn TestTrait, 8>::new(Box::new([255; 128]));
    assert_eq!(size_of_val(&f), 16);
}

#[test]
fn dynfist_size() {
    let f = DynFist::<dyn TestTrait, 1>::new(0_u8);
    assert_eq!(size_of_val(&f), 24);
    let f = DynFist::<dyn TestTrait, 4>::new(0_i32);
    assert_eq!(size_of_val(&f), 24);
    let f = DynFist::<dyn TestTrait, 32>::new([0; 32]);
    assert_eq!(size_of_val(&f), 48);
    let f = DynFist::<dyn TestTrait, 128>::new([0; 128]);
    assert_eq!(size_of_val(&f), 144);
    let f = DynFist::<dyn TestTrait, 8>::new([0; 128]);
    assert_eq!(size_of_val(&f), 24);
    let f = DynFist::<dyn TestTrait, 8>::new(Box::new([255; 128]));
    assert_eq!(size_of_val(&f), 24);
}

#[test]
fn dynfist_location() {
    let f = DynFist::<dyn TestTrait, 128>::new([0; 128]);
    assert!(f.on_stack());
    let f = DynFist::<dyn TestTrait, 8>::new([0; 128]);
    assert!(f.on_heap());
    let f = DynFist::<dyn TestTrait, 8>::new(Box::new([255; 128]));
    assert!(f.on_stack());
}
