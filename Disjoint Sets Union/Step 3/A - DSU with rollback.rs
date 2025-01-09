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

mod cm_persist {
    pub struct DSU {
        parents: Vec<usize>,
        ranks: Vec<usize>,
        size: usize,
        history: Vec<Record>,
    }

    struct Record {
        actions: Vec<((usize, usize), (usize, usize), usize)>,
    }

    impl Record {
        fn new() -> Self {
            Self { actions: vec![] }
        }
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
                size,
                history: vec![Record::new()],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.parents[x] {
                // Do not compress
                // self.parents[x] = self.find(self.parents[x]);
                return self.find(self.parents[x]);
            }

            self.parents[x]
        }

        pub fn same(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            if x == y {
                return false;
            }

            if self.ranks[x] < self.ranks[y] {
                std::mem::swap(&mut y, &mut x);
            }

            let py = self.parents[y];
            let rx = self.ranks[x];
            let sz = self.size;
            let last = self.last_record_mut();
            last.actions.push(((y, py), (x, rx), sz));

            self.size -= 1;
            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];

            true
        }

        pub fn persist(&mut self) {
            self.history.push(Record::new());
        }

        pub fn rollback(&mut self) {
            assert!(!self.history.is_empty());

            let last = self.history.pop().unwrap();

            for ((y, py), (x, rx), sz) in last.actions.into_iter().rev() {
                self.parents[y] = py;
                self.ranks[x] = rx;
                self.size = sz;
            }
        }

        pub fn size(&self) -> usize {
            self.size
        }

        fn last_record_mut(&mut self) -> &mut Record {
            if self.history.is_empty() {
                self.history.push(Record::new());
            }

            let cnt = self.history.len();

            return &mut self.history[cnt - 1];
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

        let mut dsu = cm_persist::DSU::new(n);

        for _ in 0..m {
            let t = cin.next::<String>();

            match t.as_str() {
                "union" => {
                    let (x, y) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);
                    dsu.union(x, y);
                    println!("{}", dsu.size());
                }
                "persist" => {
                    dsu.persist();
                }
                "rollback" => {
                    dsu.rollback();
                    println!("{}", dsu.size());
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
