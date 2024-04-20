use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use itertools::Itertools;
use proconio::{input, marker::Chars};

struct OpK;
impl MapMonoid for OpK {
    type M = Max<i64>;
    type F = i64;

    fn identity_map() -> Self::F {
        1
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f * x
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f * g
    }
}

struct OpX;
impl MapMonoid for OpX {
    type M = Max<i64>;
    type F = (i64, i64);

    fn identity_map() -> Self::F {
        (1, 0)
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f.0 * x + f.1
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        (f.0 * g.0, f.0 * g.1 + f.1)
    }
}

fn main() {
    input! {
        s: Chars,
    }

    let mut segtree_k: LazySegtree<OpK> = vec![1; s.len()].into();
    let mut segtree_x: LazySegtree<OpX> = vec![0; s.len()].into();

    let mut v = vec![];
    let mut stack = vec![];
    for &c in &s {
        match c {
            '(' => stack.push(v.len()),
            ')' => {
                let l = stack.pop().unwrap();
                let r = v.len();
                if l < r {
                    segtree_k.apply_range(l..r, -1);
                    segtree_x.apply_range(l..r, (-1, (l + r - 1) as i64));
                }
            }
            _ => {
                v.push(c);
            }
        }
    }

    let mut results = vec![' '; v.len()];
    for (i, &c) in v.iter().enumerate() {
        let k = segtree_k.get(i);
        let x = segtree_x.get(i);
        let c = if k == -1 {
            if c.is_lowercase() {
                (c as u8 + 'A' as u8 - 'a' as u8) as char
            } else {
                (c as u8 + 'a' as u8 - 'A' as u8) as char
            }
        } else {
            c
        };
        results[(k * i as i64 + x) as usize] = c;
    }
    let result = results.iter().join("");
    println!("{result}");
}
