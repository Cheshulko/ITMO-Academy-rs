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

fn ok(x: f64, c: f64) -> bool {
    x * x + x.sqrt() - c <= 0.
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let c = cin.next::<f64>();

        let mut l = 0.;
        let mut r = c.sqrt() + 1.;

        for _ in 0..100 {
            let m = (r + l) / 2.;
            if ok(m, c) {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
