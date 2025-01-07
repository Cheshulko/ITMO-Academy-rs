use core::num;
use std::cmp::*;
use std::collections::*;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

fn ok(a: &Vec<usize>, b: &Vec<usize>, x: usize) -> usize {
    let n = a.len();
    let mut left = 0;

    for i in 0..n {
        if a[i] >= x {
            return left;
        }

        left += b.partition_point(|&y| y < x - a[i]);
    }

    left
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut b = vec![0; n];
        for i in 0..n {
            b[i] = cin.next::<usize>();
        }

        a.sort_unstable();
        b.sort_unstable();

        let mut l = 0;
        let mut r = 2 * 1_000_000_000 + 1;

        while r - l > 1 {
            let m = (l + r) / 2;

            let left = ok(&a, &b, m);

            if left < k {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
