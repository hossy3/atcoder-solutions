use ac_library::{Additive, Dsu, Segtree};
use proconio::{input, marker::Usize1};

fn f(x: usize, y: usize, v: i64, segtree: &Segtree<Additive<i64>>) -> i64 {
    if x == y {
        v
    } else if x < y {
        let prod = segtree.prod((x + 1)..=y);
        let kx = if x % 2 == 0 { 1 } else { -1 };
        let ky = if y % 2 == 0 { -1 } else { 1 };
        (prod - v * kx) * ky
    } else {
        let prod = segtree.prod((y + 1)..=x);
        let kx = if x % 2 == 0 { -1 } else { 1 };
        let ky = if y % 2 == 0 { 1 } else { -1 };
        (prod - v * kx) * ky
    }
}

fn main() {
    input! {
        n: usize,
        txyv: [(u8, Usize1, Usize1, i64)],
    }

    let mut segtree = Segtree::<Additive<i64>>::new(n);
    let mut uf = Dsu::new(n);

    for &(t, x, y, v) in &txyv {
        match t {
            0 => {
                uf.merge(x, y);
                let k = if x % 2 == 0 { 1 } else { -1 };
                segtree.set(x + 1, v * k);
            }
            1 => {
                if uf.same(x, y) {
                    let result = f(x, y, v, &segtree);
                    println!("{result}");
                } else {
                    println!("Ambiguous");
                }
            }
            _ => unreachable!(),
        }
    }
}
