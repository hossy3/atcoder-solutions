use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    let yes = s.contains("ab") || s.contains("ba");
    println!("{}", if yes { "Yes" } else { "No" });
}
