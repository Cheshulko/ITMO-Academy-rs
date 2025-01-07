use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

mod cm_seg_tree {
    use std::fmt::Debug;
    use std::ops::Range;

    // [l, r)
    pub struct SegmentTree<T: Debug + Default + Copy> {
        len: usize,
        tree: Vec<T>,
        merge: fn(T, T) -> T,
    }

    impl<T: Debug + Default + Copy> SegmentTree<T> {
        pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = SegmentTree {
                len,
                tree: vec![T::default(); 2 * pow_2_len - 1],
                merge,
            };

            sgtr.build_recursive(arr, 0, 0..len, merge);
            sgtr
        }

        // range: [l, r)
        pub fn query(&self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
        }

        pub fn update(&mut self, idx: usize, val: T) {
            self.update_recursive(0, 0..self.len, idx, val);
        }

        fn build_recursive(
            &mut self,
            arr: &[T],
            idx: usize,
            range: Range<usize>,

            merge: fn(T, T) -> T,
        ) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid, merge);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end, merge);
                self.tree[idx] = merge(self.tree[2 * idx + 1], self.tree[2 * idx + 2]);
            }
        }

        fn query_recursive(
            &self,
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
            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            match (left, right) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => Some((self.merge)(l, r)),
            }
        }

        fn update_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            element_idx: usize,
            val: T,
        ) {
            if element_range.start > element_idx || element_range.end <= element_idx {
                return;
            }
            if element_range.end - element_range.start == 1 && element_range.start == element_idx {
                self.tree[idx] = val;
                return;
            }
            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.update_recursive(idx * 2 + 1, element_range.start..mid, element_idx, val);
            self.update_recursive(idx * 2 + 2, mid..element_range.end, element_idx, val);
            self.tree[idx] = (self.merge)(self.tree[idx * 2 + 1], self.tree[idx * 2 + 2]);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MaxSegment {
    max_pref: i64,
    max_suf: i64,
    max_seg: i64,
    sum: i64,
}

impl MaxSegment {
    pub fn one(v: i64) -> Self {
        Self {
            max_pref: v,
            max_suf: v,
            max_seg: v,
            sum: v,
        }
    }

    pub fn new(max_pref: i64, max_suf: i64, max_seg: i64, sum: i64) -> Self {
        Self {
            max_pref,
            max_suf,
            max_seg,
            sum,
        }
    }
}

impl Default for MaxSegment {
    fn default() -> Self {
        Self {
            max_pref: 0,
            max_suf: 0,
            max_seg: 0,
            sum: 0,
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<i64>();
            arr.push(MaxSegment::one(x));
        }

        let mut tree = cm_seg_tree::SegmentTree::from_vec(&arr, |left, right| {
            MaxSegment::new(
                left.max_pref.max(left.sum + right.max_pref),
                right.max_suf.max(right.sum + left.max_suf),
                left.max_seg
                    .max(right.max_seg)
                    .max(left.max_suf + right.max_pref),
                left.sum + right.sum,
            )
        });

        let max_seg = tree.query(0..n).unwrap();
        println!("{sum}", sum = max_seg.max_seg.max(0));

        for _ in 0..m {
            let (i, v) = (cin.next::<usize>(), cin.next::<i64>());
            tree.update(i, MaxSegment::one(v));
            let max_seg = tree.query(0..n).unwrap();
            println!("{sum}", sum = max_seg.max_seg.max(0));
        }
    }
}
