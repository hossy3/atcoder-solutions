use std::collections::HashSet;

use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

/// 遅延セグメント木 区間加算 区間和 (区間の大きさも保持する)
/// # Examples
/// ```
/// let mut segtree: LazySegtree<AddAdd> = (0..10).map(|&x| (1, x)).collect::<Vec<_>>().into();
/// ```
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

const N: usize = 40;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [usize; n],
    }

    let mut segtrees: Vec<LazySegtree<AddAdd>> =
        vec![s.iter().map(|&x| (1, x)).collect::<Vec<_>>().into()];
    let mut set = HashSet::new();
    for i in 2..=N {
        segtrees.push(vec![(1, 0); (n / i).max(1)].into());
    }

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    k: usize,
                    v: usize,
                }
                if k <= N {
                    if k > 1 {
                        set.insert(k);
                    }
                    segtrees[k - 1].apply_range(0.., v);
                } else {
                    for i in ((k - 1)..n).step_by(k) {
                        segtrees[0].apply(i, v);
                    }
                }
            }
            2 => {
                input! {
                    x: usize,
                }
                let mut result = segtrees[0].prod(..x).1;
                for &k in &set {
                    result += segtrees[k - 1].prod(..(x / k)).1;
                }
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
