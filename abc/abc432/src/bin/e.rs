use ac_library::{Additive, Segtree};
use proconio::input;

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
    let mut seg0: Segtree<Additive<_>> = v.clone().into();
    let mut seg1: Segtree<Additive<_>> = v
        .iter()
        .enumerate()
        .map(|(i, &x)| i * x)
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
                seg0.set(y, seg0.get(y) + 1);
                seg1.set(y, seg1.get(y) + y);

                let y0 = a[x - 1];
                seg0.set(y0, seg0.get(y0) - 1);
                seg1.set(y0, seg1.get(y0) - y0);

                a[x - 1] = y;
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let result = if l < r {
                    seg0.prod(0..l) * l + seg1.prod(l..r) + seg0.prod(r..) * r
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
