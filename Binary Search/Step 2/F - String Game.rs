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

fn ok(s: &[u8], t: &[u8], removed: &[bool]) -> bool {
    let mut j = 0;
    for i in 0..s.len() {
        if s[i] == t[j] && !removed[i] {
            j += 1;
        }

        if j == t.len() {
            return true;
        }
    }

    return false;
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();
        let t = cin.next::<String>().into_bytes();

        let n = s.len();
        let mut x = vec![0; n];
        for i in 0..n {
            x[i] = cin.next::<usize>();
        }

        let mut l = 0;
        let mut r = n;

        while r - l > 1 {
            let m = (r + l) / 2;
            let mut removed = vec![false; n];
            for i in 0..m {
                removed[x[i] - 1] = true;
            }
            if ok(&s, &t, &removed) {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
