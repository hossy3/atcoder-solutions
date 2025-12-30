use itertools::Itertools;
use proconio::input;

fn f(wp: &[(isize, isize)]) -> usize {
    let n = wp.len();
    let mut rest: isize = wp.iter().map(|&(_, p)| p).sum();
    let v: Vec<_> = wp.iter().map(|&(w, p)| w + p).sorted().collect();
    for (i, &x) in v.iter().enumerate() {
        rest -= x;
        if rest < 0 {
            return i;
        }
    }
    n
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            wp: [(isize, isize); n],
        }
        let result = f(&wp);
        println!("{result}");
    }
}
