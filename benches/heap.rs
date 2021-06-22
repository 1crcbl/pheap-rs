use criterion::{criterion_group, criterion_main, Criterion};
use keyed_priority_queue::KeyedPriorityQueue;
use pheap::PairingHeap;
use priority_queue::PriorityQueue;

fn ph_no_change_prio(op: i32) {
    let mut insert_op = op + 1;
    let mut delete_op = 0;
    let mut count = 0;

    let mut ph = PairingHeap::<i32, i32>::new();

    while insert_op > -1 {
        for _ in 0..insert_op {
            ph.insert(count, count);
            count += 1;
        }

        for _ in 0..delete_op {
            ph.delete_min();
        }

        insert_op -= 1;
        delete_op += 1;
    }
}

fn aph_no_change_prio(op: i32) {
    let mut insert_op = op + 1;
    let mut delete_op = 0;
    let mut count = 0;

    let mut aph = addressable_pairing_heap::PairingHeap::<i32, i32>::new();

    while insert_op > -1 {
        for _ in 0..insert_op {
            aph.push(count, count);
            count += 1;
        }

        for _ in 0..delete_op {
            aph.pop();
        }

        insert_op -= 1;
        delete_op += 1;
    }
}

fn a422ph_no_change_prio(op: i32) {
    let mut insert_op = op + 1;
    let mut delete_op = 0;
    let mut count = 0;

    let mut aph = pairing_heap::PairingHeap::<i32>::new();

    while insert_op > -1 {
        for _ in 0..insert_op {
            aph.push(count);
            count += 1;
        }

        for _ in 0..delete_op {
            aph.pop();
        }

        insert_op -= 1;
        delete_op += 1;
    }
}

fn pq_no_change_prio(op: i32) {
    let mut insert_op = op + 1;
    let mut delete_op = 0;
    let mut count = 0;

    let mut pq = PriorityQueue::<i32, i32>::new();

    while insert_op > -1 {
        for _ in 0..insert_op {
            pq.push(count, count);
            count += 1;
        }

        for _ in 0..delete_op {
            pq.pop();
        }

        insert_op -= 1;
        delete_op += 1;
    }
}

fn kpq_no_change_prio(op: i32) {
    let mut insert_op = op + 1;
    let mut delete_op = 0;
    let mut count = 0;

    let mut kpq = KeyedPriorityQueue::<i32, i32>::new();

    while insert_op > -1 {
        for _ in 0..insert_op {
            kpq.push(count, count);
            count += 1;
        }

        for _ in 0..delete_op {
            kpq.pop();
        }

        insert_op -= 1;
        delete_op += 1;
    }
}

fn no_change_prio(c: &mut Criterion) {
    c.bench_function("Pairing Heap (1000)", |b| {
        b.iter(|| ph_no_change_prio(1_000))
    });
    c.bench_function("Addressable Pairing Heap (1000)", |b| {
        b.iter(|| aph_no_change_prio(1_000))
    });
    c.bench_function("APasel422's Pairing Heap (1000)", |b| {
        b.iter(|| a422ph_no_change_prio(1_000))
    });
    c.bench_function("Priority Queue (1000)", |b| {
        b.iter(|| pq_no_change_prio(1_000))
    });
    c.bench_function("Keyed Priority Queue (1000)", |b| {
        b.iter(|| kpq_no_change_prio(1_000))
    });
}

criterion_group!(benches, no_change_prio);
criterion_main!(benches);
