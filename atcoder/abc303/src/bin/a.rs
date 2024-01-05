use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let yes = (0..n).all(|i| {
        let s = s[i];
        let t = t[i];
        s == t
            || (s == '1' && t == 'l')
            || (s == 'l' && t == '1')
            || (s == '0' && t == 'o')
            || (s == 'o' && t == '0')
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
