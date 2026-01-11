use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let m = n - s.len();
    let s0 = "o".repeat(m);
    println!("{s0}{s}");
}
