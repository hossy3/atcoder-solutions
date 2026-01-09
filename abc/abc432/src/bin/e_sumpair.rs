use ac_library::{Monoid, Segtree};
use proconio::input;

pub struct SumPair;
impl Monoid for SumPair {
    type S = (usize, usize);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut v = vec![0; 500_001];
    for &x in &a {
        v[x] += 1;
    }
    let mut segtree: Segtree<SumPair> = v
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i * x))
        .collect::<Vec<_>>()
        .into();

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let z = segtree.get(y).0 + 1;
                segtree.set(y, (z, z * y));

                let y0 = a[x - 1];
                let z = segtree.get(y0).0 - 1;
                segtree.set(y0, (z, z * y0));

                a[x - 1] = y;
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let result = if l < r {
                    segtree.prod(0..l).0 * l + segtree.prod(l..r).1 + segtree.prod(r..).0 * r
                } else {
                    l * n
                };
                println!("{result}");
            }
            _ => {
                unreachable!()
            }
        }
    }
}
