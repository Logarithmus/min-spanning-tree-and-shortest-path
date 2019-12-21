use core::cmp::Ordering;

#[derive(Debug)]
pub struct MinHeap<T, F: Fn(&T, &T) -> Ordering> {
    data: Vec<T>,
    cmp: F,
}

impl<T, F: Fn(&T, &T) -> Ordering> MinHeap<T, F> {
    pub fn new(cmp: F) -> Self {
        MinHeap {
            data: Vec::new(),
            cmp,
        }
    }

    fn heapify(&mut self, i: usize) {
        let (l, r) = (2 * i + 1, 2 * i + 2);
        let mut min_i = i;
        if (l < self.len()) && ((self.cmp)(&self.data[l], &self.data[min_i]) == Ordering::Less) {
            min_i = l;
        }
        if (r < self.len()) && ((self.cmp)(&self.data[r], &self.data[min_i]) == Ordering::Less) {
            min_i = r;
        }
        if min_i != i {
            self.data.swap(min_i, i);
            self.heapify(min_i);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn pop_root(&mut self) -> Option<T> {
        match self.data.is_empty() {
            false => {
                let last_i = self.len() - 1;
                self.data.swap(0, last_i);
                let root = self.data.pop();
                self.heapify(0);
                root
            }
            true => None,
        }
    }

    pub fn insert(&mut self, x: T) {
        self.data.push(x);
        if self.len() > 1 {
            let mut i = self.len() - 1;
            let mut parent_i = (i - 1) / 2;
            loop {
                if (self.cmp)(&self.data[i], &self.data[parent_i]) == Ordering::Less {
                    self.data.swap(i, parent_i);
                }
                if parent_i == 0 {
                    break;
                }
                i = parent_i;
                parent_i = (i - 1) / 2;
            }
        }
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> From<(Vec<T>, F)> for MinHeap<T, F> {
    fn from((data, cmp): (Vec<T>, F)) -> Self {
        let len = data.len();
        let mut heap = MinHeap { data, cmp };
        (0..len / 2).rev().for_each(|i| heap.heapify(i));
        heap
    }
}
