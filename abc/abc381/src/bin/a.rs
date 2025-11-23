use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let yes = (n % 2 == 1)
        && s[0..(n / 2)].iter().all(|&x| x == '1')
        && s[n / 2] == '/'
        && s[(n / 2) + 1..].iter().all(|&x| x == '2');
    println!("{}", if yes { "Yes" } else { "No" });
}
