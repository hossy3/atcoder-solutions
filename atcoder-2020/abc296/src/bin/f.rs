use std::collections::BTreeMap;

use proconio::input;

struct FenwickTree(Vec<usize>);

impl FenwickTree {
    pub fn add(&mut self, mut i: usize, x: usize) {
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

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    let a0 = a.clone();
    let b0 = b.clone();
    a.sort();
    b.sort();

    if a != b {
        println!("{}", "No");
        return;
    }
    if a.windows(2).any(|x| x[0] == x[1]) {
        println!("{}", "Yes");
        return;
    }

    let mut map = BTreeMap::new();
    for (i, &x) in a.iter().enumerate() {
        map.insert(x, i);
    }
    let mut a_sum = 0;
    {
        let mut tree = FenwickTree::new(n + 1);
        for (i, &x) in a0.iter().enumerate() {
            let x = map.get(&x).unwrap() + 1;
            a_sum += i - tree.sum(x);
            tree.add(x + 1, 1);
        }
    }
    let mut b_sum = 0;
    {
        let mut tree = FenwickTree::new(n + 1);
        for (i, &x) in b0.iter().enumerate() {
            let x = map.get(&x).unwrap() + 1;
            b_sum += i - tree.sum(x);
            tree.add(x + 1, 1);
        }
    }

    let yes = a_sum % 2 == b_sum % 2;
    println!("{}", if yes { "Yes" } else { "No" });
}
