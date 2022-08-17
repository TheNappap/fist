use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fist::{DynFist, Fist};

trait TestTrait {
    fn test(&self) -> u8;
}

impl TestTrait for u8 {
    fn test(&self) -> u8 {
        black_box(*self)
    }
}

impl TestTrait for i32 {
    fn test(&self) -> u8 {
        black_box(*self as u8)
    }
}

impl TestTrait for [u8; 128] {
    fn test(&self) -> u8 {
        black_box(self[0])
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("box_init_small", |b| {
        b.iter(|| {
            let b: Box<dyn TestTrait> = Box::new(0_u8);
            b
        })
    });
    c.bench_function("fist_init_small", |b| {
        b.iter(|| Fist::<dyn TestTrait, 4>::new(0_u8))
    });
    c.bench_function("dynfist_init_small", |b| {
        b.iter(|| DynFist::<dyn TestTrait, 4>::new(0_u8))
    });
    c.bench_function("box_init_big", |b| {
        b.iter(|| {
            let b: Box<dyn TestTrait> = Box::new([0; 128]);
            b
        })
    });
    c.bench_function("fist_init_big", |b| {
        b.iter(|| Fist::<dyn TestTrait, 128>::new([0; 128]))
    });
    c.bench_function("dynfist_stack_init_big", |b| {
        b.iter(|| DynFist::<dyn TestTrait, 128>::new([0; 128]))
    });
    c.bench_function("dynfist_heap_init_big", |b| {
        b.iter(|| DynFist::<dyn TestTrait, 4>::new([0; 128]))
    });
    c.bench_function("box_call", |b| {
        b.iter(|| {
            let b: Box<dyn TestTrait> = Box::new(250);
            assert_eq!(b.test(), 250);
            b
        })
    });
    c.bench_function("fist_call", |b| {
        b.iter(|| {
            let f = Fist::<dyn TestTrait, 4>::new(250);
            assert_eq!(f.test(), 250);
            f
        })
    });
    c.bench_function("dynfist_stack_call", |b| {
        b.iter(|| {
            let d = DynFist::<dyn TestTrait, 4>::new(250);
            assert_eq!(d.test(), 250);
            d
        })
    });
    c.bench_function("dynfist_heap_call", |b| {
        b.iter(|| {
            let d = DynFist::<dyn TestTrait, 3>::new(250);
            assert_eq!(d.test(), 250);
            d
        })
    });
    c.bench_function("box_10_calls", |b| {
        b.iter(|| {
            let b: Box<dyn TestTrait> = Box::new(250);
            for _ in 0..10 {
                assert_eq!(b.test(), 250);
            }
            b
        })
    });
    c.bench_function("fist_10_calls", |b| {
        b.iter(|| {
            let f = Fist::<dyn TestTrait, 4>::new(250);
            for _ in 0..10 {
                assert_eq!(f.test(), 250);
            }
            f
        })
    });
    c.bench_function("dynfist_stack_10_calls", |b| {
        b.iter(|| {
            let d = DynFist::<dyn TestTrait, 4>::new(250);
            for _ in 0..10 {
                assert_eq!(d.test(), 250);
            }
            d
        })
    });
    c.bench_function("dynfist_heap_10_calls", |b| {
        b.iter(|| {
            let d = DynFist::<dyn TestTrait, 3>::new(250);
            for _ in 0..10 {
                assert_eq!(d.test(), 250);
            }
            d
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
