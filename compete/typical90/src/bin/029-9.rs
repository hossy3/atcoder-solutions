use proconio::input;

struct FenwickTree {
    data: Vec<i64>,
}

impl FenwickTree {
    pub fn add(&mut self, mut i: usize, x: i64) {
        while i < self.data.len() {
            self.data[i] += x;
            i += i & i.wrapping_neg(); // i & -i
        }
    }

    pub fn sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i];
            i -= i & i.wrapping_neg(); // i & -i
        }
        sum
    }

    pub fn new(n: usize) -> Self {
        let data = vec![0; n];
        FenwickTree { data }
    }

    pub fn lower_bound(&self, mut x: i64) -> usize {
        if x == 0 {
            return 0;
        }
        let mut l = 0;
        let ceil_pow2 = 32 - ((self.data.len() as u32) - 1).leading_zeros();
        let mut len = 1 << ceil_pow2;
        while len > 0 {
            let i = l + len;
            if i < self.data.len() && self.data[i] < x {
                x -= self.data[i];
                l += len;
            }
            len = len >> 1;
        }
        l + 1
    }

    pub fn contains(&mut self, i: usize) -> bool {
        self.sum(i) - self.sum(i - 1) > 0
    }

    pub fn insert(&mut self, i: usize) {
        if !self.contains(i) {
            self.add(i, 1);
        }
    }

    pub fn remove(&mut self, i: usize) {
        if self.contains(i) {
            self.add(i, -1);
        }
    }
}

fn main() {
    input! {
        w: usize,
        lr: [(usize, usize)],
    }

    let mut v = vec![0usize; w + 2]; // vec<height>
    let mut t = FenwickTree::new(w + 2);
    t.insert(1);
    t.insert(w + 1);

    for &(l, r) in &lr {
        let hl = v[t.lower_bound(t.sum(l))];
        let hr = v[t.lower_bound(t.sum(r + 1))];

        let mut height = hl + 1;
        loop {
            let i = t.lower_bound(t.sum(r));
            if i <= l {
                break;
            }
            height = height.max(v[i] + 1);
            t.remove(i);
        }

        v[l] = height;
        t.insert(l);
        v[r + 1] = hr;
        t.insert(r + 1);

        println!("{}", height);
    }
}
