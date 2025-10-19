use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        s: [String; n],
        x: Usize1,
        y: String
    }
    let yes = s[x] == y;
    println!("{}", if yes { "Yes" } else { "No" });
}
