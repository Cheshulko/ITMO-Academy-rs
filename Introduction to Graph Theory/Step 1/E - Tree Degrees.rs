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

        let mut A = vec![0; N];
        for i in 0..N {
            A[i] = cin.next::<usize>();
        }

        A.sort_unstable();

        let mut leafs = 0;
        for i in 0..N {
            if i == N - 1 {
                if leafs != A[i] {
                    println!("NO");
                } else {
                    println!("YES");
                }
            } else {
                if A[i] == 0 {
                    continue;
                }
                if A[i] == 1 {
                    leafs += 1;
                } else {
                    let more = A[i] - 1;
                    if leafs < more {
                        println!("NO");

                        continue 'test;
                    } else {
                        leafs -= more;
                        leafs += 1;
                    }
                }
            }
        }
    }
}
