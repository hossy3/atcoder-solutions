use ac_library::{Monoid, Segtree};
use proconio::input;

pub struct Max2nd;
impl Monoid for Max2nd {
    type S = ((usize, usize), (usize, usize));
    fn identity() -> Self::S {
        ((0, 0), (0, 0))
    }
    fn binary_operation(
        &((a0, ca0), (a1, ca1)): &Self::S,
        &((b0, cb0), (b1, cb1)): &Self::S,
    ) -> Self::S {
        if a0 == b0 {
            if a1 == b1 {
                ((a0, ca0 + cb0), (a1, ca1 + cb1))
            } else if a1 > b1 {
                ((a0, ca0 + cb0), (a1, ca1))
            } else {
                ((a0, ca0 + cb0), (b1, cb1))
            }
        } else if a0 > b0 {
            if a1 == b0 {
                ((a0, ca0), (a1, ca1 + cb0))
            } else if a1 > b0 {
                ((a0, ca0), (a1, ca1))
            } else {
                ((a0, ca0), (b0, cb0))
            }
        } else {
            if a0 == b1 {
                ((b0, cb0), (a0, ca0 + cb1))
            } else if a0 > b1 {
                ((b0, cb0), (a0, ca0))
            } else {
                ((b0, cb0), (b1, cb1))
            }
        }
    }
}

fn main() {
    input! {
        (n, q): (usize, usize),
        mut a: [usize; n],
        query: [(u8, usize, usize); q],
    }

    let mut segtree = Segtree::<Max2nd>::new(n);
    for (i, &x) in a.iter().enumerate() {
        segtree.set(i, ((x, 1), (0, 0)));
    }
    for (k, l, r) in query {
        match k {
            1 => {
                let (p, x) = (l - 1, r);
                segtree.set(p, ((x, 1), (0, 0)));
            }
            2 => {
                let (l, r) = (l - 1, r - 1);
                let x = segtree.prod(l..=r).1 .1;
                println!("{}", x);
            }
            _ => unreachable!(),
        }
    }
}
