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
        let (N, _M) = (cin.next::<usize>(), cin.next::<usize>());

        let S = cin
            .next::<String>()
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();

        let C = cin
            .next::<String>()
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();

        let mut c_cnt = vec![0; 26];
        for c in C.into_iter() {
            c_cnt[c] += 1;
        }

        let mut ans = 0;
        let mut cur_cnt = vec![0; 26];
        let mut l = 0;

        let is_valid = |c_cnt: &Vec<usize>, cur_cnt: &Vec<usize>| -> bool {
            c_cnt.iter().zip(cur_cnt.iter()).all(|(c, cur)| c >= cur)
        };

        for r in 0..N {
            cur_cnt[S[r]] += 1;

            while !is_valid(&c_cnt, &cur_cnt) {
                cur_cnt[S[l]] -= 1;
                l += 1;
            }

            ans += 1 + r - l;
        }

        println!("{ans}");
    }
}
