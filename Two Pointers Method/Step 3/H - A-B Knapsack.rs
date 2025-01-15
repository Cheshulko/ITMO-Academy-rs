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
        let (N, M, W, A_W, B_W) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut A = vec![0; N];
        for i in 0..N {
            A[i] = cin.next::<usize>();
        }
        A.sort_unstable();

        let mut B = vec![0; M];
        for i in 0..M {
            B[i] = cin.next::<usize>();
        }
        B.sort_unstable();
        B.reverse();

        let mut B_i = 0;
        let mut cur_sum = A.iter().sum::<usize>();
        let mut cur_w = A_W * N;

        while B_i < M && cur_w + B_W <= W {
            cur_w += B_W;
            cur_sum += B[B_i];

            B_i += 1;
        }

        let mut ans = 0;
        if cur_w <= W {
            ans = cur_sum;
        }

        for A_i in 0..N {
            cur_sum -= A[A_i];
            cur_w -= A_W;

            while B_i < M && cur_w + B_W <= W {
                cur_w += B_W;
                cur_sum += B[B_i];

                B_i += 1;
            }

            if cur_w <= W {
                ans = ans.max(cur_sum);
            }
        }

        println!("{ans}");
    }
}
