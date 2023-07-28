use criterion::{criterion_group, criterion_main, Criterion};
use rust_linked_list::LinkedList;

fn benchmark_push(c: &mut Criterion) {
    c.bench_function("push", |b| {
        b.iter(|| {
            let mut list: LinkedList<i32> = LinkedList::new();
            for i in 0..10000 {
                list.push(i);
            }
        })
    });
}

fn benchmark_pop(c: &mut Criterion) {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10000 {
        list.push(i);
    }

    c.bench_function("pop", |b| b.iter(|| list.pop()));
}

criterion_group!(benches, benchmark_push, benchmark_pop);
criterion_main!(benches);
