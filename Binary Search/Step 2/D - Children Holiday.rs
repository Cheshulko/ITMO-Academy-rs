use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

fn solve(a: &Vec<(usize, usize, usize)>, lim: usize) -> (Vec<usize>, usize) {
    let mut cnt = 0;
    let mut each = vec![0; a.len()];

    for (i, &(t, z, y)) in a.iter().enumerate() {
        let g = t * z + y;
        let c1 = lim / g;
        let rem = lim % g;
        let c2 = (rem / t).min(z);
        cnt += c1 * z + c2;
        each[i] = c1 * z + c2;
    }

    (each, cnt)
}

fn ok(m: usize, a: &Vec<(usize, usize, usize)>, lim: usize) -> bool {
    let (_, cnt) = solve(a, lim);

    m <= cnt
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (m, n) = (cin.next::<usize>(), cin.next::<usize>());

        if m == 0 {
            println!("0");
            println!("0");
            continue;
        }

        let mut a = vec![];
        for _ in 0..n {
            let x = (
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
            );

            a.push(x);
        }

        let mut l = 0;
        let mut r = 1_000_000_000_000;

        while r - l > 1 {
            let lim = (r + l) / 2;

            if ok(m, &a, lim) {
                r = lim;
            } else {
                l = lim;
            }
        }

        let (mut ans, _) = solve(&a, r);

        let mut need = m;
        for x in ans.iter_mut() {
            *x = (*x).min(need);
            need -= *x;
        }

        assert!(ans.iter().sum::<usize>() == m);

        println!("{r}");
        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
