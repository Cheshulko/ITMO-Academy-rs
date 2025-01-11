#![allow(non_snake_case)]
#![allow(unused_imports)]

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
    let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let _ = cin.next::<String>();
        let (N, M, K) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut A = vec![0; K];
        for i in 0..K {
            A[i] = cin.next::<usize>() - 1;
        }

        let mut edges = HashSet::<(usize, usize)>::new();
        for _ in 0..M {
            let (u, v) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);

            edges.insert((u, v));
            edges.insert((v, u));
        }

        for w in A.windows(2) {
            let (u, v) = (w[0], w[1]);

            if !edges.contains(&(u, v)) {
                println!("none");

                continue 'test;
            }
        }

        let mut cnt = vec![0; N];
        for &v in A.iter() {
            cnt[v] += 1;
        }

        if A[0] == A[K - 1] {
            for &v in A.iter() {
                if v == A[0] {
                    if cnt[v] > 2 {
                        println!("cycle");

                        continue 'test;
                    }
                } else {
                    if cnt[v] > 1 {
                        println!("cycle");

                        continue 'test;
                    }
                }
            }
            println!("simple cycle");
        } else {
            for &v in A.iter() {
                if cnt[v] > 1 {
                    println!("path");

                    continue 'test;
                }
            }
            println!("simple path");
        }
    }
}
