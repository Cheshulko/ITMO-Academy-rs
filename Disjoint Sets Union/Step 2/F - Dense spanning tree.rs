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
        pub max: usize,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
                max: 1,
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.parents[x] {
                self.parents[x] = self.find(self.parents[x]);
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

            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];
            self.max = self.max.max(self.ranks[x]);

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

        let mut edges = vec![];
        for _ in 0..m {
            let edge = (
                cin.next::<usize>() - 1,
                cin.next::<usize>() - 1,
                cin.next::<i64>(),
            );
            edges.push(edge);
        }

        edges.sort_unstable_by_key(|a| a.2);

        let mut ans = i64::MAX;

        'out: for i in 0..m {
            let mut dsu = cm::DSU::new(n);
            let cur = edges[i].2;
            for j in i..m {
                dsu.union(edges[j].0, edges[j].1);
                if dsu.max == n {
                    ans = ans.min(edges[j].2 - cur);
                    continue 'out;
                }
                if edges[j].2 - cur >= ans {
                    continue 'out;
                }
            }
        }

        if ans != i64::MAX {
            println!("YES");
            println!("{ans}");
        } else {
            println!("NO");
        }
    }

    Ok(())
}
