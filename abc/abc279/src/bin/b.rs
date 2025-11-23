use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let yes = s.contains(t.as_str());
    println!("{}", if yes { "Yes" } else { "No" });
}
