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

fn ok(n: &[usize], p: &[usize], each: &[usize], mut money: usize, c: usize) -> bool {
    for i in 0..3 {
        let need = (c * each[i] - n[i].min(c * each[i])) * p[i];
        if money >= need {
            money -= need;
        } else {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();

        let mut each = [0; 3];
        for c in s.into_iter() {
            match c {
                b'B' => each[0] += 1,
                b'S' => each[1] += 1,
                b'C' => each[2] += 1,
                _ => unreachable!(),
            }
        }

        let mut n = [0; 3];
        for i in 0..3 {
            n[i] = cin.next::<usize>();
        }

        let mut p = [0; 3];
        for i in 0..3 {
            p[i] = cin.next::<usize>();
        }

        let money = cin.next::<usize>();

        let mut l = 0;
        let mut r = 1_000_000_000_000_000_000;

        while r - l > 1 {
            let m = (r + l) / 2;

            if ok(&n, &p, &each, money, m) {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
