use proconio::input;
use rustc_hash::FxHasher;
use std::{collections::HashSet, hash::BuildHasherDefault};

fn isqrt(n: usize) -> usize {
    let m = (n as f64).sqrt() as usize;
    if (m + 1).pow(2) <= n {
        m + 1
    } else if m.pow(2) <= n {
        m
    } else {
        m - 1
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut set = HashSet::<usize, BuildHasherDefault<FxHasher>>::default();
    let n = n / 2;
    let m = isqrt(n);
    for i in 1..=m {
        set.insert(2 * i * i);
    }

    let n = n / 2;
    let m = isqrt(n);
    for i in 1..=m {
        set.insert(4 * i * i);
    }

    let result = set.len();
    println!("{result}");
}
