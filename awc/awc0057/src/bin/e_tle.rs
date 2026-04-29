use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        pv: [(usize, usize); n],
    }

    // 同じ位置、同じ速度は存在しないので、重複チェック不要
    let mut set = BTreeSet::new();
    let mut result = 0;
    for &(_, v) in pv.iter().sorted() {
        result += set.range(v..).count();
        set.insert(v);
    }
    println!("{result}");
}
