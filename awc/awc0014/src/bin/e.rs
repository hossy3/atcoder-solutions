use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::{input, marker::Usize1};

// 区間加算: 区間の大きさも保持する

pub struct AddAddM;
impl Monoid for AddAddM {
    type S = (usize, usize); // (区間, 値)
    fn identity() -> Self::S {
        (1, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

struct AddAdd;
impl MapMonoid for AddAdd {
    type M = AddAddM;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (x.0, x.1 + f * x.0)
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
    }

    let mut segtree: LazySegtree<AddAdd> = c.iter().map(|&x| (1, x)).collect::<Vec<_>>().into();
    for _ in 0..q {
        input! {
            t: u8,
        }

        match t {
            1 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                    v: usize,
                }
                segtree.apply_range(l..=r, v);
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let result = segtree.prod(l..=r).1;
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
