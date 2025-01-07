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

fn ok(cnt: usize, w: usize, h: usize, a: usize) -> bool {
    let c1 = a / w;
    let c2 = a / h;

    c1 * c2 >= cnt
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (w, h, n) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut l = 0;
        let mut r = (((n as f64).sqrt() as usize) + 1) * w.max(h);

        while r - l > 1 {
            let m = (r + l) / 2;
            if ok(n, w, h, m) {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{r}");
    }
}
