use proconio::{input, marker::Usize1};

struct FenwickTree(Vec<i64>);
struct FenwickTreeSet(FenwickTree);

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

    fn lower_bound(&self, mut x: i64) -> usize {
        if x == 0 {
            return 0;
        }
        let data = &self.0;
        let mut l = 0;
        let ceil_pow2 = 32 - ((data.len() as u32) - 1).leading_zeros();
        let mut len = 1 << ceil_pow2;
        while len > 0 {
            let i = l + len;
            if i < data.len() && data[i] < x {
                x -= data[i];
                l += len;
            }
            len = len >> 1;
        }
        l
    }

    pub fn new(n: usize) -> Self {
        FenwickTree(vec![0; n])
    }
}

impl FenwickTreeSet {
    pub fn new(n: usize) -> Self {
        FenwickTreeSet(FenwickTree::new(n + 1))
    }

    pub fn lower_bound(&mut self, i: usize) -> usize {
        self.0.lower_bound(self.0.sum(i + 1))
    }

    pub fn contains(&mut self, i: usize) -> bool {
        self.0.sum(i + 1) - self.0.sum(i) > 0
    }

    pub fn insert(&mut self, i: usize) {
        if !self.contains(i) {
            self.0.add(i + 1, 1);
        }
    }

    pub fn remove(&mut self, i: usize) {
        if self.contains(i) {
            self.0.add(i + 1, -1);
        }
    }
}

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut v = vec![0usize; w + 1];
    let mut s = FenwickTreeSet::new(w + 1);
    s.insert(0);
    s.insert(w);

    for &(l, r) in &lr {
        let hl = v[s.lower_bound(l)];
        let hr = v[s.lower_bound(r + 1)];

        let mut height = hl + 1;
        loop {
            let i = s.lower_bound(r);
            if i <= l {
                break;
            }
            height = height.max(v[i] + 1);
            s.remove(i);
        }

        v[l] = height;
        s.insert(l);
        v[r + 1] = hr;
        s.insert(r + 1);

        println!("{}", height);
    }
}
