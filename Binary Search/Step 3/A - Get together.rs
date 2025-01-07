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

fn ok(a: &Vec<(f64, f64)>, t: f64) -> bool {
    let mut l = a[0].0 - t * a[0].1;
    let mut r = a[0].0 + t * a[0].1;

    for &(x, y) in a.iter().skip(1) {
        l = l.max(x - t * y);
        r = r.min(x + t * y);
    }

    l <= r
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![];
        for _ in 0..n {
            let (x, y) = (cin.next::<f64>(), cin.next::<f64>());
            a.push((x, y));
        }

        let mut l = 0.;
        let mut r = 1e18;

        for _ in 0..100 {
            let m = (r + l) / 2.;

            if ok(&a, m) {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{r}");
    }
}
