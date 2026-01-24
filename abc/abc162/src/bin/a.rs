use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let yes = n.contains(&'7');
    println!("{}", if yes { "Yes" } else { "No" });
}
