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
    let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let _ = cin.next::<String>();
        let (N, M) = (cin.next::<usize>(), cin.next::<usize>());

        let mut mtx = vec![vec![false; N]; N];
        let mut edges = vec![];
        for _ in 0..M {
            let (u, v) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);

            mtx[u][v] = true;
            mtx[v][u] = true;
            edges.push((u, v));
        }

        let mut ans = 0;
        for e1 in 0..M {
            let (e1_v, e1_u) = edges[e1];
            for e2 in e1 + 1..M {
                let (e2_v, e2_u) = edges[e2];

                if [e1_v, e1_u, e2_v, e2_u]
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .len()
                    != 4
                {
                    continue;
                }

                if mtx[e1_v][e2_v] && mtx[e1_v][e2_u] && mtx[e1_u][e2_v] && mtx[e1_u][e2_u] {
                    ans += 1;
                }
            }
        }

        println!("{}", ans / 3);
    }
}
