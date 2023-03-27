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

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut tree = FenwickTree::new(n + 1);
    for (i, &x) in a.iter().enumerate() {
        tree.add(i + 1, x);
    }
    for &(l, r) in &lr {
        let result = tree.sum(r) - tree.sum(l - 1);
        println!("{}", result);
    }
}
