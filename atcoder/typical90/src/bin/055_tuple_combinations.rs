use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }
    let count = a
        .iter()
        .tuple_combinations()
        .filter(|x: &(&_, &_, &_, &_, &_)| {
            (((((((x.0 * x.1) % p) * x.2) % p) * x.3) % p) * x.4) % p == q
        })
        .count();
    println!("{count}");
}
