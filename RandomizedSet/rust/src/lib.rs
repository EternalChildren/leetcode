// 164ms 5.2MB
extern crate rand;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            return false;
        }
        self.set.insert(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.set.contains(&val) {
            return false;
        }
        self.set.remove(&val);
        true
    }

    fn get_random(&self) -> i32 {
        if self.set.is_empty() {
            return 0;
        }
        let i = thread_rng().gen_range(0 as usize, self.set.len());
        let v: Vec<_> = self.set.iter().cloned().collect();
        v[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut random_set = RandomizedSet::new();

        assert_eq!(random_set.insert(1), true);
        assert_eq!(random_set.remove(2), false);
        assert_eq!(random_set.insert(2), true);
        assert!(random_set.get_random() == 1 || random_set.get_random() == 2);
        assert_eq!(random_set.remove(1), true);
        assert_eq!(random_set.insert(2), false);
        assert_eq!(random_set.get_random(), 2);
        assert_eq!(random_set.get_random(), 2);
        assert_eq!(random_set.get_random(), 2);
    }
}
