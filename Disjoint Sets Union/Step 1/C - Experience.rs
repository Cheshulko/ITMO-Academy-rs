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
    times: Vec<usize>,
    list: Vec<(Vec<usize>, Vec<usize>, usize)>,
    own: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            times: vec![0; n],
            list: (0..n).map(|x| (vec![x], vec![0], 0)).collect(),
            own: vec![0; n],
        }
    }

    pub fn find(&self, x: usize) -> usize {
        self.parent[x]
    }

    pub fn get_value(&self, x: usize) -> usize {
        let own = self.own[x];
        let time = self.times[x];

        let x = self.find(x);
        let mut sum = self.list[x].2;
        for i in 0..time {
            sum -= self.list[x].1[i];
        }
        own + sum
    }

    pub fn add(&mut self, x: usize, v: usize) {
        let x = self.find(x);

        let l = self.list[x].1.len();
        self.list[x].1[l - 1] += v;
        self.list[x].2 += v;
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
        let mut small_sum = self.list[x].2;
        let mut cur_time = 0;
        for &xx in self.list[x].0.iter() {
            if self.times[xx] == cur_time {
            } else {
                assert!(self.times[xx] == cur_time + 1);
                small_sum -= self.list[x].1[cur_time];
                cur_time += 1;
            }
            self.own[xx] += small_sum;
            self.parent[xx] = y;
            tmp.push(xx);
        }

        let time = self.list[y].1.len();
        self.list[y].1.push(0);
        for xx in tmp.into_iter() {
            self.times[xx] = time;
            self.list[y].0.push(xx);
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
        let mut dsu = Dsu::new(n + 1);

        for _ in 0..m {
            let t = cin.next::<String>();
            if &t == "join" {
                let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
                dsu.union(x, y);
            } else if &t == "add" {
                let (x, v) = (cin.next::<usize>(), cin.next::<usize>());
                dsu.add(x, v);
            } else {
                let x = cin.next::<usize>();
                println!("{}", dsu.get_value(x));
            }
        }
    }

    Ok(())
}
