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

struct Dsu {
    parent: Vec<usize>,
    list: Vec<(Vec<usize>, usize, usize)>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            list: (0..n).map(|x| (vec![x], x, x)).collect(),
        }
    }

    pub fn find(&self, x: usize) -> usize {
        self.parent[x]
    }

    pub fn min(&self, x: usize) -> usize {
        let x = self.find(x);
        self.list[x].1
    }

    pub fn max(&self, x: usize) -> usize {
        let x = self.find(x);
        self.list[x].2
    }

    pub fn len(&self, x: usize) -> usize {
        let x = self.find(x);
        self.list[x].0.len()
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }

        if self.list[x].0.len() > self.list[y].0.len() {
            swap(&mut x, &mut y);
        }

        let mut tmp = vec![];
        for &xx in self.list[x].0.iter() {
            self.parent[xx] = y;
            tmp.push(xx);
        }

        self.list[y].0.extend(tmp.into_iter());
        self.list[y].1 = self.list[y].1.min(self.list[x].1);
        self.list[y].2 = self.list[y].2.max(self.list[x].2);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());
        let mut dsu = Dsu::new(n + 1);

        for _ in 0..m {
            let t = cin.next::<String>();
            if &t == "union" {
                let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
                dsu.union(x, y);
            } else {
                let x = cin.next::<usize>();
                let (min, max, len) = (dsu.min(x), dsu.max(x), dsu.len(x));
                println!("{} {} {}", min, max, len);
            }
        }
    }

    Ok(())
}
