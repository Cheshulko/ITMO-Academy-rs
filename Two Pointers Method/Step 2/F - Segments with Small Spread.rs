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

    struct Stack<T> {
        stack: Vec<T>,
        min_stack: Vec<T>,
        max_stack: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Self {
                stack: vec![],
                min_stack: vec![],
                max_stack: vec![],
            }
        }
    }

    impl<T: Ord + Copy + Debug> Stack<T> {
        fn push(&mut self, element: T) {
            self.stack.push(element);

            if let Some(&mi) = self.min_stack.last() {
                self.min_stack.push(mi.min(element));
            } else {
                self.min_stack.push(element);
            }

            if let Some(&ma) = self.max_stack.last() {
                self.max_stack.push(ma.max(element));
            } else {
                self.max_stack.push(element);
            }
        }

        fn pop(&mut self) -> Option<T> {
            let _ = self.min_stack.pop();
            let _ = self.max_stack.pop();

            self.stack.pop()
        }

        fn is_empty(&self) -> bool {
            self.stack.is_empty()
        }

        fn min(&self) -> Option<&T> {
            self.min_stack.last()
        }

        fn max(&self) -> Option<&T> {
            self.max_stack.last()
        }
    }

    pub struct QueueMinMax<T> {
        stack_left: Stack<T>,
        stack_right: Stack<T>,
    }

    impl<T> QueueMinMax<T> {
        pub fn new() -> Self {
            Self {
                stack_left: Stack::new(),
                stack_right: Stack::new(),
            }
        }
    }

    impl<T: Ord + Copy + Debug> QueueMinMax<T> {
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

        pub fn max(&self) -> Option<&T> {
            self.stack_left.max().max(self.stack_right.max())
        }

        pub fn min(&self) -> Option<&T> {
            // None.min(Some(10)) == None
            // self.stack_left.min().min(self.stack_right.min())

            match (self.stack_left.min(), self.stack_right.min()) {
                (None, Some(mi)) => Some(mi),
                (Some(mi), None) => Some(mi),
                (Some(mi1), Some(mi2)) => Some(mi1.min(mi2)),
                _ => None,
            }
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (N, K) = (cin.next::<usize>(), cin.next::<usize>());

        let mut A = vec![0; N];
        for i in 0..N {
            A[i] = cin.next::<usize>();
        }

        let mut ans = 0;
        let mut l = 0;
        let mut q = cm::QueueMinMax::new();

        for r in 0..N {
            q.push_back(A[r]);

            while let (Some(&mi), Some(&ma)) = (q.min(), q.max()) {
                if ma - mi > K {
                    q.pop_front();
                    l += 1;
                } else {
                    break;
                }
            }

            ans += 1 + r - l;
        }

        println!("{ans}");
    }
}
