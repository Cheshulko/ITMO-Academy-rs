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

fn ok(a: &Vec<usize>, k: usize, x: usize) -> bool {
    let mut p = k * x;

    for &y in a.iter() {
        p -= p.min(x.min(y));
    }

    p == 0
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let k = cin.next::<usize>();
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }
        a.sort_unstable();

        let mut l = 0;
        let mut r = a.iter().sum();

        while r - l > 1 {
            let m = (r + l) / 2;

            if ok(&a, k, m) {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
