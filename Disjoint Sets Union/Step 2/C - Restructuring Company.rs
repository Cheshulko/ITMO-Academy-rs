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

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut dsu = cm::DSU::new(n);
        let mut bs = (0..n).map(|x| (x, x, x)).collect::<BTreeSet<_>>();
        for _ in 0..q {
            let (t, mut x, mut y) = (
                cin.next::<usize>(),
                cin.next::<usize>() - 1,
                cin.next::<usize>() - 1,
            );

            match t {
                1 => {
                    let mut xc_range =
                        *bs.range(..(x, usize::MAX, usize::MAX)).next_back().unwrap();
                    let mut yc_range =
                        *bs.range(..(y, usize::MAX, usize::MAX)).next_back().unwrap();

                    // println!("x = {x} xc_range = {:?}", xc_range);
                    // println!("y = {y} xc_range = {:?}", yc_range);

                    bs.remove(&xc_range);
                    bs.remove(&yc_range);

                    dsu.union(xc_range.2, yc_range.2);
                    let x = dsu.find(xc_range.2);

                    xc_range.2 = x;
                    yc_range.2 = x;

                    bs.insert(xc_range);
                    bs.insert(yc_range);
                }
                2 => {
                    if x > y {
                        swap(&mut x, &mut y);
                    }

                    let mut ranges = vec![];
                    let mut st = bs.range(..(y, usize::MAX, usize::MAX));

                    while let Some(&range) = st.next_back() {
                        if range.1 < x {
                            break;
                        }
                        ranges.push(range);
                    }
                    assert!(!ranges.is_empty());

                    ranges.reverse();

                    let x = dsu.find(ranges[0].2);
                    let start = ranges[0].0;
                    let end = ranges[ranges.len() - 1].1;
                    for range in ranges.into_iter() {
                        bs.remove(&range);
                        dsu.union(x, range.2);
                    }
                    bs.insert((start, end, dsu.find(x)));
                }
                3 => {
                    if dsu.same(x, y) {
                        println!("YES");
                    } else {
                        println!("NO");
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
