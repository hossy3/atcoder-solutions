use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let yes = s.as_str() != "AAA" && s.as_str() != "BBB";
    println!("{}", if yes { "Yes" } else { "No" });
}
