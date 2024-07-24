use criterion::{criterion_group, criterion_main, Criterion};
use priority_queue::PriorityQueue;
use prioritymap::PriorityMap;
use rand::random;

pub fn insert_pop(c: &mut Criterion) {
    let num_entries = 10_000;
    let priorities: Vec<_> = (0..num_entries).map(|_| rand::random::<u64>()).collect();

    c.bench_function("insert_pop", |b| {
        b.iter(|| {
            let mut map = PriorityMap::new();
            for entry_id in 0..num_entries {
                map.insert(priorities[entry_id], entry_id as u128, [0; 512]);
            }
            for _ in 0..num_entries {
                map.pop();
            }
        })
    });
}

pub fn reprioritize(c: &mut Criterion) {
    let num_entries = 10_000;
    let priorities: Vec<_> = (0..num_entries).map(|_| rand::random::<u64>()).collect();
    let mut map = PriorityMap::new();
    for entry_id in 0..num_entries {
        map.insert(priorities[entry_id], entry_id as u128, [0; 512]);
    }

    c.bench_function("reprioritize", |b| {
        b.iter(|| {
            for entry_id in 0..num_entries {
                map.reprioritize(&(entry_id as u128), random());
            }
        })
    });
}

struct Value {
    priority: u64,
    value: [u8; 512],
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub fn insert_pop_pq(c: &mut Criterion) {
    let num_entries = 10_000;
    let priorities: Vec<_> = (0..num_entries).map(|_| rand::random::<u64>()).collect();

    c.bench_function("insert_pop_pq", |b| {
        b.iter(|| {
            let mut map = PriorityQueue::new();
            for entry_id in 0..num_entries {
                map.push(
                    entry_id as u128,
                    Value {
                        priority: priorities[entry_id],
                        value: [0; 512],
                    },
                );
            }
            for _ in 0..num_entries {
                map.pop();
            }
        })
    });
}

pub fn reprioritize_pq(c: &mut Criterion) {
    let num_entries = 10_000;
    let priorities: Vec<_> = (0..num_entries).map(|_| rand::random::<u64>()).collect();
    let mut map = PriorityQueue::new();
    for entry_id in 0..num_entries {
        map.push(
            entry_id as u128,
            Value {
                priority: priorities[entry_id],
                value: [0; 512],
            },
        );
    }

    c.bench_function("reprioritize_pq", |b| {
        b.iter(|| {
            for entry_id in 0..num_entries {
                map.change_priority_by(&(entry_id as u128), |value| {
                    value.priority = random();
                });
            }
        })
    });
}

criterion_group!(
    benches,
    insert_pop,
    insert_pop_pq,
    reprioritize,
    reprioritize_pq
);
criterion_main!(benches);
