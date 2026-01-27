use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s: Vec<_> = s.chars().collect();
    let yes = s[2] == s[3] && s[4] == s[5];
    println!("{}", if yes { "Yes" } else { "No" });
}
