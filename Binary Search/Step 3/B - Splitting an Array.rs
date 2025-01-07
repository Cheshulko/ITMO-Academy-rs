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
    let mut s = 0;
    let mut cnt = 0;
    let mut i = 0;

    while i < n {
        while i < n && s + a[i] <= x {
            s += a[i];
            i += 1;
        }
        if i < n && a[i] > x {
            return false;
        }
        cnt += 1;
        s = 0;
    }

    if i == n {
        return cnt <= k;
    } else {
        return false;
    }
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

        let mut l = 0;

        let mut x = 0;
        for i in 0..n - k + 1 {
            x += a[i];
        }
        let mut r = x;
        for i in (n - k + 1)..n {
            r = r.max(a[i]);
        }

        while r - l > 1 {
            let m = (r + l) / 2;

            if ok(&a, k, m) {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{r}");
    }
}
