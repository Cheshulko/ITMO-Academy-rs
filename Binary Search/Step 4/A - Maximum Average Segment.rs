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

fn ok(a: &Vec<usize>, d: usize, x: f64) -> (bool, usize, usize) {
    let n = a.len();
    // [l..r]

    let aa = a.iter().map(|&y| y as f64 - x).collect::<Vec<_>>();

    let mut pref = vec![0.; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + aa[i];
    }

    let mut pref_min = vec![(0., 0); n + 1];
    for i in 1..=n {
        if pref_min[i - 1].0 > pref[i] {
            pref_min[i] = (pref[i], i);
        } else {
            pref_min[i] = pref_min[i - 1];
        }
    }

    for i in d..=n {
        if pref[i] - pref_min[i - d].0 >= 0. {
            return (true, pref_min[i - d].1 + 1, i);
        }
    }

    (false, 0, 0)
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, d) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut l = 0.;
        let mut r = 101.;

        for _ in 0..100 {
            let m = (l + r) / 2.;

            let (can, _, _) = ok(&a, d, m);
            if can {
                l = m;
            } else {
                r = m;
            }
        }

        let (_, l, r) = ok(&a, d, l);
        println!("{l} {r}");
    }
}
