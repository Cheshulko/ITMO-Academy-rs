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
        let (N, M) = (cin.next::<usize>(), cin.next::<usize>());

        let mut A = vec![0; N];
        for i in 0..N {
            A[i] = cin.next::<i64>();
        }

        let mut B = vec![0; M];
        for i in 0..M {
            B[i] = cin.next::<i64>();
        }

        let mut C = vec![0; N + M];

        let mut i = 0;
        let mut j = 0;
        while i < N && j < M {
            if A[i] < B[j] {
                C[i + j] = A[i];
                i += 1;
            } else {
                C[i + j] = B[j];
                j += 1;
            }
        }
        while i < N {
            C[i + j] = A[i];
            i += 1;
        }
        while j < M {
            C[i + j] = B[j];
            j += 1;
        }

        for x in C.into_iter() {
            print!("{} ", x);
        }
        println!();
    }
}
