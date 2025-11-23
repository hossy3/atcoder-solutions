use std::cmp::Reverse;

use ac_library::{Additive, Segtree};
use proconio::input;

fn order(a: &[usize]) -> Vec<usize> {
    let mut v = vec![];
    for (i, &x) in a.iter().enumerate() {
        v.push((x, Reverse(i)));
    }
    v.sort();
    let v: Vec<_> = v.iter().map(|&(_, Reverse(i))| i).collect();
    v
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let v = order(&a);
    let mut segtree = Segtree::<Additive<usize>>::new(n);
    let mut segtree0 = Segtree::<Additive<usize>>::new(n);
    let mut result = 0usize;
    for &i in &v {
        segtree.set(i, a[i]);
        segtree0.set(i, 1);
        result += segtree0.prod(..i) * a[i] - segtree.prod(..i);
    }
    println!("{result}");
}
