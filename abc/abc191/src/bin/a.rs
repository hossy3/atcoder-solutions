use proconio::input;

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    }
    let yes = d < v * t || v * s < d;
    println!("{}", if yes { "Yes" } else { "No" });
}
