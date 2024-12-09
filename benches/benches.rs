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

enum TestEnum<T: TestTrait> {
    Variant1(T),
    Variant2(i32)
}

impl<T: TestTrait> TestTrait for TestEnum<T> {
    fn test(&self) -> u8 {
        match self {
            TestEnum::Variant1(t) => black_box(t.test()),
            TestEnum::Variant2(i) => black_box(*i as u8),
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut init_small = c.benchmark_group("init_small");
    init_small.bench_function("box_init_small", |b| {
        b.iter(|| { let b: Box<dyn TestTrait> = Box::new(black_box(0_u8)); b } )
    });
    init_small.bench_function("enum_init_small", |b| {
        b.iter(|| { let e = TestEnum::Variant1(black_box(0_u8)); e } )
    });
    init_small.bench_function("fist_init_small", |b| {
        b.iter(|| { let f = Fist::<dyn TestTrait, 4>::new(black_box(0_u8)); f })
    });
    init_small.bench_function("dynfist_init_small", |b| {
        b.iter(|| { let d = DynFist::<dyn TestTrait, 4>::new(black_box(0_u8)); d })
    });
    init_small.finish();

    let mut init_big = c.benchmark_group("init_big");
    init_big.bench_function("box_init_big", |b| {
        b.iter_with_large_drop(|| { let b: Box<dyn TestTrait> = Box::new(black_box([0; 128])); b } )
    });
    init_big.bench_function("enum_init_big", |b| {
        b.iter_with_large_drop(|| { let e = TestEnum::Variant1(black_box([0; 128])); e } )
    });
    init_big.bench_function("fist_init_big", |b| {
        b.iter_with_large_drop(|| { let f = Fist::<dyn TestTrait, 128>::new(black_box([0; 128])); f })
    });
    init_big.bench_function("dynfist_stack_init_big", |b| {
        b.iter_with_large_drop(|| { let d = DynFist::<dyn TestTrait, 128>::new(black_box([0; 128])); d })
    });
    init_big.bench_function("dynfist_heap_init_big", |b| {
        b.iter_with_large_drop(|| { let d = DynFist::<dyn TestTrait, 4>::new(black_box([0; 128])); d })
    });
    init_big.finish();

    let mut call = c.benchmark_group("call");
    call.bench_function("box_call", |b| {
        let bx: Box<dyn TestTrait> = Box::new(black_box(250));
        b.iter(|| {
            bx.test()
        })
    });
    call.bench_function("enum_call", |b| {
        let e = TestEnum::<i32>::Variant2(black_box(250));
        b.iter(|| {
            e.test()
        })
    });
    call.bench_function("fist_call", |b| {
        let f = Fist::<dyn TestTrait, 4>::new(black_box(250));
        b.iter(|| {
            f.test()
        })
    });
    call.bench_function("dynfist_stack_call", |b| {
        let d = DynFist::<dyn TestTrait, 4>::new(black_box(250));
        b.iter(|| {
            d.test()
        })
    });
    call.bench_function("dynfist_heap_call", |b| {
        let d = DynFist::<dyn TestTrait, 3>::new(black_box(250));
        b.iter(|| {
            d.test()
        })
    });
    call.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
