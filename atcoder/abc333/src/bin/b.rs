use proconio::{input, marker::Chars};

fn f(s: &[char]) -> bool {
    let x = (s[0] as u8).abs_diff(s[1] as u8);
    x == 1 || x == 4
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let yes = f(&s) == f(&t);
    println!("{}", if yes { "Yes" } else { "No" });
}
