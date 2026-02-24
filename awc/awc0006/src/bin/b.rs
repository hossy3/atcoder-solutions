use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: usize,
        dr: [(usize, usize); n],
    }
    let sum = dr
        .iter()
        .filter(|&&(d, r)| r >= k * d)
        .map(|&(_, r)| r)
        .sum::<usize>();
    let yes = sum >= t;
    println!("{}", if yes { "Yes" } else { "No" });
}
