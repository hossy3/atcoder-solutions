use itertools::Itertools;
use proconio::input;

fn f(wp: &[(isize, isize)]) -> usize {
    let v: Vec<_> = wp.iter().map(|(w, p)| w - p).sorted().collect();
    let mut sum = 0;
    for (i, &x) in v.iter().enumerate() {
        sum += x;
        if sum > 0 {
            return i;
        }
    }
    v.len()
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
