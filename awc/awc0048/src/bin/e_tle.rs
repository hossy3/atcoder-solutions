use itertools::Itertools;
use proconio::input;

const N: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n],
    }

    let mut result = 0;
    for v in a.iter().combinations(k) {
        let sum: usize = v.iter().copied().sum();
        if sum % m == 0 {
            result = (result + 1) % N;
        }
    }
    println!("{result}");
}
