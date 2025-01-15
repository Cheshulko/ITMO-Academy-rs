#![allow(non_snake_case)]
#![allow(unused_imports)]

use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
use std::mem::swap;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &std::path::Path) -> Self {
        use std::fs::File;

        let tokens = VecDeque::new();
        let file = File::open(&path).expect("Expect file exists");
        Self {
            reader: Box::new(BufReader::new(file)),
            tokens,
        }
    }
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self {
            reader: Box::new(BufReader::new(std::io::stdin())),
            tokens,
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            self.reader.read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (N, S) = (cin.next::<usize>(), cin.next::<usize>());

        let mut W = vec![0; N];
        for i in 0..N {
            W[i] = cin.next::<usize>();
        }

        let mut C = vec![0; N];
        for i in 0..N {
            C[i] = cin.next::<usize>();
        }

        let mut ans = 0;
        let mut cur_w = 0;
        let mut cur_c = 0;
        let mut l = 0;

        for r in 0..N {
            cur_w += W[r];
            cur_c += C[r];

            while cur_w > S {
                cur_w -= W[l];
                cur_c -= C[l];
                l += 1;
            }

            ans = ans.max(cur_c);
        }

        println!("{ans}");
    }
}
