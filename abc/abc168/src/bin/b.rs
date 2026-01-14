use proconio::input;

fn main() {
    input! {
        k: usize,
        s: String,
    }
    if s.len() <= k {
        println!("{s}");
    } else {
        println!("{}...", &s[..k]);
    }
}
