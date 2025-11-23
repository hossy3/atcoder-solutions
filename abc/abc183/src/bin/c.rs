use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }
    let result = (1..n)
        .permutations(n - 1)
        .filter(|v| {
            v.windows(2).map(|a| t[a[0]][a[1]]).sum::<usize>() + t[0][v[0]] + t[v[n - 2]][0] == k
        })
        .count();
    println!("{result}");
}
