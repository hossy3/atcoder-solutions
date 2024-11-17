use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    let yes = s == ['A', 'B', 'C'];
    println!("{}", if yes { "Yes" } else { "No" });
}
