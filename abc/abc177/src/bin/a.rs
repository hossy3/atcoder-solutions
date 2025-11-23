use proconio::input;

fn main() {
    input! {
        d: usize,
        t: usize,
        s: usize,
    }
    let yes = d <= t * s;
    println!("{}", if yes { "Yes" } else { "No" });
}
