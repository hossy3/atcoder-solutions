use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        n: Usize1,
        s: Chars,
    }
    let yes = s[n] == 'o';
    println!("{}", if yes { "Yes" } else { "No" });
}
