use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    }
    let yes = d.windows(3).any(|w| w.iter().all(|(x, y)| x == y));
    println!("{}", if yes { "Yes" } else { "No" });
}
