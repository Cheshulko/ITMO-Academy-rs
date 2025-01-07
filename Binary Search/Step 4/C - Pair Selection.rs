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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Wrap(pub f64);

impl Eq for Wrap {}

impl PartialOrd for Wrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Wrap {
    fn cmp(&self, other: &Wrap) -> Ordering {
        other.partial_cmp(self).unwrap()
    }
}

fn ok(a: &Vec<(usize, usize)>, k: usize, lim: f64) -> (bool, f64) {
    let n = a.len();

    let mut b = a
        .iter()
        .map(|&(x, y)| (Wrap(x as f64 - y as f64 * lim), x, y))
        .collect::<Vec<_>>();

    b.sort_unstable();
    let mut num = 0.;
    let mut den = 0.;

    for i in 0..k {
        num += b[n - 1 - i].1 as f64;
        den += b[n - 1 - i].2 as f64;
    }

    (num / den >= lim, num / den)
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![];
        for i in 0..n {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
            a.push((x, y));
        }

        let mut l = 0.;
        let mut r = 1e18;

        for _ in 0..100 {
            let m = (l + r) / 2.;

            let (can, _) = ok(&a, k, m);
            if can {
                l = m;
            } else {
                r = m;
            }
        }

        let (_, x) = ok(&a, k, l);
        println!("{x}");
    }
}
