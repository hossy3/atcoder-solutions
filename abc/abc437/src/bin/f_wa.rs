use proconio::{input, marker::Usize1};

use ac_library::{LazySegtree, MapMonoid, Monoid};

pub struct Abc437FM;
impl Monoid for Abc437FM {
    type S = (i32, i32);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let a0 = a.0.abs() + a.1.abs();
        let b0 = b.0.abs() + b.1.abs();
        if a0 >= b0 {
            *a
        } else {
            *b
        }
    }
}

struct Abc437F;
impl MapMonoid for Abc437F {
    type M = Abc437FM;
    type F = (i32, i32);

    fn identity_map() -> Self::F {
        (0, 0)
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (f.0 + x.0, f.1 + x.1)
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        (f.0 + g.0, f.1 + g.1)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i32, i32); n],
    }
    let mut segtree: LazySegtree<Abc437F> = xy.into();
    for _ in 0..q {
        input! {
            t: u32,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: i32,
                y: i32,
            }
            segtree.set(i, (x, y));
        } else if t == 2 {
            input! {
                l: Usize1,
                r: Usize1,
                x: i32,
                y: i32,
            }
            segtree.apply_range(l..=r, (-x, -y));
            let p = segtree.prod(l..=r);
            let result = p.0.abs() + p.1.abs();
            println!("{result}");
            segtree.apply_range(l..=r, (x, y));
        }
    }
}
