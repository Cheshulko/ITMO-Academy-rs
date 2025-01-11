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

        let mut Out = vec![0; N];
        let mut In = vec![0; N];
        for _ in 0..M {
            let (u, v) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);

            Out[u] += 1;
            In[v] += 1;
        }

        let sources = In.into_iter().filter(|&v| v == 0).count();
        let sinks = Out.into_iter().filter(|&v| v == 0).count();

        println!("{} {}", sources, sinks);
    }
}
