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

mod cm {
    use std::fmt::Debug;

    struct Stack<T, F> {
        stack: Vec<T>,
        func_stack: Vec<T>,
        func: F,
    }

    impl<T, F> Stack<T, F> {
        fn new(func: F) -> Self {
            Self {
                stack: vec![],
                func_stack: vec![],
                func,
            }
        }
    }

    impl<T: Copy + Debug, F: Fn(T, T) -> T> Stack<T, F> {
        fn push(&mut self, element: T) {
            self.stack.push(element);

            if let Some(&back_element) = self.func_stack.last() {
                self.func_stack.push((self.func)(back_element, element));
            } else {
                self.func_stack.push(element);
            }
        }

        fn pop(&mut self) -> Option<T> {
            let _ = self.func_stack.pop();

            self.stack.pop()
        }

        fn is_empty(&self) -> bool {
            self.stack.is_empty()
        }

        fn func_last(&self) -> Option<&T> {
            self.func_stack.last()
        }
    }

    pub struct QueueFunc<T, F> {
        stack_left: Stack<T, F>,
        stack_right: Stack<T, F>,
        func: F,
    }

    impl<T, F: Clone> QueueFunc<T, F> {
        pub fn new(func: F) -> Self {
            Self {
                stack_left: Stack::new(func.clone()),
                stack_right: Stack::new(func.clone()),
                func,
            }
        }
    }

    impl<T: Copy + Debug, F: Fn(T, T) -> T> QueueFunc<T, F> {
        pub fn push_back(&mut self, element: T) {
            self.stack_right.push(element);
        }

        pub fn pop_front(&mut self) -> Option<T> {
            if self.stack_left.is_empty() {
                while let Some(element) = self.stack_right.pop() {
                    self.stack_left.push(element);
                }
            }

            self.stack_left.pop()
        }

        pub fn func_last(&self) -> Option<T> {
            match (self.stack_left.func_last(), self.stack_right.func_last()) {
                (None, Some(&mi)) => Some(mi),
                (Some(&mi), None) => Some(mi),
                (Some(&mi1), Some(&mi2)) => Some((self.func)(mi1, mi2)),
                _ => None,
            }
        }
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    while a != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    b
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (N,) = (cin.next::<usize>(),);

        let mut A = vec![0; N];
        for i in 0..N {
            A[i] = cin.next::<usize>();
        }

        let mut ans = usize::MAX;
        let mut l = 0;
        let mut q = cm::QueueFunc::new(|a: usize, b: usize| -> usize { gcd(a, b) });

        for r in 0..N {
            q.push_back(A[r]);

            while let Some(v1) = q.func_last() {
                if v1 == 1 {
                    ans = ans.min(1 + r - l);

                    q.pop_front();
                    l += 1;
                } else {
                    break;
                }
            }

            if let Some(v) = q.func_last() {
                if v == 1 {
                    ans = ans.min(1 + r - l);
                }
            }
        }

        if ans == usize::MAX {
            println!("-1");
        } else {
            println!("{ans}");
        }
    }
}
