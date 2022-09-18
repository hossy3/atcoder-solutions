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
        Btree {
            data,
            up_size,
            up_data,
            up_lazy,
        }
    }

    pub fn update(&mut self, l: usize, r: usize, x: usize) {
        let n = self.up_size;
        let mut l0 = l / n;
        let r0 = r / n;
        if l0 == r0 {
            self.up_data[l0] = self.up_data[l0].max(x);
            for i in l..r {
                self.data[i] = x;
            }
        } else {
            if l % n > 0 {
                self.up_data[l0] = self.up_data[l0].max(x);
                for i in l..((l0 + 1) * n) {
                    self.data[i] = x;
                }
                l0 += 1;
            }
            if l0 < r0 {
                for i in l0..r0 {
                    self.up_data[i] = x;
                    self.up_lazy[i] = x;
                }
            }
            if r % n > 0 {
                self.up_data[r0] = self.up_data[r0].max(x);
                for i in (r0 * n)..r {
                    self.data[i] = x;
                }
            }
        }
    }

    pub fn prod(&mut self, l: usize, r: usize) -> usize {
        let n = self.up_size;
        let mut l0 = l / n;
        let r0 = r / n;
        let mut x = 0;
        if l0 == r0 {
            x = x.max(self.up_lazy[l0]);
            x = x.max(*self.data[l..r].iter().max().unwrap());
        } else {
            if l % n > 0 {
                x = x.max(self.up_lazy[l0]);
                x = x.max(*self.data[l..((l0 + 1) * n)].iter().max().unwrap());
                l0 += 1;
            }
            if l0 < r0 {
                x = x.max(*self.up_data[l0..r0].iter().max().unwrap());
            }
            if r % n > 0 {
                x = x.max(self.up_lazy[r0]);
                x = x.max(*self.data[(r0 * n)..r].iter().max().unwrap());
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
