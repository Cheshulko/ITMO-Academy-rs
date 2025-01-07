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

fn ok(d: usize, adj: &Vec<Vec<(usize, i64)>>, lim: i64) -> (bool, Vec<i32>) {
    let n = adj.len();
    let mut q = VecDeque::<(usize, usize)>::new();
    let mut used = vec![false; n];
    used[0] = true;
    q.push_back((0, 0));

    let mut par = vec![-1; n];
    while let Some((cur, dist)) = q.pop_front() {
        if cur == n - 1 {
            return (dist <= d, par);
        }

        for &(to, c) in adj[cur].iter() {
            if c <= lim && !used[to] {
                used[to] = true;
                par[to] = cur as i32;
                q.push_back((to, dist + 1));
            }
        }
    }

    return (false, par);
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m, d) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut max_c = 0;
        let mut adj = vec![vec![]; n];
        for _ in 0..m {
            let (a, b, c) = (cin.next::<usize>(), cin.next::<usize>(), cin.next::<i64>());
            adj[a - 1].push((b - 1, c));
            max_c = max_c.max(c);
        }

        let (can, _) = ok(d, &adj, max_c);
        if !can {
            println!("-1");
            continue;
        }

        let mut l = -1;
        let mut r = max_c;

        while r - l > 1 {
            let m = (r + l) / 2;

            let (can, _) = ok(d, &adj, m);
            if can {
                r = m;
            } else {
                l = m;
            }
        }

        let (_, par) = ok(d, &adj, r);
        let mut ans = vec![];
        let mut cur = n as i32 - 1;

        while cur != -1 {
            ans.push(cur + 1);
            cur = par[cur as usize];
        }

        println!("{l}", l = ans.len() - 1);
        for x in ans.into_iter().rev() {
            print!("{x} ");
        }
        println!();
    }
}
