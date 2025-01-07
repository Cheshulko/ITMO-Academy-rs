use core::num;
use std::cmp::*;
use std::collections::*;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;
use std::rc;
use std::slice::*;
use std::usize;
use std::vec;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

mod cm_lazy_seg_tree_capture {
    use std::ops::Range;

    // [l, r)
    pub struct LazySegmentTree<F, M, L, T: Default + Copy>
    where
        F: Fn(&T, &T) -> T,                // left, right
        M: Fn(&T, &T) -> T,                // old lazy, new lazy
        L: Fn(&T, &Range<usize>, &T) -> T, // node value, node range, lazy value
    {
        len: usize,
        tree: Vec<T>,
        lazy: Vec<Option<T>>,
        merge: F,
        merge_lazy: M,
        apply_lazy: L,
    }

    impl<F, M, L, T: Default + Copy> LazySegmentTree<F, M, L, T>
    where
        F: Fn(&T, &T) -> T,
        M: Fn(&T, &T) -> T,
        L: Fn(&T, &Range<usize>, &T) -> T,
    {
        pub fn from_vec(arr: &[T], default: T, merge: F, merge_lazy: M, apply_lazy: L) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = LazySegmentTree {
                len,
                tree: vec![default; 2 * pow_2_len - 1],
                lazy: vec![None; 2 * pow_2_len - 1],
                merge,
                merge_lazy,
                apply_lazy,
            };

            sgtr.build_recursive(arr, 0, 0..len);
            sgtr
        }

        // range: [l, r). O(log(n))
        pub fn query(&mut self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
        }

        // range: [l, r). O(log(n))
        pub fn update(&mut self, range: Range<usize>, val: T) {
            self.update_recursive(0, 0..self.len, &range, val);
        }

        fn build_recursive(&mut self, arr: &[T], idx: usize, range: Range<usize>) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end);
                self.tree[idx] = (self.merge)(&self.tree[2 * idx + 1], &self.tree[2 * idx + 2]);
            }
        }

        fn query_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            query_range: &Range<usize>,
        ) -> Option<T> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }
            if element_range.start >= query_range.start && element_range.end <= query_range.end {
                return Some(self.tree[idx]);
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            match (left, right) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => Some((self.merge)(&l, &r)),
            }
        }

        fn update_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            update_range: &Range<usize>,
            val: T,
        ) {
            if element_range.start >= update_range.end || element_range.end <= update_range.start {
                return;
            }
            if element_range.end - element_range.start == 1 {
                self.apply(idx, &element_range, val);
                return;
            }
            if element_range.start >= update_range.start && element_range.end <= update_range.end {
                self.apply(idx, &element_range, val);
                return;
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            self.update_recursive(idx * 2 + 1, element_range.start..mid, update_range, val);
            self.update_recursive(idx * 2 + 2, mid..element_range.end, update_range, val);
            self.tree[idx] = (self.merge)(&self.tree[idx * 2 + 1], &self.tree[idx * 2 + 2]);
        }

        fn apply(&mut self, idx: usize, element_range: &Range<usize>, val: T) {
            self.tree[idx] = (self.apply_lazy)(&self.tree[idx], element_range, &val);
            self.lazy[idx] = match self.lazy[idx] {
                Some(lazy) => Some((self.merge_lazy)(&lazy, &val)),
                None => Some(val),
            };
        }

        fn propagate(&mut self, idx: usize, element_range: &Range<usize>) {
            if let Some(lazy) = self.lazy[idx] {
                let mid = element_range.start + (element_range.end - element_range.start) / 2;
                self.apply(2 * idx + 1, &(element_range.start..mid), lazy);
                self.apply(2 * idx + 2, &(mid..element_range.end), lazy);
            }
            self.lazy[idx] = None;
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let arr = vec![0; n];
        let mut tree = cm_lazy_seg_tree_capture::LazySegmentTree::from_vec(
            &arr,
            usize::MAX,
            |&left, &right| left.min(right),
            |&lazy_1, &lazy_2| lazy_1 + lazy_2,
            |node_val, _, &lazy| node_val + lazy,
        );

        for _ in 0..m {
            let t = cin.next::<usize>();

            if t == 1 {
                let (l, r, v) = (
                    cin.next::<usize>(),
                    cin.next::<usize>(),
                    cin.next::<usize>(),
                );
                tree.update(l..r, v);
            } else {
                let (l, r) = (cin.next::<usize>(), cin.next::<usize>());
                let ans = tree.query(l..r).unwrap();
                println!("{ans}");
            }
        }
    }
}
