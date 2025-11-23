use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let yes = s[..(s.len() - 1)]
        .windows(2)
        .all(|v| v[0] == "salty" || v[1] == "salty");
    println!("{}", if yes { "Yes" } else { "No" });
}
