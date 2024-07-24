use std::collections::{hash_map, HashMap};

pub struct PriorityMap<P, K, V>
where
    K: std::hash::Hash,
{
    heap: Vec<Entry<P, K, V>>,
    map: HashMap<K, usize>,
}

impl<P, K, V> PriorityMap<P, K, V>
where
    P: PartialOrd + Clone,
    K: Eq + std::hash::Hash + Clone,
    V: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: vec![],
            map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.map.len(), self.heap.len());
        self.map.len()
    }

    pub fn insert(&mut self, priority: P, key: K, value: V) {
        match self.map.entry(key.clone()) {
            hash_map::Entry::Occupied(e) => {
                let position = *e.get();
                let heap_element = &mut self.heap[*e.get()];
                heap_element.value = value;
                debug_assert!(&heap_element.key == e.key());
                self.reprioritize_position(position, priority);
            }
            hash_map::Entry::Vacant(e) => {
                let position = self.heap.len();
                e.insert(position);
                self.heap.push(Entry {
                    priority,
                    key,
                    value,
                });
                self.swim_up(position);
            }
        }
    }

    pub fn peek(&self) -> Option<&V> {
        let entry = self.heap.get(0)?;
        Some(&entry.value)
    }

    pub fn pop(&mut self) -> Option<V> {
        if self.heap.is_empty() {
            debug_assert!(self.map.is_empty());
            return None;
        }
        let entry = self.heap.swap_remove(0);
        let position = self.map.remove(&entry.key);
        debug_assert_eq!(position, Some(0));

        if !self.heap.is_empty() {
            self.sink_down(0);
        }

        Some(entry.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let position = self.map.remove(&key)?;
        let entry = self.heap.swap_remove(position);
        debug_assert!(key == &entry.key);

        if self.heap.len() > position {
            self.sink_down(position);
        }
        Some(entry.value)
    }

    pub fn reprioritize(&mut self, key: &K, priority: P) -> Option<P> {
        let position = *self.map.get(&key)?;
        self.reprioritize_position(position, priority)
    }

    fn reprioritize_position(&mut self, position: usize, mut priority: P) -> Option<P> {
        let target = &mut self.heap[position].priority;
        std::mem::swap(target, &mut priority);
        if *target > priority {
            self.swim_up(position);
        } else {
            self.sink_down(position);
        }
        Some(priority)
    }

    fn swim_up(&mut self, position: usize) -> usize {
        self.sift(position, Self::lesser_parent)
    }

    fn sink_down(&mut self, position: usize) -> usize {
        self.sift(position, Self::greater_child)
    }

    fn sift<F: Fn(&Self, usize) -> Option<usize>>(&mut self, mut position: usize, f: F) -> usize {
        let original_key = self.heap[position].key.clone();
        while let Some(other) = f(self, position) {
            let other_key = self.heap[other].key.clone();
            self.heap.swap(other, position);
            debug_assert_eq!(self.map[&other_key], other);
            self.map.insert(other_key.clone(), position);

            position = other;
        }
        self.map.insert(original_key, position);
        position
    }

    fn lesser_parent(&self, position: usize) -> Option<usize> {
        if position == 0 {
            return None;
        }

        let parent = (position - 1) / 2;
        (self.heap[parent].priority < self.heap[position].priority).then_some(parent)
    }

    fn greater_child(&self, position: usize) -> Option<usize> {
        self.max_child(position)
            .filter(|child| self.heap[*child].priority > self.heap[position].priority)
    }

    fn max_child(&self, position: usize) -> Option<usize> {
        let left = 2 * position + 1;
        if left < self.heap.len() {
            let right = 2 * position + 2;
            if right < self.heap.len() {
                if self.heap[left].priority < self.heap[right].priority {
                    return Some(right);
                }
            }
            return Some(left);
        }
        None
    }
}

#[derive(Debug)]
struct Entry<P, K, V> {
    priority: P,
    key: K,
    value: V,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = PriorityMap::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.peek(), None);
        assert_eq!(map.pop(), None);

        map.insert(2, "b", "2");
        map.insert(7, "g", "7");
        map.insert(1, "a", "1");
        map.insert(6, "f", "6");
        map.insert(5, "e", "5");
        map.insert(3, "c", "3");
        map.insert(4, "d", "4");

        assert_eq!(map.len(), 7);
        assert_eq!(map.peek(), Some(&"7"));

        assert_eq!(map.pop(), Some("7"));
        assert_eq!(map.pop(), Some("6"));
        assert_eq!(map.pop(), Some("5"));
        assert_eq!(map.pop(), Some("4"));
        assert_eq!(map.pop(), Some("3"));
        assert_eq!(map.pop(), Some("2"));
        assert_eq!(map.pop(), Some("1"));

        assert_eq!(map.len(), 0);
        assert_eq!(map.peek(), None);
        assert_eq!(map.pop(), None);

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn reprioritize() {
        let mut map = PriorityMap::new();
        map.insert(1, "a", "1");
        map.insert(2, "b", "2");
        map.insert(3, "c", "3");

        map.reprioritize(&"b", 200);

        assert_eq!(map.pop(), Some("2"));
        assert_eq!(map.pop(), Some("3"));
        assert_eq!(map.pop(), Some("1"));
    }

    #[test]
    fn replace() {
        let mut map = PriorityMap::new();
        map.insert(1, "a", "1");
        map.insert(2, "b", "2");
        map.insert(3, "c", "3");

        map.insert(200, "b", "200");

        assert_eq!(map.pop(), Some("200"));
        assert_eq!(map.pop(), Some("3"));
        assert_eq!(map.pop(), Some("1"));
    }

    #[test]
    fn remove() {
        for (key, expected_value, expected_order) in [
            ("a", "1", ["3", "2"]),
            ("b", "2", ["3", "1"]),
            ("c", "3", ["2", "1"]),
        ] {
            let mut map = PriorityMap::new();
            assert!(map.remove(&key).is_none());

            map.insert(1, "a", "1");
            map.insert(2, "b", "2");
            map.insert(3, "c", "3");

            assert_eq!(map.remove(&key), Some(expected_value));
            assert!(map.remove(&key).is_none());

            assert_eq!(map.pop(), Some(expected_order[0]));
            assert_eq!(map.pop(), Some(expected_order[1]));
            assert!(map.pop().is_none());
        }
    }
}
