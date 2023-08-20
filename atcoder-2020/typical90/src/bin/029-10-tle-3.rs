use num_integer::Roots;
use proconio::input;
use proconio::marker::Usize1;

struct Btree {
    data: Vec<usize>,
    up_size: usize,
    up_data: Vec<usize>,
    up_lazy: Vec<usize>,
}

impl Btree {
    pub fn new(n: usize) -> Self {
        let up_size = (n - 1).sqrt() + 1;
        let data = vec![0; up_size * up_size];
        let up_data = vec![0; up_size];
        let up_lazy = vec![0; up_size];
        Btree { data, up_size, up_data, up_lazy }
    }

    pub fn update(&mut self, l: usize, r: usize, x: usize) {
        let n = self.up_size;
        let mut i = l;
        while i < r {
            let j = i / n;
            self.up_data[j] = self.up_data[j].max(x);
            if i % n == 0 && i + n <= r {
                self.up_lazy[j] = self.up_lazy[j].max(x);
                i += n;
            } else {
                self.data[i] = self.up_lazy[j].max(x);
                i += 1;
            }
        }
    }

    pub fn prod(&mut self, l: usize, r: usize) -> usize {
        let n = self.up_size;
        let mut x = 0;
        let mut i = l;
        while i < r {
            let j = i / n;
            if i % n == 0 && i + n <= r {
                x = x.max(self.up_data[j]);
                i += n;
            } else {
                x = x.max(self.up_lazy[j].max(self.data[i]));
                i += 1;
            }
        }
        x
    }
}

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut t = Btree::new(w);
    for &(l, r) in &lr {
        let height = t.prod(l, r + 1) + 1;
        t.update(l, r + 1, height);
        println!("{}", height);
    }
}
