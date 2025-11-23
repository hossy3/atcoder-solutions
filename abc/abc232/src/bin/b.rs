use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let v = s
        .iter()
        .zip(t.iter())
        .map(|(&x, &y)| (y as usize + 26 - x as usize) % 26)
        .collect_vec();
    let yes = v[1..].iter().all(|&x| x == v[0]);
    println!("{}", if yes { "Yes" } else { "No" });
}
