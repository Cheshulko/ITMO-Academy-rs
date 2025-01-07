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
    let n = a.len();
    let mut cnt = 1;
    let mut cur = 0;
    let mut i = 1;

    while i < n {
        while i < n && a[i] - a[cur] < x {
            i += 1;
        }

        if i == n {
            return cnt >= k;
        }

        cur = i;
        cnt += 1;
    }

    return cnt >= k;
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

        let mut l = usize::MAX;
        for i in 0..k - 1 {
            l = l.min(a[i + 1] - a[i]);
        }
        let mut r = a[n - 1] - a[0] + 1;

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
