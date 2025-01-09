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
    list: Vec<Vec<usize>>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            list: (0..=n).map(|x| vec![x]).collect(),
        }
    }

    pub fn get(&self, x: usize) -> usize {
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let mut x = self.get(x);
        let mut y = self.get(y);
        if x == y {
            return;
        }

        if self.list[x].len() > self.list[y].len() {
            swap(&mut x, &mut y);
        }

        let mut tmp = vec![];
        for &xx in self.list[x].iter() {
            self.parent[xx] = y;
            tmp.push(xx);
        }

        self.list[y].extend(tmp.into_iter());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );
        for _ in 0..m {
            let (_, _) = (cin.next::<usize>(), cin.next::<usize>());
        }

        let mut dsu = Dsu::new(n + 1);

        let mut ops = vec![];
        let mut res = vec![];
        for _ in 0..k {
            let t = cin.next::<String>();
            if &t == "cut" {
                let (u, v) = (cin.next::<usize>(), cin.next::<usize>());
                ops.push((0, u, v));
            } else {
                let (u, v) = (cin.next::<usize>(), cin.next::<usize>());
                ops.push((1, u, v));
            }
        }
        for (op, u, v) in ops.into_iter().rev() {
            if op == 0 {
                dsu.union(u, v);
            } else {
                res.push(dsu.get(u) == dsu.get(v));
            }
        }
        for r in res.into_iter().rev() {
            if r {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }

    Ok(())
}
