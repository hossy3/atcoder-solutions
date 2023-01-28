use proconio::input;

fn main() {
    input! {
        s: [String],
    }
    let count = s.iter().filter(|&s| s == "For").count();
    let yes = count * 2 > s.len();
    println!("{}", if yes { "Yes" } else { "No" });
}
