use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        mut l: Usize1,
        mut r: Usize1,
        mut s: Chars,
    }
    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
    let result = s.iter().join("");
    println!("{}", result);
}
