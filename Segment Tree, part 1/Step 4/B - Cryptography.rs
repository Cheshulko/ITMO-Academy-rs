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

mod cm_seg_tree_capture {
    use std::ops::Range;

    // [l, r)
    pub struct SegmentTree<F, T: Default + Copy>
    where
        F: Fn(&T, &T) -> T,
    {
        len: usize,
        tree: Vec<T>,
        merge: F,
    }

    impl<F, T: Default + Copy> SegmentTree<F, T>
    where
        F: Fn(&T, &T) -> T,
    {
        pub fn from_vec(arr: &[T], merge: F) -> Self {
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

            sgtr.build_recursive(arr, 0, 0..len);
            sgtr
        }

        // range: [l, r). O(log(n))
        pub fn query(&self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
        }

        // range: [l, r). O(log(n))
        pub fn update(&mut self, idx: usize, val: T) {
            self.update_recursive(0, 0..self.len, idx, val);
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
                (Some(l), Some(r)) => Some((self.merge)(&l, &r)),
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
            self.tree[idx] = (self.merge)(&self.tree[idx * 2 + 1], &self.tree[idx * 2 + 2]);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Matrix<T: Add + Mul> {
    r0: [T; 2],
    r1: [T; 2],
}

impl<T: Add + Mul> Matrix<T> {
    pub fn new(a00: T, a01: T, a10: T, a11: T) -> Self {
        Matrix {
            r0: [a00, a01],
            r1: [a10, a11],
        }
    }
}

impl<T: Display + Add + Mul> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{a00} {a01}\n", a00 = self.r0[0], a01 = self.r0[1])?;
        write!(f, "{a10} {a11}", a10 = self.r1[0], a11 = self.r1[1])?;
        Ok(())
    }
}

impl Default for Matrix<i64> {
    fn default() -> Self {
        Self {
            r0: [1, 0],
            r1: [0, 1],
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix {
            r0: [
                self.r0[0] * rhs.r0[0] + self.r0[1] * rhs.r1[0],
                self.r0[0] * rhs.r0[1] + self.r0[1] * rhs.r1[1],
            ],
            r1: [
                self.r1[0] * rhs.r0[0] + self.r1[1] * rhs.r1[0],
                self.r1[0] * rhs.r0[1] + self.r1[1] * rhs.r1[1],
            ],
        }
    }
}

impl<T: Copy + Add + Mul + Rem<Output = T>> Rem<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Matrix::<T>::new(
            self.r0[0] % rhs,
            self.r0[1] % rhs,
            self.r1[0] % rhs,
            self.r1[1] % rhs,
        )
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (r, n, m) = (cin.next::<i64>(), cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![];
        for _ in 0..n {
            let (a00, a01) = (cin.next::<i64>(), cin.next::<i64>());
            let (a10, a11) = (cin.next::<i64>(), cin.next::<i64>());
            let _ = cin.next::<String>();

            a.push(Matrix::new(a00, a01, a10, a11));
        }

        let tree = cm_seg_tree_capture::SegmentTree::from_vec(&a, |&a, &b| (a * b) % r);

        for _ in 0..m {
            let (l, r) = (cin.next::<usize>(), cin.next::<usize>());
            let s = tree.query((l - 1)..r).unwrap();

            println!("{s}\n");
        }
    }
}
