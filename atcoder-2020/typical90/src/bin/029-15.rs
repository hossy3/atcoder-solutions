use proconio::input;
use proconio::marker::Usize1;

struct LazySegTree {
    size: usize,
    data: Vec<usize>,
    lazy: Vec<usize>,
}

impl LazySegTree {
    fn update_impl(
        &mut self,
        lr: (usize, usize),
        x: usize,
        k: usize,
        lr0: (usize, usize),
        lazy: usize,
    ) {
        let (l, r) = lr;
        let (l0, r0) = lr0;
        if r0 <= l || r <= l0 {
            // noop
        } else if l <= l0 && r0 <= r {
            self.data[k] = self.data[k].max(x);
            if k < self.size {
                self.lazy[k] = self.lazy[k].max(x);
            }
        } else {
            let lazy = lazy.max(self.lazy[k]);
            self.update_impl(lr, x, k * 2, (l0, (l0 + r0) / 2), lazy);
            self.update_impl(lr, x, k * 2 + 1, ((l0 + r0) / 2, r0), lazy);
            self.data[k] = self.data[k * 2].max(self.data[k * 2 + 1]);
        }
    }

    fn prod_impl(
        &mut self,
        lr: (usize, usize),
        k: usize,
        lr0: (usize, usize),
        lazy: usize,
    ) -> usize {
        let (l, r) = lr;
        let (l0, r0) = lr0;
        if r0 <= l || r <= l0 {
            0
        } else if l <= l0 && r0 <= r {
            lazy.max(self.data[k])
        } else {
            let lazy = lazy.max(self.lazy[k]);
            let l_val = self.prod_impl(lr, k * 2, (l0, (l0 + r0) / 2), lazy);
            let r_val = self.prod_impl(lr, k * 2 + 1, ((l0 + r0) / 2, r0), lazy);
            l_val.max(r_val)
        }
    }

    pub fn new(n: usize) -> Self {
        let ceil_pow2 = 32 - ((n as u32) - 1).leading_zeros();
        let size = 1 << ceil_pow2;
        let data = vec![0; size * 2];
        let lazy = vec![0; size];
        LazySegTree { size, data, lazy }
    }

    pub fn update(&mut self, l: usize, r: usize, x: usize) {
        self.update_impl((l, r), x, 1, (0, self.size), 0);
    }

    pub fn prod(&mut self, l: usize, r: usize) -> usize {
        self.prod_impl((l, r), 1, (0, self.size), 0)
    }
}

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut seg = LazySegTree::new(w);
    for &(l, r) in &lr {
        let height = seg.prod(l, r + 1) + 1;
        seg.update(l, r + 1, height);
        println!("{}", height);
    }
}
