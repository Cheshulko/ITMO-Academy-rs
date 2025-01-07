use core::num;
use std::cmp::*;
use std::collections::*;
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

fn ok(a: &Vec<(i64, i64)>, x: i64) -> i64 {
    let mut left = 0;

    for &(l, mut r) in a.iter() {
        r = r.min(x - 1);
        if r >= l {
            left += r - l + 1;
        }
    }

    left
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<i64>());

        let mut v = vec![];
        for _ in 0..n {
            let (l, r) = (cin.next::<i64>(), cin.next::<i64>());
            v.push((l, r));
        }

        let mut l = -1_000_000_000_000_000;
        let mut r = 1_000_000_000_000_000;

        while r - l > 1 {
            let m = (l + r) / 2;

            let left = ok(&v, m);
            if left <= k {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
