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

fn count_sort(p: &mut Vec<usize>, c: &Vec<usize>) {
    let n = p.len();

    let mut cnt = vec![0; n];
    for &x in c.iter() {
        cnt[x] += 1;
    }

    let mut pos = vec![0; n];
    for i in 1..n {
        pos[i] = pos[i - 1] + cnt[i - 1];
    }

    let mut p_sorted = vec![usize::default(); n];
    for &el in p.iter() {
        p_sorted[pos[c[el]]] = el;
        pos[c[el]] += 1;
    }

    *p = p_sorted;
}

fn suffix_array(input: &[u8]) -> Vec<usize> {
    let n = input.len() + 1;
    let mut p = vec![0; n];
    let mut c = vec![0; n];

    let mut a = input
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect::<Vec<_>>();

    a.push((b'$', input.len()));

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
            p[i] = (n + p[i] - (1 << k)) % n;
        }

        count_sort(&mut p, &c);

        let mut c_new = vec![0; n];
        c_new[p[0]] = 0;
        for i in 1..n {
            let prev = (c[p[i - 1]], c[(p[i - 1] + (1 << k)) % n]);
            let now = (c[p[i]], c[(p[i] + (1 << k)) % n]);
            if prev == now {
                c_new[p[i]] = c_new[p[i - 1]];
            } else {
                c_new[p[i]] = c_new[p[i - 1]] + 1;
            }
        }
        c = c_new;
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
