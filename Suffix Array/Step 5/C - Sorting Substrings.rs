use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::usize;
use std::vec;

use cm_rmq::RMQ;

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

fn suffix_array(mut input: Vec<u8>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
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

    input.push(32);

    let n = input.len();
    let mut p = vec![0; n];
    let mut c = vec![0; n];

    let mut a = input
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect::<Vec<_>>();

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

    // lcp
    // lcp[i] = lcp(input[i..], input[j..])
    let mut lcp = vec![0; n];
    let mut k = 0;
    for i in 0..n - 1 {
        let pi = c[i];
        let j = p[pi - 1];

        while input[i + k] == input[j + k] {
            k += 1;
        }
        lcp[pi] = k;

        if k > 0 {
            k -= 1;
        }
    }

    (p, c, lcp)
}

mod cm_rmq {
    use std::{
        cmp::{max, min},
        ops::Range,
    };

    pub struct RMQ<T: Ord + Copy> {
        sparse_table: Vec<Vec<T>>,
        logs2: Vec<usize>,
        f: fn(T, T) -> T,
    }

    impl<T: Ord + Copy> RMQ<T> {
        fn new(f: fn(T, T) -> T, input: &[T]) -> RMQ<T> {
            RMQ {
                sparse_table: Self::build_sparse_table(f, input),
                logs2: vec![0]
                    .into_iter()
                    .chain((1..=input.len()).map(|x| x.ilog2() as usize))
                    .collect(),
                f: f,
            }
        }

        pub fn query(&self, range: Range<usize>) -> Option<T> {
            if range.is_empty()
                || (self.sparse_table.len() > 0 && self.sparse_table[0].len() < range.end)
            {
                return None;
            }
            let loglen = self.logs2[range.end - range.start];
            let idx: usize = range.end - (1 << loglen);
            let a = self.sparse_table[loglen][range.start];
            let b = self.sparse_table[loglen][idx];
            Some((self.f)(a, b))
        }

        fn build_sparse_table(f: fn(T, T) -> T, input: &[T]) -> Vec<Vec<T>> {
            let len = input.len();
            let mut sparse_table: Vec<Vec<T>> = vec![vec![]; len.ilog2() as usize + 1];

            for i in 0..input.len() {
                sparse_table[0].push(input[i]);
            }

            for i in 1..=len.ilog2() as usize {
                let mut j = 0;
                while j + (1 << i) <= input.len() {
                    let a = sparse_table[i - 1][j];
                    let b = sparse_table[i - 1][j + (1 << (i - 1))];
                    sparse_table[i].push(f(a, b));
                    j += 1;
                }
            }
            sparse_table
        }
    }

    impl<T: Ord + Copy> RMQ<T> {
        pub fn max(input: &[T]) -> RMQ<T> {
            RMQ::new(max, input)
        }
    }

    impl<T: Ord + Copy> RMQ<T> {
        pub fn min(input: &[T]) -> RMQ<T> {
            RMQ::new(min, input)
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();
        let (_, suf_arr_rev, lcp) = suffix_array(s);

        let rmq = RMQ::min(&lcp);

        let n = cin.next::<usize>();
        let mut a = vec![];

        for _ in 0..n {
            let (l, r) = (cin.next::<usize>(), cin.next::<usize>());
            a.push((l - 1, r - 1));
        }

        a.sort_by(|&(l0, r0), &(l1, r1)| {
            if l0 == l1 {
                return r0.cmp(&r1);
            }

            let c0 = suf_arr_rev[l0];
            let c1 = suf_arr_rev[l1];

            let c_min = c0.min(c1);
            let c_max = c0.max(c1);

            let d0 = r0 - l0 + 1;
            let d1 = r1 - l1 + 1;
            let d = d0.min(d1);

            let lcp = rmq.query((c_min + 1)..(c_max + 1)).unwrap();

            if lcp >= d {
                if d0 == d && d1 == d {
                    return l0.cmp(&l1);
                } else {
                    return d0.cmp(&d1);
                }
            } else {
                return c0.cmp(&c1);
            }
        });

        for (l, r) in a.into_iter() {
            println!("{l} {r}", l = l + 1, r = r + 1);
        }
    }
}
