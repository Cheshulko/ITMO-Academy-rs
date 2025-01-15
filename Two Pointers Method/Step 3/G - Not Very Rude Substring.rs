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
        let (N, C) = (cin.next::<usize>(), cin.next::<usize>());

        let S = cin
            .next::<String>()
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();

        let mut A_cnt = 0;
        let mut B_cnt = 0;

        let mut cur = 0;
        let mut ans = 0;
        let mut l = 0;

        for r in 0..N {
            if S[r] == 0 {
                A_cnt += 1;
            } else if S[r] == 1 {
                B_cnt += 1;
                cur += A_cnt;
            }

            while cur > C {
                if S[l] == 0 {
                    A_cnt -= 1;
                    cur -= B_cnt;
                } else if S[l] == 1 {
                    B_cnt -= 1;
                }

                l += 1;
            }

            ans = ans.max(1 + r - l);
        }

        println!("{ans}");
    }
}
