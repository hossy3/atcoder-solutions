use std::collections::{BTreeSet, BTreeMap};

use proconio::input;

struct FenwickTree(Vec<usize>);

impl FenwickTree {
    pub fn add(&mut self, mut i: usize, x: usize) {
        debug_assert_ne!(i, 0);
        while i < self.0.len() {
            self.0[i] += x;
            i += i & i.wrapping_neg(); // i & -i
        }
    }

    pub fn sum(&self, mut i: usize) -> usize {
        let mut sum = 0;
        while i > 0 {
            sum += self.0[i];
            i -= i & i.wrapping_neg(); // i & -i
        }
        sum
    }

    pub fn new(n: usize) -> Self {
        FenwickTree(vec![0; n])
    }
}

fn f(a: &[Vec<usize>]) -> usize {
    let n = a.len();
    let mut tree = FenwickTree::new(n * a[0].len() + 1);
    let mut map = BTreeMap::new();
    for a in a {
        for &x in a {
            map.insert(x, 0);
        }
    }
    for (i, (_, v)) in map.iter_mut().enumerate() {
        *v = i + 1;
    }

    let mut result = 0usize;
    for (i, a) in a.iter().enumerate().rev() {
        for &x in a {
            let &i = map.get(&x).unwrap();
            result += tree.sum(i);
        }

        let mut set = BTreeSet::new();
        for &x in a {
            set.insert(x);
            let &i = map.get(&x).unwrap();
            tree.add(i, 1);
        }
        for &x in a {
            result += set.range(0..=x).count() * (n - i - 1);
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }
    let result = f(&a);
    println!("{}", result);
}
