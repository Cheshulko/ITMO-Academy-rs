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

        let mut A = HashSet::<usize>::new();
        for _ in 0..K {
            let v = cin.next::<usize>() - 1;
            A.insert(v);
        }

        let mut adj = vec![vec![]; N];
        for _ in 0..M {
            let (u, v) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);

            adj[v].push(u);
            adj[u].push(v);
        }

        let mut B = HashSet::<usize>::new();

        fn add_comp(cur: usize, adj: &Vec<Vec<usize>>, B: &mut HashSet<usize>) {
            B.insert(cur);

            for &to in adj[cur].iter() {
                if !B.contains(&to) {
                    add_comp(to, adj, B);
                }
            }
        }

        for &cur in A.iter() {
            add_comp(cur, &adj, &mut B);
        }

        if A == B {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
