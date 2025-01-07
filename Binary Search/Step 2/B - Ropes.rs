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

fn ok(a: &Vec<usize>, k: usize, l: f64) -> bool {
    let mut cnt = 0;
    for &x in a.iter() {
        cnt += ((x as f64) / l) as usize;
    }

    cnt >= k
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

        let mut l = 0.;
        let mut r = *a.iter().max().unwrap() as f64 + 1.;

        const EPS: f64 = 1e-8;

        while r - l > EPS {
            let m = (r + l) / 2.;
            if ok(&a, k, m) {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
