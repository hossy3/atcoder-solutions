use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let result = "x".repeat(s.len());
    println!("{result}");
}
