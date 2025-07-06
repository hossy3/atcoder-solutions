use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars,
        a: Chars,
    }

    let yes = std::iter::zip(t, a).any(|(t, a)| t == 'o' && a == 'o');
    println!("{}", if yes { "Yes" } else { "No" });
}
