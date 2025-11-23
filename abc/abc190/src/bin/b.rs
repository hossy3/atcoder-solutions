use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n],
    }
    let yes = xy.iter().any(|&(x, y)| x < s && y > d);
    println!("{}", if yes { "Yes" } else { "No" });
}
