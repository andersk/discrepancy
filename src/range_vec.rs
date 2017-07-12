use std::borrow::Borrow;
use std::ops::{Index, Range};
use super::monoid::Monoid;

#[derive(Clone)]
pub struct RangeVec<T: Monoid> {
    nodes: Vec<T>,
}

impl<T: Monoid> RangeVec<T> {
    pub fn new() -> RangeVec<T> {
        RangeVec { nodes: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        (self.nodes.len() + 1) / 2
    }

    pub fn concat_all(&self) -> T {
        if self.nodes.is_empty() {
            T::empty()
        } else {
            self.nodes[(!0usize >> 1) >> self.nodes.len().leading_zeros()].clone()
        }
    }

    pub fn concat_range(&self, range: Range<usize>) -> T {
        self.concat_map_range(|node| node, range)
    }

    pub fn concat_map_range<'a, U, BU, F>(&'a self, f: F, range: Range<usize>) -> U
    where
        U: Monoid,
        BU: Borrow<U>,
        F: Fn(&'a T) -> BU,
    {
        let Range { start: i, end: j } = range;
        if i == j {
            U::empty()
        } else {
            debug_assert!(i < j);
            let k = i.wrapping_sub(1) & !i & (!0 >> 1) >> (j & !i).leading_zeros();
            let (mut value, mut i) = (f(&self.nodes[2 * i + k]).borrow().clone(), i + k + 1);
            while i != j {
                let k = i.wrapping_sub(1) & !i & (!0 >> 1) >> (j & !i).leading_zeros();
                value = value.append(f(&self.nodes[2 * i + k]).borrow());
                i += k + 1;
            }
            value
        }
    }

    fn bubble(&mut self, mut i: usize) {
        loop {
            let k = i & !(i + 1);
            if i & (2 * k + 2) == 0 && i + k + 1 < self.nodes.len() {
                let j = i + k + 1;
                self.nodes[j] = self.nodes[i].append(
                    &self.nodes[j + 1 +
                                    ((j & !(j + 1)) >> 1 &
                                         (!0 >> 1) >>
                                             (self.nodes.len() &
                                                 !(j + 1))
                                                 .leading_zeros())],
                );
                i = j;
            } else if k == i {
                break;
            } else {
                let j = i - k - 1;
                self.nodes[j] = self.nodes[j - (((j + 1) & !j) >> 1)].append(&self.nodes[i]);
                i = j;
            }
        }
    }

    pub fn push(&mut self, value: T) {
        let n = self.len();
        if n == 0 {
            self.nodes.push(value);
        } else {
            let k = (n - 1) & !n;
            let node = self.nodes[2 * (n - 1) - k].append(&value);
            self.nodes.push(node);
            self.bubble(2 * n - 1);
            self.nodes.push(value);
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.nodes[2 * index] = value;
        self.bubble(2 * index);
    }
}

impl<T: Monoid> Index<usize> for RangeVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.nodes[2 * index]
    }
}
