use proconio::{input, marker::Chars};

fn main() {
    input! {
        p: Chars,
        l: usize,
    }
    let yes = p.len() >= l;
    println!("{}", if yes { "Yes" } else { "No" });
}
