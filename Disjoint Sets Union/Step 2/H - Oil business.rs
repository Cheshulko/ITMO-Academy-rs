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
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
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

            true
        }
    }
}

type Edge = (usize, usize, usize, usize); // from - to - weight

fn kruskal(n: usize, edges: &mut Vec<Edge>) -> Vec<Edge> {
    let m = edges.len();
    edges.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    let mut ans = vec![];
    let mut dsu = cm::DSU::new(n);
    for i in 0..m {
        let from = edges[i].0;
        let to = edges[i].1;
        if !dsu.same(from, to) {
            ans.push(edges[i]);
            dsu.union(from, to);
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, m, mut s) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut edges = vec![];
        for i in 0..m {
            let edge = (
                cin.next::<usize>() - 1,
                cin.next::<usize>() - 1,
                cin.next::<usize>(),
                i,
            );
            edges.push(edge);
        }

        let mst = kruskal(n, &mut edges);
        let mut need = vec![false; m];
        for i in 0..mst.len() {
            need[mst[i].3] = true;
        }

        let mut ans = vec![];
        for edge in edges.into_iter().rev() {
            if edge.2 <= s && !need[edge.3] {
                ans.push(edge.3);
                s -= edge.2;
            }
        }
        println!("{}", ans.len());
        ans.sort_unstable();
        for x in ans.into_iter() {
            print!("{} ", x + 1);
        }
        println!();
    }

    Ok(())
}
