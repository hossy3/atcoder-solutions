use proconio::input;

// Fenwick Tree (Binary Indexed Tree)

struct FenwickTree(Vec<i64>);

impl FenwickTree {
    pub fn add(&mut self, mut i: usize, x: i64) {
        while i < self.0.len() {
            self.0[i] += x;
            i += i & i.wrapping_neg(); // i & -i
        }
    }

    pub fn sum(&self, mut i: usize) -> i64 {
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
        a: [usize; n],
    }
    let mut ft = FenwickTree::new(n + 1); // don't use ft[0]
    let mut result = 0;
    for (i, &a) in a.iter().enumerate() {
        result += i as i64 - ft.sum(a);
        ft.add(a, 1);
    }
    println!("{}", result);
}
