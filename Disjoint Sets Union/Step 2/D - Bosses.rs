use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &Path) -> Self {
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

mod cm {
    pub struct DSU {
        parents: Vec<usize>,
        pub depth: Vec<usize>,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                depth: vec![0; size],
            }
        }

        pub fn find(&mut self, x: usize) -> (usize, usize) {
            if x != self.parents[x] {
                let (p, d) = self.find(self.parents[x]);
                self.parents[x] = p;
                self.depth[x] += d;
            }

            (self.parents[x], self.depth[x])
        }

        pub fn same(&mut self, mut x: usize, mut y: usize) -> bool {
            (x, _) = self.find(x);
            (y, _) = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            (x, _) = self.find(x);
            (y, _) = self.find(y);

            if x == y {
                return false;
            }

            self.parents[x] = y;
            self.depth[x] = 1 + self.depth[y];

            true
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut dsu = cm::DSU::new(n);
        for _ in 0..m {
            let t = cin.next::<usize>();
            match t {
                1 => {
                    let (x, y) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);
                    dsu.union(x, y);
                }
                2 => {
                    let c = cin.next::<usize>() - 1;
                    let _ = dsu.find(c);
                    println!("{}", dsu.depth[c]);
                }

                _ => unreachable!(),
            }
        }
        println!();
    }

    Ok(())
}
