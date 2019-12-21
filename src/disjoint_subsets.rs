use std::cmp::{max, min};

pub struct DisjointSubsets {
    parent: Vec<usize>,
    set_size: Vec<usize>,
}

impl DisjointSubsets {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            set_size: vec![1; size],
        }
    }

    pub fn find_set(&mut self, elem: usize) -> usize {
        if self.parent[elem] != elem {
            self.parent[elem] = self.find_set(self.parent[elem]);
        }
        self.parent[elem]
    }

    pub fn union_sets(&mut self, elem1: usize, elem2: usize) {
        let set1 = self.find_set(elem1);
        let set2 = self.find_set(elem2);
        if set1 != set2 {
            let (min_set, max_set) = (min(set1, set2), max(set1, set2));
            self.parent[min_set] = max_set;
            self.set_size[max_set] += self.set_size[min_set];
        }
    }
}
