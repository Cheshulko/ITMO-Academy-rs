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

    pub fn get_list(&self, x: usize) -> &Vec<usize> {
        let x = self.get(x);

        &self.list[x]
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

fn dfs(cur: usize, con: &Vec<[i32; 2]>, used: &mut Vec<bool>, dsu: &mut Dsu) {
    used[cur] = true;

    for i in 0..2 {
        let to = con[cur][i];
        if to != -1 {
            dsu.union(cur, to as usize);
            if !used[to as usize] {
                dfs(to as usize, con, used, dsu);
            }
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

        let mut con = vec![[-1, -1]; n];
        for i in 0..n {
            let (l, r) = (cin.next::<i32>(), cin.next::<i32>());
            let l = (-1).max(l - 1);
            let r = (-1).max(r - 1);
            con[i] = [l, r];
        }

        let mut ord = vec![];
        for _ in 0..m {
            let (x, t) = (cin.next::<usize>(), cin.next::<i32>());
            if t == 1 {
                ord.push((x - 1, 0, con[x - 1][0] as usize));
            } else {
                ord.push((x - 1, 1, con[x - 1][1] as usize));
            }
        }

        for &(i, h, _) in ord.iter() {
            con[i][h] = -1;
        }

        let mut dsu = Dsu::new(n);
        let mut used = vec![false; n];
        for i in 0..n {
            if !used[i] {
                dfs(i, &con, &mut used, &mut dsu);
            }
        }

        let mut times = vec![42; n];
        {
            for &i in dsu.get_list(0) {
                times[i] = -1;
            }
        }

        for (tick, (i, _, to)) in ord.into_iter().rev().enumerate() {
            let main = dsu.get(0);
            let ii = dsu.get(i);
            let tto = dsu.get(to);
            if ii != tto {
                if main == ii {
                    for &i in dsu.get_list(to) {
                        times[i] = (m - tick - 1) as i32;
                    }
                }
                if main == tto {
                    for &i in dsu.get_list(i) {
                        times[i] = (m - tick - 1) as i32;
                    }
                }
                dsu.union(i, to);
            }
        }

        for x in times.into_iter() {
            println!("{}", x);
        }
    }

    Ok(())
}
