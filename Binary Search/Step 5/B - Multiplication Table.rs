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

fn ok(n: usize, x: usize) -> usize {
    let mut left = 0;

    for i in 1..=n {
        left += ((x - 1) / i).min(n);
    }

    left
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut l = 0;
        let mut r = 100_000 * 100_000 + 1;

        while r - l > 1 {
            let m = (l + r) / 2;

            let left = ok(n, m);

            if left < k {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
