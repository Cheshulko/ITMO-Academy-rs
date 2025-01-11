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
        let (N,) = (cin.next::<usize>(),);

        let mut Deg = BTreeSet::<(usize, usize)>::new();
        for i in 0..N {
            let d = cin.next::<usize>();
            if d != 0 {
                Deg.insert((d, i));
            }
        }

        let mut edges = vec![];

        while !Deg.is_empty() {
            let first = Deg.pop_first().unwrap();

            assert!(Deg.len() >= first.0);

            let mut to_update = vec![];
            for _ in 0..first.0 {
                let mut next = Deg.pop_last().unwrap();

                edges.push((first.1 + 1, next.1 + 1));

                next.0 -= 1;
                if next.0 > 0 {
                    to_update.push(next);
                }
            }

            for updated in to_update {
                Deg.insert(updated);
            }
        }

        println!("{}", edges.len());
        for (u, v) in edges {
            println!("{} {}", u, v);
        }
    }
}
