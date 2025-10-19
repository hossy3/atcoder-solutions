use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    let yes = s.ends_with("tea");
    println!("{}", if yes { "Yes" } else { "No" });
}
