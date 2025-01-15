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
    use std::{collections::HashSet, fmt::Debug};

    struct Stack<T, F, E> {
        stack: Vec<E>,
        func_stack: Vec<T>,
        func: F,
    }

    impl<T, F, E> Stack<T, F, E> {
        fn new(func: F) -> Self {
            Self {
                stack: vec![],
                func_stack: vec![],
                func,
            }
        }
    }

    impl<T: Clone + Debug, F: Fn(Option<&T>, E) -> T, E: Copy> Stack<T, F, E> {
        fn push(&mut self, element: E) {
            self.stack.push(element);

            self.func_stack
                .push((self.func)(self.func_stack.last(), element));
        }

        fn pop(&mut self) -> Option<E> {
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

    pub struct QueueFunc<T, F, E> {
        stack_left: Stack<T, F, E>,
        stack_right: Stack<T, F, E>,
    }

    impl<T, F: Clone, E: Copy> QueueFunc<T, F, E> {
        pub fn new(func: F) -> Self {
            Self {
                stack_left: Stack::new(func.clone()),
                stack_right: Stack::new(func.clone()),
            }
        }
    }

    impl<T: Clone + Debug, F: Fn(Option<&T>, E) -> T, E: Copy> QueueFunc<T, F, E> {
        pub fn push_back(&mut self, element: E) {
            self.stack_right.push(element);
        }

        pub fn pop_front(&mut self) -> Option<E> {
            if self.stack_left.is_empty() {
                while let Some(element) = self.stack_right.pop() {
                    self.stack_left.push(element);
                }
            }

            self.stack_left.pop()
        }

        pub fn func_last<C: Fn(Option<&T>, Option<&T>) -> bool>(&self, check: C) -> bool {
            check(self.stack_left.func_last(), self.stack_right.func_last())
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (N, S) = (cin.next::<usize>(), cin.next::<usize>());

        let mut A = vec![0; N];
        for i in 0..N {
            A[i] = cin.next::<usize>();
        }

        let mut q = cm::QueueFunc::new(|a: Option<&Vec<bool>>, b: usize| -> Vec<bool> {
            if let Some(a) = a {
                let mut c = a.clone();
                c[b] = true;

                for x in 0..S + 1 {
                    if a[x] && x + b <= S {
                        c[x + b] = true;
                    }
                }

                c
            } else {
                let mut c = vec![false; 1000 + 1];
                c[b] = true;

                c
            }
        });

        let mut l = 0;
        let mut ans = usize::MAX;
        for r in 0..N {
            q.push_back(A[r]);

            while q.func_last(|a: Option<&Vec<bool>>, b: Option<&Vec<bool>>| -> bool {
                match (a, b) {
                    (None, Some(mi)) => mi[S],
                    (Some(mi), None) => mi[S],
                    (Some(mi1), Some(mi2)) => {
                        if mi1[S] {
                            return true;
                        }
                        if mi2[S] {
                            return true;
                        }
                        for x in 0..S + 1 {
                            if mi1[x] && mi2[S - x] {
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => false,
                }
            }) {
                ans = ans.min(1 + r - l);
                q.pop_front();
                l += 1;
            }
        }

        if ans == usize::MAX {
            println!("-1");
        } else {
            println!("{ans}");
        }
    }
}
