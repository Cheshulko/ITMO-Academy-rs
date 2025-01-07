use std::cmp;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
use std::usize;
use std::vec;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn suffix_array(input: &[u8]) -> Vec<usize> {
    let n = input.len() + 1;
    let mut p = vec![0; n];
    let mut c = vec![0; n];

    let mut a = input
        .iter()
        .enumerate()
        .map(|(i, x)| ((*x as usize, 0), i))
        .collect::<Vec<_>>();

    a.push(((b'$' as usize, 0), input.len()));

    {
        // k = 0
        a.sort_unstable();
        for i in 0..n {
            p[i] = a[i].1;
        }
        c[p[0]] = 0;
        for i in 1..n {
            if a[i - 1].0 == a[i].0 {
                c[p[i]] = c[p[i - 1]];
            } else {
                c[p[i]] = c[p[i - 1]] + 1;
            }
        }
    }

    let mut k = 0;
    while (1 << k) < n {
        // k -> k + 1

        for i in 0..n {
            a[i] = ((c[i], c[(i + (1 << k)) % n]), i);
        }

        a.sort_unstable();

        for i in 0..n {
            p[i] = a[i].1;
        }
        c[p[0]] = 0;
        for i in 1..n {
            if a[i - 1].0 == a[i].0 {
                c[p[i]] = c[p[i - 1]];
            } else {
                c[p[i]] = c[p[i - 1]] + 1;
            }
        }
        k += 1;
    }

    p
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();
        let s = suffix_array(&s);

        for x in s.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
