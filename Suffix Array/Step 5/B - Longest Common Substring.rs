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

fn suffix_array(mut input: Vec<u8>) -> (Vec<usize>, Vec<usize>) {
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

    input.push(b'$');

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

    (p, lcp)
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let s = cin.next::<String>();
        let t = cin.next::<String>();

        let end_s = s.len() - 1;
        let start_t = s.len() + 1;

        let st_str = format!("{s}%{t}");
        let st = st_str.clone().into_bytes();

        let (suffix_arr, lcp) = suffix_array(st);
        let n = suffix_arr.len();

        let mut ans = 0;
        let mut pos = 0;
        for i in 2..n {
            if (suffix_arr[i - 1] <= end_s && suffix_arr[i] >= start_t)
                || (suffix_arr[i] <= end_s && suffix_arr[i - 1] >= start_t)
            {
                if lcp[i] > ans {
                    ans = lcp[i];
                    pos = suffix_arr[i];
                }
            }
        }
        if ans == 0 {
            println!("");
        } else {
            println!("{ans}", ans = &st_str[pos..(pos + ans)]);
        }
    }
}
