use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::i64;
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
        ranks: Vec<usize>,
        pub parity: Vec<usize>,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
                parity: vec![0; size],
            }
        }

        pub fn find(&mut self, x: usize) -> (usize, usize) {
            if x != self.parents[x] {
                let (p, d) = self.find(self.parents[x]);
                self.parents[x] = p;
                self.parity[x] = (self.parity[x] + d) % 2;
            }

            (self.parents[x], self.parity[x])
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            let (x, _) = self.find(x);
            let (y, _) = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            let (mut par_x, px) = self.find(x);
            let (mut par_y, py) = self.find(y);

            if par_x == par_y {
                return false;
            }

            assert!(px == self.parity[x]);
            assert!(py == self.parity[y]);

            if self.ranks[par_x] > self.ranks[par_y] {
                std::mem::swap(&mut par_x, &mut par_y);
                std::mem::swap(&mut y, &mut x);
            }

            self.parents[par_x] = par_y;
            // parity(a) + 1 == parity(b) + x
            self.parity[par_x] = (1 + self.parity[y] + self.parity[x]) % 2;
            self.ranks[par_y] += self.ranks[par_x];

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

        let mut shift = 0;
        let mut dsu = cm::DSU::new(n);
        for _ in 0..m {
            let (t, a, b) = (
                cin.next::<usize>(),
                cin.next::<usize>() - 1,
                cin.next::<usize>() - 1,
            );

            let x = (a + shift) % n;
            let y = (b + shift) % n;

            if t == 0 {
                assert!(dsu.union(x, y));
            } else {
                let (cx, px) = dsu.find(x);
                let (cy, py) = dsu.find(y);

                if px == py || cx != cy {
                    println!("YES");
                    shift = (shift + 1) % n;
                } else {
                    println!("NO");
                }
            }
        }
    }

    Ok(())
}
