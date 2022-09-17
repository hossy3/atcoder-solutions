use std::collections::BTreeMap;

use num_integer::Roots;
use proconio::input;
use proconio::marker::Usize1;

struct BTree {
    data: Vec<usize>,
    up_size: usize,
    up_data: Vec<usize>,
    up_lazy: Vec<usize>,
}

impl BTree {
    pub fn new(n: usize) -> Self {
        let up_size = (n - 1).sqrt() + 1;
        let data = vec![0; up_size * up_size];
        let up_data = vec![0; up_size];
        let up_lazy = vec![0; up_size];
        BTree {
            data,
            up_size,
            up_data,
            up_lazy,
        }
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
        _: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut m = BTreeMap::new();
    for &(l, r) in &lr {
        m.insert(l, 0);
        m.insert(r + 1, 0);
    }
    for (i, x) in m.iter_mut().enumerate() {
        *x.1 = i;
    }

    let mut t = BTree::new(m.len());
    for &(l, r) in &lr {
        let l0 = *m.get(&l).unwrap();
        let r0 = *m.get(&(r + 1)).unwrap();
        let height = t.prod(l0, r0) + 1;
        t.update(l0, r0, height);
        println!("{}", height);
    }
}
