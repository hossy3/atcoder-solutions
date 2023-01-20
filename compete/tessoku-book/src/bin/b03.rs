use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = (0..n)
        .permutations(3)
        .any(|v| a[v[0]] + a[v[1]] + a[v[2]] == 1000);
    println!("{}", if yes { "Yes" } else { "No" });
}
