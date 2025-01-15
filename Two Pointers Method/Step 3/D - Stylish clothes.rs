#![allow(non_snake_case)]
#![allow(unused_imports)]

use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
use std::mem::swap;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &std::path::Path) -> Self {
        use std::fs::File;

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

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        const S: usize = 4;

        let mut N = [0; S];
        let mut A = vec![vec![]; S];

        for i in 0..S {
            N[i] = cin.next::<usize>();
            A[i].resize(N[i], 0);

            for j in 0..N[i] {
                A[i][j] = cin.next::<usize>();
            }
            A[i].sort_unstable();
        }

        let calc = |A: &Vec<Vec<usize>>, I: &[usize; S]| -> usize {
            let mut tmp = A
                .iter()
                .enumerate()
                .map(|(i, a)| a[I[i]])
                .collect::<Vec<_>>();
            tmp.sort_unstable();

            tmp[S - 1] - tmp[0]
        };

        let mut I = [0; S];

        let mut ans_i = [0; S];
        let mut ans = calc(&A, &I);

        loop {
            let mut next_best = usize::MAX;
            let mut next_best_i = 0;
            for i in 0..S {
                if I[i] + 1 < N[i] {
                    if A[i][I[i] + 1] < next_best {
                        next_best = A[i][I[i] + 1];
                        next_best_i = i;
                    }
                }
            }

            if next_best != usize::MAX {
                I[next_best_i] += 1;

                let maybe_ans = calc(&A, &I);
                if maybe_ans < ans {
                    ans = maybe_ans;
                    for i in 0..S {
                        ans_i[i] = I[i];
                    }
                }
            } else {
                break;
            }
        }

        for i in 0..S {
            print!("{} ", A[i][ans_i[i]]);
        }
        println!();
    }
}
