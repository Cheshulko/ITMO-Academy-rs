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
        let (N, mut P) = (cin.next::<usize>(), cin.next::<usize>());

        let mut A = vec![0; 2 * N];
        let mut s = 0;
        for i in 0..N {
            A[i] = cin.next::<usize>();
            A[i + N] = A[i];
            s += A[i];
        }

        let ans = N * (P / s);
        P %= s;

        let mut cnt = usize::MAX;
        let mut st = 0;
        let mut l = 0;
        let mut s = 0;

        if P > 0 {
            for r in 0..2 * N {
                s += A[r];

                while s - A[l] >= P {
                    s -= A[l];
                    l += 1;
                }

                if s >= P {
                    if cnt > r - l + 1 {
                        cnt = r - l + 1;
                        st = l;
                    }
                }
            }
        } else {
            cnt = 0;
        }

        println!("{} {}", st + 1, ans + cnt);
    }
}
