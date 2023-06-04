use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, usize); n],
    }
    let i = sa.iter().position_min_by_key(|&x| x.1).unwrap();
    for j in 0..n {
        let result = &sa[(i + j) % n].0;
        println!("{}", result);
    }
}
